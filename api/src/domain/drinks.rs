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
    image_type: ImageType,
}

impl DrinkImage {
    pub fn parse(bytes: Vec<u8>) -> Result<Self, InvalidDrink> {
        if !(1..=MAX_IMAGE_SIZE).contains(&bytes.len()) {
            return Err(InvalidDrink::Image);
        }

        let image_type = ImageType::detect(&bytes).ok_or(InvalidDrink::Image)?;

        Ok(Self { bytes, image_type })
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn content_type(&self) -> &str {
        self.image_type.content_type()
    }

    pub fn extension(&self) -> &str {
        self.image_type.extension()
    }
}

#[derive(Clone, Copy)]
pub(crate) enum ImageType {
    Jpeg,
    Png,
    Gif,
    Webp,
}

impl ImageType {
    pub(crate) fn detect(bytes: &[u8]) -> Option<Self> {
        if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
            Some(Self::Jpeg)
        } else if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
            Some(Self::Png)
        } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
            Some(Self::Gif)
        } else if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
            Some(Self::Webp)
        } else {
            None
        }
    }

    pub(crate) fn content_type(self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Gif => "image/gif",
            Self::Webp => "image/webp",
        }
    }

    fn extension(self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Gif => "gif",
            Self::Webp => "webp",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DrinkImage;

    #[test]
    fn derives_image_metadata_from_bytes() {
        let image = DrinkImage::parse(b"\x89PNG\r\n\x1a\ncontents".to_vec()).unwrap();

        assert_eq!(image.content_type(), "image/png");
        assert_eq!(image.extension(), "png");
    }

    #[test]
    fn rejects_active_and_unknown_image_content() {
        assert!(DrinkImage::parse(b"<svg><script>alert(1)</script></svg>".to_vec()).is_err());
    }
}
