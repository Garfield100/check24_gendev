use anyhow::Context;
use anyhow::Ok;
use domain::Product;

use anyhow::Result;
use domain::Personalisation;
use domain::UserID;
use domain::VariantArray;
use domain::Widget;
use domain::WidgetRepository;
use fred::prelude::*;
use moka::Expiry;
use moka::future::CacheBuilder;
use std::collections::HashSet;
use std::hash::Hash;
use std::time::Duration;
use tokio::task::JoinSet;
use uuid::Uuid;

use moka::future::Cache;

#[derive(Debug, Hash, PartialEq, Eq)]
struct L1Key(Personalisation, Product);
const CACHED_USERS_SET: &str = "cached_users";

#[derive(Debug, Clone)]
pub struct WidgetCache {
    l1: Cache<L1Key, Widget>,
    l2: Pool,
}

// TODO config value?
const GENERIC_CACHE_DURATION: Duration = Duration::from_secs(5);
pub(crate) struct ExpireGeneric {}

impl Expiry<L1Key, Widget> for ExpireGeneric {
    fn expire_after_create(
        &self,
        _key: &L1Key,
        value: &Widget,
        _created_at: std::time::Instant,
    ) -> Option<Duration> {
        if let Personalisation(None) = value.personalisation {
            Some(GENERIC_CACHE_DURATION)
        } else {
            None
        }
    }
}

impl WidgetCache {
    pub async fn new() -> Result<Self> {
        // TODO use `config`
        let pool_size = 8;
        let url = "valkey://127.0.0.1:6379";

        let config = Config::from_url(url)
            .with_context(|| format!("Failed to create valkey config from url {url}"))?;

        let pool = Builder::from_config(config)
            .with_connection_config(|config| {
                config.connection_timeout = Duration::from_secs(10);
            })
            // linear backoff starting at 100ms ending at 2s
            .set_policy(ReconnectPolicy::new_linear(0, 2000, 100))
            .build_pool(pool_size)
            .with_context(|| "Failed to create valkey pool")?;

        pool.init()
            .await
            .with_context(|| "Failed to connect to valkey")?;

        let l1 = CacheBuilder::<_, Widget, _>::new(10_000_000) // max size approx. in bytes
            .name("Home_Service_L1_Cache")
            .expire_after(ExpireGeneric {})
            .weigher(|_, v| v.data.len() as u32) // weigh entries by their byte length so that (barring key size) max size is roughly in bytes
            .build();

        Ok(Self { l1, l2: pool })
    }

    async fn register_user_as_cached(&self, UserID(user_uuid): &UserID) -> Result<()> {
        self.l2
            .sadd::<(), &str, &[u8]>(CACHED_USERS_SET, user_uuid.as_bytes())
            .await.with_context(|| format!("Failed to register user {user_uuid} to Valkey hashset of cached users using SADD"))
    }
}

impl WidgetRepository for WidgetCache {
    // TODO break out into smaller functions
    #[tracing::instrument]
    async fn get_widgets_for_user(
        &self,
        personalisation: &Personalisation,
    ) -> Result<Vec<Widget>, anyhow::Error> {
        if let Personalisation(Some(user_id)) = personalisation {
            self.register_user_as_cached(user_id).await?;
        }

        let (l1_widgets, l1_missing) = self.l1_get_all(personalisation.clone()).await;
        let mut widgets = l1_widgets;
        let mut l2_missing: Vec<Product> = Vec::new();

        // if any product entries are still missing, we try to grab the them from Valkey
        if !l1_missing.is_empty() {
            // cache miss, get from Valkey
            let personalisation = personalisation.clone();
            let l1_missing_str: Vec<String> =
                l1_missing.iter().cloned().map(String::from).collect();

            let missing_res: Vec<Widget> = self
                .l2
                // type info: hmget::<Vec<Option<Widget-string>>, UserID, Vec<Product-string>> (oddly it's Value type then Key type)
                // Option<String> because Valkey returns nil values for the fields it doesn't have
                .hmget::<Vec<Option<String>>, String, Vec<String>>(
                    String::from(&personalisation),
                    l1_missing_str,
                )
                .await
                .context("Error while HMGETing widgets for user")?
                .into_iter()
                .zip(l1_missing) // zip the products we queried with back in
                .filter_map(|(widget_data, product)| {
                    if let Some(widget_data) = widget_data {
                        Some((widget_data, product))
                    } else {
                        l2_missing.push(product);
                        None
                    }
                })
                .map(|(widget_data, product)| {
                    (
                        // here we clone a value and an arc (moka Cache) so that they can be moved into the following async closure
                        personalisation.clone(),
                        self.l1.clone(),
                        product,
                        widget_data,
                    )
                })
                .map(async move |(personalisation, l1, product, widget_data)| {
                    let widget = Widget {
                        data: widget_data,
                        personalisation: personalisation.clone(),
                        product,
                    };

                    // put stuff we found back up into higher cache
                    l1.insert(L1Key(personalisation, product), widget.clone())
                        .await;

                    Ok(widget)
                })
                .collect::<JoinSet<_>>()
                .join_all()
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;

            widgets.extend(missing_res);
        };

        // if at this point we are still missing some product widgets, we have to fall back on the generic one
        for product in l2_missing {
            // TODO send in request to product backend?
            let generic_widget = Widget {
                product,
                data: load_widget_generic(product).to_string(),
                personalisation: Personalisation(None),
            };
            widgets.push(generic_widget.clone());
            // put these generic ones into L1 as well
            // our ExpireGeneric policy evicts these after a few seconds
            self.l1
                .insert(L1Key(Personalisation(None), product), generic_widget)
                .await;
        }

        Ok(widgets)
    }

