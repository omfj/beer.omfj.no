use crate::{
    database::Database,
    services::drinks::{CreatedDrink, DrinkOptions, DrinkSize, DrinkType, DrinkTypeSize},
};

pub struct DeletedDrink {
    pub image_id: Option<String>,
}

#[derive(Clone)]
pub struct DrinksRepository(pub Database);

impl DrinksRepository {
    pub async fn options(&self) -> Result<DrinkOptions, sqlx::Error> {
        Ok(DrinkOptions {
            drink_types: sqlx::query_as!(
                DrinkType,
                "SELECT id, name, description, abv, multiplier FROM drink_type ORDER BY name",
            )
            .fetch_all(&self.0)
            .await?,
            drink_sizes: sqlx::query_as!(
                DrinkSize,
                "SELECT id, name, volume_ml, description FROM drink_size ORDER BY volume_ml",
            )
            .fetch_all(&self.0)
            .await?,
            drink_type_sizes: sqlx::query_as!(
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
        event_id: &str,
        drink_id: &str,
        user_id: &str,
    ) -> Result<Option<DeletedDrink>, sqlx::Error> {
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
            drink.id,
            drink.event_id,
            drink.user_id,
            drink.image_id,
            drink.created_at,
            drink.drink_type_id,
            drink.drink_size_id,
            drink.abv,
        )
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
