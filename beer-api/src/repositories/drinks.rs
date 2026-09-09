use crate::database::Database;
use beer_domain::drinks::{CreatedDrink, DrinkSize, DrinkType, DrinkTypeSize};
use beer_domain::id::{DrinkId, EventId, ImageId, UserId};

pub struct DrinkOptionsRecord {
    pub types: Vec<DrinkType>,
    pub sizes: Vec<DrinkSize>,
    pub type_sizes: Vec<DrinkTypeSize>,
}

pub struct DeletedDrink {
    pub image_id: Option<String>,
}

#[derive(Clone)]
pub struct DrinksRepository(pub Database);

impl DrinksRepository {
    pub async fn image_event(&self, image_id: &ImageId) -> Result<Option<String>, sqlx::Error> {
        let image_id = image_id.as_str();
        sqlx::query_scalar!(
            "SELECT attendee.event_id
            FROM attendee
            JOIN event ON event.id = attendee.event_id
            WHERE attendee.image_id = ? LIMIT 1",
            image_id,
        )
        .fetch_optional(&self.0)
        .await
    }

    pub async fn options(&self) -> Result<DrinkOptionsRecord, sqlx::Error> {
        Ok(DrinkOptionsRecord {
            types: sqlx::query_as!(
                DrinkType,
                "SELECT id, name, description, abv, multiplier FROM drink_type ORDER BY name",
            )
            .fetch_all(&self.0)
            .await?,
            sizes: sqlx::query_as!(
                DrinkSize,
                "SELECT id, name, volume_ml, description FROM drink_size ORDER BY volume_ml",
            )
            .fetch_all(&self.0)
            .await?,
            type_sizes: sqlx::query_as!(
                DrinkTypeSize,
                "SELECT id, drink_type_id, drink_size_id FROM drink_type_size ORDER BY id",
            )
            .fetch_all(&self.0)
            .await?,
        })
    }

    pub async fn valid_selection(
        &self,
        kind: Option<&str>,
        size: Option<&str>,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar!(
            r#"
            SELECT
                (? IS NULL OR EXISTS(SELECT 1 FROM drink_type WHERE id = ?))
                AND (? IS NULL OR EXISTS(SELECT 1 FROM drink_size WHERE id = ?))
                AND (? IS NULL OR ? IS NULL OR EXISTS(
                    SELECT 1 FROM drink_type_size WHERE drink_type_id = ? AND drink_size_id = ?
                )) AS "valid!: bool"
            "#,
            kind,
            kind,
            size,
            size,
            kind,
            size,
            kind,
            size,
        )
        .fetch_one(&self.0)
        .await
    }

    pub async fn delete(
        &self,
        event_id: &EventId,
        drink_id: &DrinkId,
        user_id: &UserId,
    ) -> Result<Option<DeletedDrink>, sqlx::Error> {
        let user_id = user_id.as_str();
        let event_id = event_id.as_str();
        let drink_id = drink_id.as_str();
        sqlx::query_as!(
            DeletedDrink,
            r#"
            DELETE FROM attendee
            WHERE id = ? AND event_id = ? AND user_id = ?
            RETURNING image_id
            "#,
            drink_id,
            event_id,
            user_id,
        )
        .fetch_optional(&self.0)
        .await
    }

    pub async fn insert(&self, drink: &CreatedDrink) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO attendee (id, event_id, user_id, image_id, created_at, drink_type_id, drink_size_id, abv)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            drink.id.as_str(),
            drink.event_id.as_str(),
            drink.user_id.as_str(),
            drink.image_id.as_str(),
            drink.created_at.as_seconds(),
            drink.drink_type_id,
            drink.drink_size_id,
            drink.abv,
        )
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
