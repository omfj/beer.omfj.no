use thiserror::Error;

pub const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024;

#[derive(Debug, Error)]
pub enum InvalidDrink {
    #[error("image must be nonempty, at most 10 MiB, and have an image content type")]
    Image,
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

pub struct DrinkImage {
    bytes: Vec<u8>,
    content_type: String,
    extension: String,
}

impl DrinkImage {
    pub fn parse(
        bytes: Vec<u8>,
        content_type: String,
        extension: String,
    ) -> Result<Self, InvalidDrink> {
        if !(1..=MAX_IMAGE_SIZE).contains(&bytes.len()) || !content_type.starts_with("image/") {
            return Err(InvalidDrink::Image);
        }

        let valid_extension = (1..=10).contains(&extension.len())
            && extension.chars().all(|c| c.is_ascii_alphanumeric());
        let extension = if valid_extension {
            extension
        } else {
            "jpg".into()
        };

        Ok(Self {
            bytes,
            content_type,
            extension,
        })
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    pub fn extension(&self) -> &str {
        &self.extension
    }
}
