use crate::time::UnixSeconds;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidDrink {
    #[error("alcohol percentage must be between 0 and 100")]
    Abv,
}

pub struct Abv(f64);

impl Abv {
    /// Parses an alcohol percentage.
    ///
    /// # Errors
    /// Returns an error for nonfinite values or percentages outside 0–100.
    pub fn parse(value: f64) -> Result<Self, InvalidDrink> {
        if (0.0..=100.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(InvalidDrink::Abv)
        }
    }

    #[must_use]
    pub fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Serialize)]
pub struct DrinkType {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub abv: Option<i64>,
    pub multiplier: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DrinkSize {
    pub id: String,
    pub name: String,
    #[serde(rename = "volumeML")]
    pub volume_ml: i64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DrinkTypeSize {
    pub id: String,
    pub drink_type_id: String,
    pub drink_size_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedDrink {
    pub id: String,
    pub event_id: String,
    pub user_id: String,
    pub image_id: String,
    pub created_at: UnixSeconds,
    pub drink_type_id: Option<String>,
    pub drink_size_id: Option<String>,
    pub abv: Option<f64>,
}
