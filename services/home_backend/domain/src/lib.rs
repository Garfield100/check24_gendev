use std::future::Future;

use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserID(pub Uuid);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Product {
    Travel,
    CarInsurance,
    CellularContract,
}

// slightly clearer than an Option<UserID>
#[derive(Debug)]
pub enum Personalisation {
    User(UserID),
    Generic,
}

#[derive(Debug)]
pub struct Widget {
    product: Product,

    /// The actual JSON content of the widget meant for SDUI
    data: Value,

    /// Whether this widget is personalised to a specific user or a generic fallback
    personalisation: Personalisation,
}

pub trait WidgetRepository {
    /// Get all widgets personalised for a user across all products
    fn get_widgets_for_user(
        &self,
        user_id: UserID,
    ) -> impl Future<Output = Result<Vec<Widget>, anyhow::Error>> + Send;

    /// Update or insert a new widget.
    /// Returns the old widget, if it exists
    fn upsert(
        &mut self,
        widget: Widget,
    ) -> impl Future<Output = Result<Option<Widget>, anyhow::Error>> + Send;

    /// Remove a widget entry
    /// Returns the old widget if it exists
    fn remove(
        &mut self,
        product: Product,
        personalisation: Personalisation,
    ) -> impl Future<Output = Result<Widget, anyhow::Error>> + Send;
}
