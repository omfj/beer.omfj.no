use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvalidProfileValue {
    #[error("weight must be light, medium, or heavy")]
    Weight,
    #[error("gender must be male, female, or other")]
    Gender,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Weight {
    Light,
    Medium,
    Heavy,
}

impl Weight {
    pub fn parse(value: &str) -> Result<Self, InvalidProfileValue> {
        match value {
            "light" => Ok(Self::Light),
            "medium" => Ok(Self::Medium),
            "heavy" => Ok(Self::Heavy),
            _ => Err(InvalidProfileValue::Weight),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Medium => "medium",
            Self::Heavy => "heavy",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl Gender {
    pub fn parse(value: &str) -> Result<Self, InvalidProfileValue> {
        match value {
            "male" => Ok(Self::Male),
            "female" => Ok(Self::Female),
            "other" => Ok(Self::Other),
            _ => Err(InvalidProfileValue::Gender),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Male => "male",
            Self::Female => "female",
            Self::Other => "other",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Gender, Weight};

    #[test]
    fn parses_weight() {
        assert_eq!(Weight::parse("medium").unwrap(), Weight::Medium);
        assert!(Weight::parse("unknown").is_err());
    }

    #[test]
    fn parses_gender() {
        assert_eq!(Gender::parse("other").unwrap(), Gender::Other);
        assert!(Gender::parse("unknown").is_err());
    }
}
