use crate::repositories::HealthRepository;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HealthError {
    #[error("database health check failed")]
    Database(#[from] sqlx::Error),
}

#[derive(Clone)]
pub struct HealthService {
    repository: HealthRepository,
}

impl HealthService {
    pub fn new(repository: HealthRepository) -> Self {
        Self { repository }
    }

    pub async fn check(&self) -> Result<(), HealthError> {
        self.repository.check().await?;
        Ok(())
    }
}
