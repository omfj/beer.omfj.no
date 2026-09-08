use crate::domain::time::UnixSeconds;
use crate::{
    database::Database,
    domain::drinks::{Abv, DrinkImage},
    repositories::{EventsRepository, drinks::DrinksRepository},
    storage::{ImageStorage, StorageError},
    utils::time::now,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DrinksError {
    #[error("event not found")]
    NotFound,
    #[error("drink not found")]
    DrinkNotFound,
    #[error("image not found")]
    ImageNotFound,
    #[error("event access denied")]
    Forbidden,
    #[error("invalid drink type or size")]
    InvalidSelection,
    #[error("drink database operation failed")]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Storage(#[from] StorageError),
}

#[derive(Serialize)]
pub struct DrinkType {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub abv: Option<i64>,
    pub multiplier: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DrinkSize {
    pub id: String,
    pub name: String,
    #[serde(rename = "volumeML")]
    pub volume_ml: i64,
    pub description: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DrinkTypeSize {
    pub id: String,
    pub drink_type_id: String,
    pub drink_size_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_field_names)]
pub struct DrinkOptions {
    pub drink_types: Vec<DrinkType>,
    pub drink_sizes: Vec<DrinkSize>,
    pub drink_type_sizes: Vec<DrinkTypeSize>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedDrink {
    pub id: String,
    pub event_id: String,
    pub user_id: String,
    pub image_id: String,
    pub created_at: UnixSeconds,
    pub drink_type_id: Option<String>,
    pub drink_size_id: Option<String>,
    pub abv: Option<f64>,
}

pub struct NewDrink {
    pub image: DrinkImage,
    pub drink_type_id: Option<String>,
    pub drink_size_id: Option<String>,
    pub abv: Option<Abv>,
}

#[derive(Clone)]
pub struct DrinksService {
    repository: DrinksRepository,
    events: EventsRepository,
    images: ImageStorage,
}
impl DrinksService {
    pub fn new(database: Database, images: ImageStorage) -> Self {
        Self {
            repository: DrinksRepository(database.clone()),
            events: EventsRepository::new(database),
            images,
        }
    }
    pub async fn image(
        &self,
        image_id: &str,
        user_id: &str,
    ) -> Result<crate::storage::StoredImage, DrinksError> {
        let event_id = self
            .repository
            .image_event(image_id)
            .await?
            .ok_or(DrinksError::ImageNotFound)?;
        self.authorize(&event_id, user_id).await?;
        self.images
            .get(image_id)
            .await?
            .ok_or(DrinksError::ImageNotFound)
    }

    pub async fn options(&self) -> Result<DrinkOptions, DrinksError> {
        Ok(self.repository.options().await?)
    }

    pub async fn authorize(&self, event_id: &str, user_id: &str) -> Result<(), DrinksError> {
        let event = self
            .events
            .event(event_id)
            .await?
            .ok_or(DrinksError::NotFound)?;
        if event.password.is_some()
            && event.created_by.as_deref() != Some(user_id)
            && !self.events.has_access(event_id, user_id).await?
        {
            return Err(DrinksError::Forbidden);
        }
        Ok(())
    }

    pub async fn delete(
        &self,
        event_id: &str,
        drink_id: &str,
        user_id: &str,
    ) -> Result<(), DrinksError> {
        let drink = self
            .repository
            .delete(event_id, drink_id, user_id)
            .await?
            .ok_or(DrinksError::DrinkNotFound)?;

        if let Some(image_id) = drink.image_id
            && let Err(error) = self.images.delete(&image_id).await
        {
            tracing::error!(%image_id, ?error, "failed to delete image for deleted drink");
        }

        Ok(())
    }

    pub async fn create(
        &self,
        event_id: &str,
        user_id: &str,
        input: NewDrink,
    ) -> Result<CreatedDrink, DrinksError> {
        self.authorize(event_id, user_id).await?;

        let is_valid_selection = self
            .repository
            .valid_selection(
                input.drink_type_id.as_deref(),
                input.drink_size_id.as_deref(),
            )
            .await?;
        if !is_valid_selection {
            return Err(DrinksError::InvalidSelection);
        }

        let id = format!("{:032x}", rand::random::<u128>());
        let drink = CreatedDrink {
            image_id: format!("{id}.{}", input.image.extension()),
            id,
            event_id: event_id.into(),
            user_id: user_id.into(),
            created_at: now(),
            drink_type_id: input.drink_type_id,
            drink_size_id: input.drink_size_id,
            abv: input.abv.map(|abv| abv.value()),
        };
        self.images
            .put(
                &drink.image_id,
                input.image.bytes(),
                input.image.content_type(),
            )
            .await?;
        if let Err(error) = self.repository.insert(&drink).await {
            if let Err(cleanup_error) = self.images.delete(&drink.image_id).await {
                tracing::error!(image_id = %drink.image_id, error = ?cleanup_error, "failed to clean up image after drink insert failure");
            }
            return Err(error.into());
        }
        Ok(drink)
    }
}
