use crate::personalisation::Personalisation;

use crate::product::Product;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Widget {
    /// Enum of existing Products
    pub product: Product,

    /// The actual JSON content of the widget meant for SDUI
    pub data: String,

    /// Whether this widget is personalised to a specific user or a generic fallback
    pub personalisation: Personalisation,
}
