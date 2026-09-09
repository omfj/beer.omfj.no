//! S3-compatible image storage, independent of application configuration and HTTP routing.

use std::sync::Arc;

use beer_image::ImageType;
use s3::{Bucket, Region, creds::Credentials};
use thiserror::Error;

/// Connection settings supplied by the application.
pub struct StorageConfig {
    pub endpoint: String,
    pub bucket: String,
    pub access_key_id: String,
    pub secret_access_key: String,
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("image storage is not configured")]
    NotConfigured,
    #[error("image storage request failed")]
    Request(#[from] s3::error::S3Error),
    #[error("image storage returned status {0}")]
    Status(u16),
    #[error("image storage returned invalid image content")]
    InvalidImage,
}

pub struct StoredImage {
    pub bytes: Vec<u8>,
    pub content_type: String,
}

impl TryFrom<&StorageConfig> for Credentials {
    type Error = s3::creds::error::CredentialsError;

    fn try_from(config: &StorageConfig) -> Result<Self, Self::Error> {
        Self::new(
            Some(&config.access_key_id),
            Some(&config.secret_access_key),
            None,
            None,
            None,
        )
    }
}

#[derive(Clone, Default)]
pub struct ImageStorage(Option<Arc<Bucket>>);

impl ImageStorage {
    /// Creates image storage, or disabled storage when no configuration is supplied.
    ///
    /// # Errors
    /// Returns an error if credentials or the S3 client cannot be initialized.
    pub fn new(config: Option<&StorageConfig>) -> Result<Self, Box<dyn std::error::Error>> {
        let Some(config) = config else {
            return Ok(Self::default());
        };
        let credentials = Credentials::try_from(config)?;
        let mut bucket = Bucket::new(
            &config.bucket,
            Region::Custom {
                region: "auto".into(),
                endpoint: config.endpoint.clone(),
            },
            credentials,
        )?
        .with_path_style();
        bucket.set_request_timeout(Some(std::time::Duration::from_secs(30)));
        Ok(Self(Some(Arc::from(bucket))))
    }

    /// Retrieves an image; missing objects return `None`.
    ///
    /// # Errors
    /// Returns an error if storage is disabled or the S3 request fails. Also rejects unsupported image content.
    pub async fn get(&self, key: &str) -> Result<Option<StoredImage>, StorageError> {
        let bucket = self.0.as_ref().ok_or(StorageError::NotConfigured)?;
        let response = match bucket.get_object(key).await {
            Ok(response) => response,
            Err(s3::error::S3Error::HttpFailWithBody(404, _)) => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        if response.status_code() == 404 {
            return Ok(None);
        }
        check_status(response.status_code())?;
        let bytes = response.to_vec();
        let content_type = ImageType::detect(&bytes)
            .ok_or(StorageError::InvalidImage)?
            .content_type()
            .into();
        Ok(Some(StoredImage {
            bytes,
            content_type,
        }))
    }

    /// Uploads image bytes with the supplied content type.
    ///
    /// # Errors
    /// Returns an error if storage is disabled or the S3 request fails.
    pub async fn put(
        &self,
        key: &str,
        data: &[u8],
        content_type: &str,
    ) -> Result<(), StorageError> {
        let bucket = self.0.as_ref().ok_or(StorageError::NotConfigured)?;
        let response = bucket
            .put_object_with_content_type(key, data, content_type)
            .await?;
        check_status(response.status_code())
    }

    /// Deletes an image object.
    ///
    /// # Errors
    /// Returns an error if storage is disabled or the S3 request fails.
    pub async fn delete(&self, key: &str) -> Result<(), StorageError> {
        let bucket = self.0.as_ref().ok_or(StorageError::NotConfigured)?;
        let response = bucket.delete_object(key).await?;
        check_status(response.status_code())
    }
}

fn check_status(status: u16) -> Result<(), StorageError> {
    if (200..300).contains(&status) {
        Ok(())
    } else {
        Err(StorageError::Status(status))
    }
}

#[cfg(test)]
mod tests {
    use super::{ImageStorage, StorageError, check_status};

    #[tokio::test]
    async fn absent_configuration_disables_all_storage_operations() {
        let storage = ImageStorage::new(None).unwrap();
        assert!(matches!(
            storage.get("image.png").await,
            Err(StorageError::NotConfigured)
        ));
        assert!(matches!(
            storage.put("image.png", b"image", "image/png").await,
            Err(StorageError::NotConfigured)
        ));
        assert!(matches!(
            storage.delete("image.png").await,
            Err(StorageError::NotConfigured)
        ));
    }

    #[test]
    fn accepts_success_statuses_and_preserves_failure_statuses() {
        for status in [200, 201, 204, 299] {
            assert!(check_status(status).is_ok());
        }
        for status in [199, 300, 403, 404, 500] {
            assert!(matches!(
                check_status(status),
                Err(StorageError::Status(actual)) if actual == status
            ));
        }
    }
}
