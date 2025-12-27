use crate::{Widget, product::Product};

use anyhow::Result;

use std::future::Future;

use crate::personalisation::Personalisation;

pub trait WidgetRepository {
    /// Get all widgets personalised for a user across all products
    fn get_widgets_for_user(
        &self,
        personalisation: &Personalisation,
    ) -> impl Future<Output = Result<Vec<Widget>>> + Send;
    
    /// Update or insert a new widget.
    fn upsert(&mut self, widget: &Widget) -> impl Future<Output = Result<()>> + Send;

    /// Remove a widget entry
    fn remove(
        &mut self,
        product: Product,
        personalisation: &Personalisation,
    ) -> impl Future<Output = Result<()>> + Send;

    fn clear(&mut self) -> impl Future<Output = Result<()>> + Send;
}
