use crate::{UserID, Widget, product::Product};

use anyhow::Result;

use std::{collections::HashSet, future::Future};

use crate::personalisation::Personalisation;

pub trait WidgetRepository {
    /// Get all widgets personalised for a user across all products
    fn get_widgets_for_user(
        &self,
        personalisation: &Personalisation,
    ) -> impl Future<Output = Result<Vec<Widget>>> + Send;

    /// Get all users this cache wants updates for, i.e. those in cache (whether or not there is data associated with them)
    fn get_cached_users(&self) -> impl Future<Output = Result<HashSet<UserID>>> + Send;

    /// Update or insert a new widget.
    fn upsert(& self, widget: &Widget) -> impl Future<Output = Result<()>> + Send;

    /// Remove a widget entry
    fn remove(
        &mut self,
        product: Product,
        personalisation: &Personalisation,
    ) -> impl Future<Output = Result<()>> + Send;

    // Remove all entries from every level of cache
    fn clear(&mut self) -> impl Future<Output = Result<()>> + Send;
}
