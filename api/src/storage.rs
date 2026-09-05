use std::sync::Arc;

use crate::config::Config;

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
}

#[derive(Clone, Default)]
pub struct ImageStorage(Option<Arc<Bucket>>);

impl ImageStorage {
    pub fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let (Some(endpoint), Some(bucket), Some(access_key_id), Some(secret_access_key)) = (
            &config.s3_endpoint,
            &config.s3_bucket,
            &config.s3_access_key_id,
            &config.s3_secret_access_key,
        ) else {
            return Ok(Self::default());
        };
        let credentials = Credentials::new(
            Some(access_key_id),
            Some(secret_access_key),
            None,
            None,
            None,
        )?;
        let mut bucket = Bucket::new(
            bucket,
            Region::Custom {
                region: "auto".into(),
                endpoint: endpoint.clone(),
            },
            credentials,
        )?
        .with_path_style();
        bucket.set_request_timeout(Some(std::time::Duration::from_secs(30)));
        Ok(Self(Some(Arc::from(bucket))))
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
