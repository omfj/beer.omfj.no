use thiserror::Error;

pub const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024;

#[derive(Debug, Error)]
#[error("image must be nonempty, at most 10 MiB, and have a supported image type")]
pub struct InvalidImage;

pub struct DrinkImage {
    bytes: Vec<u8>,
    image_type: ImageType,
}

impl DrinkImage {
    /// Parses supported image bytes and derives their type from their signature.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidImage`] when the data is empty, too large, or does not
    /// have a supported JPEG, PNG, GIF, or WebP signature.
    pub fn parse(bytes: Vec<u8>) -> Result<Self, InvalidImage> {
        if !(1..=MAX_IMAGE_SIZE).contains(&bytes.len()) {
            return Err(InvalidImage);
        }

        let image_type = ImageType::detect(&bytes).ok_or(InvalidImage)?;

        Ok(Self { bytes, image_type })
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub fn content_type(&self) -> &'static str {
        self.image_type.content_type()
    }

    #[must_use]
    pub fn extension(&self) -> &'static str {
        self.image_type.extension()
    }
}

#[derive(Clone, Copy)]
pub enum ImageType {
    Jpeg,
    Png,
    Gif,
    Webp,
}

impl ImageType {
    #[must_use]
    pub fn detect(bytes: &[u8]) -> Option<Self> {
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

    #[must_use]
    pub fn content_type(self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Gif => "image/gif",
            Self::Webp => "image/webp",
        }
    }

    #[must_use]
    pub fn extension(self) -> &'static str {
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
