use crate::database::Database;

#[derive(Debug)]
pub struct LeaderboardRecord {
    pub user_id: String,
    pub username: String,
    pub volume_ml: Option<i64>,
    pub abv: Option<f64>,
    pub fallback_abv: Option<i64>,
    pub multiplier: f64,
}

#[derive(Clone)]
pub struct LeaderboardRepository {
    database: Database,
}

impl LeaderboardRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn records(&self, year: i64) -> Result<Vec<LeaderboardRecord>, sqlx::Error> {
        sqlx::query_as!(
            LeaderboardRecord,
            r#"
            SELECT
                attendee.user_id,
                user.username,
                drink_size.volume_ml,
                attendee.abv,
                drink_type.abv AS fallback_abv,
                COALESCE(drink_type.multiplier, 1.0) AS multiplier
            FROM attendee
            INNER JOIN event ON attendee.event_id = event.id
            INNER JOIN user ON attendee.user_id = user.id
            LEFT JOIN drink_type ON attendee.drink_type_id = drink_type.id
            LEFT JOIN drink_size ON attendee.drink_size_id = drink_size.id
            WHERE event.password IS NULL
              AND CAST(strftime('%Y', attendee.created_at, 'unixepoch') AS INTEGER) = ?
            "#,
            year,
        )
        .fetch_all(&self.database)
        .await
    }

    pub async fn available_years(&self) -> Result<Vec<i64>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT DISTINCT CAST(strftime('%Y', attendee.created_at, 'unixepoch') AS INTEGER) AS "year!: i64"
            FROM attendee
            INNER JOIN event ON attendee.event_id = event.id
            WHERE event.password IS NULL
            ORDER BY 1 DESC
            "#,
        )
        .fetch_all(&self.database)
        .await
        ?;

        Ok(records.into_iter().map(|record| record.year).collect())
    }
}
