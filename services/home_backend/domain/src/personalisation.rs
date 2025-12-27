use std::ops::Deref;

use uuid::Uuid;

use anyhow::{Context, Result};

use super::UserID;

// To clarify that None means it is not personalised but generic
// Newtype so I can turn it into a string and back in one place
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Personalisation(pub Option<UserID>);

const GENERIC_USER_ID: &str = "GENERIC";

impl From<&Personalisation> for String {
    fn from(value: &Personalisation) -> Self {
        value
            .as_ref()
            .map(|u| u.0.to_string())
            .unwrap_or(GENERIC_USER_ID.to_string())
    }
}

impl TryFrom<&String> for Personalisation {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            GENERIC_USER_ID => Ok(Personalisation(None)),
            uuid_string => Uuid::try_parse(uuid_string)
                .map(|uuid| Personalisation(Some(UserID(uuid))))
                .map_err(anyhow::Error::from)
                .with_context(|| format!("{} is an invalid UUID", uuid_string)),
        }
    }
}

// More easily use the Option functions within without always using .0
impl Deref for Personalisation {
    type Target = Option<UserID>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
