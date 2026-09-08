use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidDrink {
    #[error("alcohol percentage must be between 0 and 100")]
    Abv,
}

pub struct Abv(f64);

impl Abv {
    pub fn parse(value: f64) -> Result<Self, InvalidDrink> {
        if (0.0..=100.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(InvalidDrink::Abv)
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
