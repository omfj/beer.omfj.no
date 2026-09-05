use sqlx::FromRow;

use crate::database::Database;

#[derive(Debug, FromRow)]
pub struct LoginRecord {
    pub id: String,
    pub password_hash: String,
}

#[derive(Debug, FromRow)]
pub struct SessionRecord {
    pub session_id: String,
    pub user_id: String,
    pub expires_at: i64,
    pub username: String,
    pub has_agreed_to_terms: bool,
    pub weight: Option<String>,
    pub gender: Option<String>,
    pub created_at: Option<i64>,
}

#[derive(Clone)]
pub struct AuthRepository {
    database: Database,
}

impl AuthRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn user_for_login(&self, username: &str) -> Result<Option<LoginRecord>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT user.id, user_password.password_hash
            FROM user
            INNER JOIN user_password ON user_password.user_id = user.id
            WHERE user.username = ?
            "#,
        )
        .bind(username)
        .fetch_optional(&self.database)
        .await
    }

    pub async fn create_session(
        &self,
        id: &str,
        user_id: &str,
        expires_at: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO session (id, user_id, expires_at) VALUES (?, ?, ?)")
            .bind(id)
            .bind(user_id)
            .bind(expires_at)
            .execute(&self.database)
            .await?;
        Ok(())
    }

    pub async fn session(&self, id: &str) -> Result<Option<SessionRecord>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT
                session.id AS session_id,
                session.user_id,
                session.expires_at,
                user.username,
                user.has_agreed_to_terms,
                user.weight,
                user.gender,
                user.created_at
            FROM session
            INNER JOIN user ON user.id = session.user_id
            WHERE session.id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.database)
        .await
    }

    pub async fn renew_session(&self, id: &str, expires_at: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE session SET expires_at = ? WHERE id = ?")
            .bind(expires_at)
            .bind(id)
            .execute(&self.database)
            .await?;
        Ok(())
    }

    pub async fn delete_session(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM session WHERE id = ?")
            .bind(id)
            .execute(&self.database)
            .await?;
        Ok(())
    }
}
