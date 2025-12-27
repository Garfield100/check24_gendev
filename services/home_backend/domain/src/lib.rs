use std::future::Future;

use anyhow::Result;
use uuid::Uuid;

mod personalisation;
mod product;
mod widget;
mod widget_repository;

pub use personalisation::Personalisation;
pub use product::Product;
pub use widget::Widget;
pub use widget_repository::WidgetRepository;
pub use strum::VariantArray;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UserID(pub Uuid);

#[derive(Debug)]
pub struct HomeService<C: WidgetRepository> {
    pub widget_cache: C,
}
