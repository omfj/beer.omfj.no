use crate::database::Database;
use crate::domain::time::UnixSeconds;

#[derive(Debug)]
pub struct EventSummaryRecord {
    pub id: String,
    pub name: String,
    pub total_attendees: i64,
    pub distinct_users: i64,
}
#[derive(Debug)]
pub struct EventRecord {
    pub id: String,
    pub name: String,
    pub color: String,
    pub created_at: i64,
    pub created_by: Option<String>,
    pub password: Option<String>,
}
#[derive(Debug)]
pub struct AttendeeRecord {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub created_at: i64,
    pub image_id: Option<String>,
    pub abv: Option<f64>,
    pub drink_type_id: Option<String>,
    pub drink_type_name: Option<String>,
    pub drink_type_description: Option<String>,
    pub drink_type_abv: Option<i64>,
    pub drink_type_multiplier: Option<f64>,
    pub drink_size_id: Option<String>,
    pub drink_size_name: Option<String>,
    pub drink_size_volume_ml: Option<i64>,
    pub drink_size_description: Option<String>,
    pub user_weight: Option<String>,
    pub user_gender: Option<String>,
}
#[derive(Debug)]
pub struct EventUserRecord {
    pub id: String,
    pub username: String,
    pub weight: Option<String>,
    pub gender: Option<String>,
}

#[derive(Clone)]
pub struct EventsRepository {
    database: Database,
}

impl EventsRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn for_user(&self, user_id: &str) -> Result<Vec<EventSummaryRecord>, sqlx::Error> {
        sqlx::query_as!(EventSummaryRecord, r#"
            SELECT event.id, event.name,
                   COUNT(attendee.id) AS "total_attendees!: i64",
                   COUNT(DISTINCT attendee.user_id) AS "distinct_users!: i64"
            FROM event LEFT JOIN attendee ON attendee.event_id = event.id
            WHERE event.created_by = ?
               OR EXISTS (SELECT 1 FROM event_access WHERE event_access.event_id = event.id AND event_access.user_id = ?)
               OR EXISTS (SELECT 1 FROM attendee own WHERE own.event_id = event.id AND own.user_id = ?)
            GROUP BY event.id, event.name, event.created_at ORDER BY event.created_at DESC
        "#, user_id, user_id, user_id).fetch_all(&self.database).await
    }

    pub async fn event(&self, id: &str) -> Result<Option<EventRecord>, sqlx::Error> {
        sqlx::query_as!(
            EventRecord,
            "SELECT id, name, color, created_at, created_by, password FROM event WHERE id = ?",
            id,
        )
        .fetch_optional(&self.database)
        .await
    }

    pub async fn create(
        &self,
        id: &str,
        name: &str,
        color: &str,
        created_at: UnixSeconds,
        created_by: &str,
        password: Option<&str>,
    ) -> Result<EventRecord, sqlx::Error> {
        sqlx::query_as!(
            EventRecord,
            r#"
            INSERT INTO event (id, name, color, created_at, created_by, password)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING id, name, color, created_at, created_by, password
            "#,
            id,
            name,
            color,
            created_at.as_seconds(),
            created_by,
            password,
        )
        .fetch_one(&self.database)
        .await
    }

    pub async fn has_access(&self, event_id: &str, user_id: &str) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM event_access WHERE event_id = ? AND user_id = ?) AS \"has_access!: bool\"",
            event_id,
            user_id,
        )
        .fetch_one(&self.database)
        .await
    }

    pub async fn grant_access(
        &self,
        event_id: &str,
        user_id: &str,
        granted_at: UnixSeconds,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO event_access (event_id, user_id, granted_at)
            VALUES (?, ?, ?)
            ON CONFLICT (event_id, user_id) DO NOTHING
            "#,
            event_id,
            user_id,
            granted_at.as_seconds(),
        )
        .execute(&self.database)
        .await?;
        Ok(())
    }

    pub async fn attendees(&self, event_id: &str) -> Result<Vec<AttendeeRecord>, sqlx::Error> {
        sqlx::query_as!(AttendeeRecord, r#"
            SELECT attendee.id, attendee.user_id, user.username, attendee.created_at,
                   CASE WHEN user.has_agreed_to_terms THEN attendee.image_id ELSE NULL END AS "image_id?",
                   COALESCE(attendee.abv, drink_type.abv) AS "abv?: f64",
                   drink_type.id AS "drink_type_id?", drink_type.name AS "drink_type_name?",
                   drink_type.description AS "drink_type_description?", drink_type.abv AS "drink_type_abv?",
                   drink_type.multiplier AS "drink_type_multiplier?", drink_size.id AS "drink_size_id?",
                   drink_size.name AS "drink_size_name?", drink_size.volume_ml AS "drink_size_volume_ml?",
                   drink_size.description AS "drink_size_description?",
                   user.weight AS "user_weight?", user.gender AS "user_gender?"
            FROM attendee INNER JOIN user ON user.id = attendee.user_id
            LEFT JOIN drink_type ON drink_type.id = attendee.drink_type_id
            LEFT JOIN drink_size ON drink_size.id = attendee.drink_size_id
            WHERE attendee.event_id = ? ORDER BY attendee.created_at DESC
        "#, event_id).fetch_all(&self.database).await
    }

    pub async fn access_users(&self, event_id: &str) -> Result<Vec<EventUserRecord>, sqlx::Error> {
        sqlx::query_as!(
            EventUserRecord,
            r#"
            SELECT user.id, user.username, user.weight, user.gender FROM user
            WHERE user.id IN (
                SELECT user_id FROM event_access WHERE event_id = ?
                UNION SELECT created_by FROM event WHERE id = ? AND created_by IS NOT NULL
            ) ORDER BY user.username, user.id
        "#,
            event_id,
            event_id,
        )
        .fetch_all(&self.database)
        .await
    }
}
