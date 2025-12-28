use anyhow::Context;
use strum::VariantArray;

// Adding a Product requires a change, recompile, deploy, cache warmup, etc.
// The alternative would be some dynamic configuration values being read on a file change,
// however this shouldn't happen too often and most of the human effort of changing text would remain anyway,
// so I believe the type safety is worth it.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    VariantArray,
    strum::Display,
)]
pub enum Product {
    Travel,
    CarInsurance,
    HorseInsurance,
    CellularContract,
}

impl From<Product> for String {
    fn from(value: Product) -> Self {
        value.to_string()
    }
}

impl TryFrom<String> for Product {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value).with_context(|| format!("Invalid product: {value}"))
    }
}
