use std::{collections::HashMap, sync::Arc};

use domain::{Personalisation, Product, UserID, Widget, WidgetRepository};
use fred::clients::Pool;

#[derive(Debug, Clone)]
pub struct WidgetCache {
    l1: Arc<HashMap<(Product, Personalisation), Widget>>,
    l2: Pool, // Pool type already an Arc
}

impl WidgetRepository for WidgetCache {
    fn get_widgets_for_user(
        &self,
        user_id: UserID,
    ) -> impl Future<Output = Result<Vec<Widget>, anyhow::Error>> + Send {
        todo!()
    }

    fn upsert(
        &mut self,
        widget: Widget,
    ) -> impl Future<Output = Result<Option<Widget>, anyhow::Error>> + Send {
        todo!()
    }

    fn remove(
        &mut self,
        product: Product,
        personalisation: domain::Personalisation,
    ) -> impl Future<Output = Result<Widget, anyhow::Error>> + Send {
        todo!()
    }
}
