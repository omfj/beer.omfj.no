use crate::database::Database;

#[derive(Clone)]
pub struct HealthRepository {
    database: Database,
}

impl HealthRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn check(&self) -> Result<(), sqlx::Error> {
        sqlx::query!("SELECT 1 AS health")
            .fetch_one(&self.database)
            .await?;
        Ok(())
    }
}