    #[tracing::instrument]
    async fn upsert(&mut self, widget: &Widget) -> Result<()> {
        let user_id_string = String::from(&widget.personalisation);

        let l1_fut = self.l1.insert(
            L1Key(widget.personalisation.clone(), widget.product),
            widget.clone(),
        );

        let user_fut = async {
            if let Personalisation(Some(user_id)) = &widget.personalisation {
                self.register_user_as_cached(user_id).await
            } else {
                Ok(())
            }
        };

        let l2_fut = self
            .l2
            .hset::<(), _, _>(user_id_string, (String::from(widget.product), &widget.data));

        let (_, user_res, l2_res) = tokio::join!(l1_fut, user_fut, l2_fut);

        user_res?;
        l2_res?;

        Ok(())
    }

    #[tracing::instrument]
    async fn remove(&mut self, product: Product, personalisation: &Personalisation) -> Result<()> {
        let key = L1Key(personalisation.clone(), product); // we love lifetime analysis
        let l1_fut = self.l1.invalidate(&key);

        let l2_fut = self
            .l2
            .hdel::<(), _, _>(String::from(personalisation), String::from(product));

        let (_, l2_res) = tokio::join!(l1_fut, l2_fut);

        l2_res?;

        Ok(())
    }

    #[tracing::instrument]
    async fn clear(&mut self) -> Result<()> {
        self.l1.invalidate_all();

        let _: bool = self.l2.flushall(true).await?;

        Ok(())
    }

    #[tracing::instrument]
    async fn get_cached_users(&self) -> Result<HashSet<domain::UserID>> {
        Ok(self
            .l2
            .smembers::<Vec<[u8; 16]>, _>(CACHED_USERS_SET)
            .await?
            .into_iter()
            .map(Uuid::from_bytes)
            .map(UserID)
            .collect())
    }
}

impl WidgetCache {
    /// Gets l1 cache contents for as many products as it contains and returns them,
    /// along with a list of products for which no widget was found.
    ///
    /// This is so we can fill in the missing ones with generic ones later.
    #[tracing::instrument]
    async fn l1_get_all(&self, personalisation: Personalisation) -> (Vec<Widget>, Vec<Product>) {
        let mut cached_widgets = Vec::new();
        let mut missing_products = Vec::new();
        for product in Product::VARIANTS {
            let key = L1Key(personalisation.clone(), *product);
            // this is currently not in parallel as we have very few products. As this increases it might be worth it to use something like an async stream
            match self.l1.get(&key).await {
                Some(widget) => cached_widgets.push(widget),
                None => missing_products.push(*product),
            }
        }

        (cached_widgets, missing_products)
    }
}

/// Just loads in some example widgets as generic ones for each product.
/// This is compiled-in here since I am simulating the product backends anyway,
/// but in a prod system this would come from those backends.
const fn load_widget_generic(product: Product) -> &'static str {
    match product {
        Product::Travel => include_str!("../assets/generic_travel.json"),
        Product::CarInsurance => include_str!("../assets/generic_car_insurance.json"),
        Product::HorseInsurance => include_str!("../assets/generic_horse_insurance.json"),
        Product::CellularContract => include_str!("../assets/generic_cellular.json"),
    }
}
