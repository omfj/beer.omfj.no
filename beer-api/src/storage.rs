use std::sync::Arc;

use crate::config::{Config, S3Config};

use beer_image::ImageType;
use s3::{Bucket, Region, creds::Credentials};
use thiserror::Error;

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

impl TryFrom<&S3Config> for Credentials {
    type Error = s3::creds::error::CredentialsError;

    fn try_from(config: &S3Config) -> Result<Self, Self::Error> {
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
    pub fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let Some(config) = &config.s3 else {
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
