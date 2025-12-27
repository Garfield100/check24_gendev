use anyhow::Context;
use anyhow::Ok;
use domain::Product;

use anyhow::Result;
use domain::Personalisation;
use domain::VariantArray;
use domain::Widget;
use domain::WidgetRepository;
use fred::prelude::*;
use parking_lot::RwLock;
use quick_cache::sync::Cache;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;

// TODO: at what point (N of Products) is a simple list faster than a hashmap?
type NestedMap<K1, K2, V> = Cache<K1, HashMap<K2, V>>;

#[derive(Debug, Clone)]
pub struct WidgetCache {
    l1: Arc<NestedMap<Personalisation, Product, Widget>>,
    l2: Pool, // Pool type already an Arc
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

        let l1 = Cache::new(10_000);

        Ok(Self {
            l1: Arc::new(l1),
            l2: pool,
        })
    }
}

trait HashKey: Eq + Hash + Clone {}

impl<T: Eq + Hash + Clone> HashKey for T {}

async fn l1_upsert<K1, K2, V>(cache: Arc<NestedMap<K1, K2, V>>, k1: &K1, k2: K2, value: V)
where
    K1: HashKey,
    K2: HashKey,
    V: Clone,
{
    match cache.get_value_or_guard_async(k1).await {
        Result::Ok(mut res) => {
            res.insert(k2, value);
        }
        // if the value isn't already present, quick_cache gives us a guard in the Err variant so we can initialise our value.
        Err(guard) => {
            // TODO see if a faster hasher is worth it here via HashMap::with_hasher or just a Vec or Product-sized array
            let mut map = HashMap::new();
            map.insert(k2, value);
            // let _ because we don't care about the old value, which quick_cache returns as a Result::Err variant
            let _ = guard.insert(map);
        }
    };
}

impl WidgetRepository for WidgetCache {
    async fn get_widgets_for_user(
        &self,
        personalisation: &Personalisation,
    ) -> Result<Vec<Widget>, anyhow::Error> {
        let mut widgets: Vec<Widget> = if let Some(res) = self.l1.get(personalisation) {
            // cache hit
            Ok(res.values().cloned().collect())
        } else {
            let personalisation = personalisation.clone();
            // cache miss, get from Valkey
            self.l2
                // type info: hgetall::<Vec<Product, Widget>, UserID> (oddly it's Value type then Key type)
                .hgetall::<Vec<(String, String)>, String>(String::from(&personalisation))
                .await?
                .into_iter()
                // here we clone a value and an arc so that they can be moved into the following async closure
                .map(|(product_string, widget_data)| {
                    (
                        personalisation.clone(),
                        self.l1.clone(),
                        product_string,
                        widget_data,
                    )
                })
                .map(async |(personalisation, l1, product_string, widget_data)| {
                    let product = Product::try_from(product_string)?;
                    let widget = Widget {
                        data: widget_data,
                        personalisation: personalisation.clone(),
                        product,
                    };

                    l1_upsert(l1, &personalisation, product, widget.clone()).await;

                    Ok(widget)
                })
                .collect::<JoinSet<_>>()
                .join_all()
                .await
                .into_iter()
                .collect()
        }?;

        // if we don't have data for all products yet, we have to fall back to the generic ones
        if widgets.len() < Product::VARIANTS.len() {
            let mut all_widgets = Vec::with_capacity(Product::VARIANTS.len());
            let mut missing_widgets = Vec::new();
            for product in Product::VARIANTS {
                let widget_or_generic = widgets
                    .iter()
                    .find(|w| w.product == *product)
                    .cloned()
                    .unwrap_or_else(|| {
                        let w = Widget {
                            product: *product,
                            data: load_widget_generic(product).to_string(),
                            personalisation: Personalisation(None),
                        };
                        missing_widgets.push(w.clone());
                        w
                    });
                all_widgets.push(widget_or_generic);
            }
            widgets = all_widgets;

            for w in missing_widgets.into_iter() {
                // TODO request these from the respective backend

                // insert the generic widget into the l1 cache for now
                l1_upsert(self.l1.clone(), personalisation, w.product, w).await;
            }
        }

        Ok(widgets)
    }

    async fn upsert(&mut self, widget: &Widget) -> Result<()> {
        let user_id_string = String::from(&widget.personalisation);

        l1_upsert(
            self.l1.clone(),
            &widget.personalisation,
            widget.product,
            widget.clone(),
        )
        .await;

        self.l2
            .hset::<(), _, _>(user_id_string, (String::from(widget.product), &widget.data))
            .await
            .with_context(|| format!("Failed to upsert {widget:?}"))
    }

    async fn remove(&mut self, product: Product, personalisation: &Personalisation) -> Result<()> {
        self.l1.get(personalisation).map(|mut m| m.remove(&product));

        self.l2
            .hdel::<(), _, _>(String::from(personalisation), String::from(product))
            .await
            .with_context(|| {
                format!(
                    "Error while removing entry for product {product:?} and user {}",
                    String::from(personalisation)
                )
            })
    }

    async fn clear(&mut self) -> Result<()> {
        self.l1.clear();

        let _: bool = self.l2.flushall(true).await?;

        Ok(())
    }
}

const fn load_widget_generic(product: &Product) -> &'static str {
    match product {
        Product::Travel => include_str!("../assets/generic_travel.json"),
        Product::CarInsurance => include_str!("../assets/generic_car_insurance.json"),
        Product::HorseInsurance => include_str!("../assets/generic_horse_insurance.json"),
        Product::CellularContract => include_str!("../assets/generic_cellular.json"),
    }
}
