use crate::utils::color::generate_soft_color;
use crate::{repositories::EventsRepository, utils::password};
use beer_domain::drinks::{DrinkSize, DrinkType};
use beer_domain::time::UnixSeconds;
use rand::RngExt;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventsError {
    #[error("failed to query events")]
    Database(#[from] sqlx::Error),
    #[error("failed to hash event password")]
    Password(#[from] password::PasswordError),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSummary {
    pub id: String,
    pub name: String,
    pub total_attendees: i64,
    pub distinct_users: i64,
}
#[derive(Debug, Serialize)]
pub struct Events {
    pub events: Vec<EventSummary>,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub name: String,
    pub color: String,
    pub created_at: UnixSeconds,
    pub created_by: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct CreatedEvent {
    pub event: Event,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attendee {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub created_at: UnixSeconds,
    pub image_id: Option<String>,
    pub abv: Option<f64>,
    pub drink_type: Option<DrinkType>,
    pub drink_size: Option<DrinkSize>,
    pub user_weight: Option<String>,
    pub user_gender: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct EventUser {
    pub id: String,
    pub username: String,
    pub weight: Option<String>,
    pub gender: Option<String>,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDetail {
    pub event: Event,
    pub attendees: Vec<Attendee>,
    pub access_users: Vec<EventUser>,
}

pub enum EventLookup {
    NotFound,
    Forbidden,
    Found(EventDetail),
}

pub enum UnlockResult {
    NotFound,
    InvalidPassword,
    Unlocked,
}

#[derive(Clone)]
pub struct EventsService {
    repository: EventsRepository,
}

impl EventsService {
    pub fn new(repository: EventsRepository) -> Self {
        Self { repository }
    }

    pub async fn list(&self, user_id: &str) -> Result<Events, EventsError> {
        let events = self
            .repository
            .for_user(user_id)
            .await?
            .into_iter()
            .map(|r| EventSummary {
                id: r.id,
                name: r.name,
                total_attendees: r.total_attendees,
                distinct_users: r.distinct_users,
            })
            .collect();
        Ok(Events { events })
    }

    pub async fn create(
        &self,
        name: &str,
        event_password: Option<&str>,
        user_id: &str,
    ) -> Result<CreatedEvent, EventsError> {
        let password_hash = match event_password {
            Some(password) => Some(password::hash(password).await?),
            None => None,
        };
        let created_at = UnixSeconds::now();
        let id = generate_event_id();
        let color = generate_soft_color();
        let record = self
            .repository
            .create(
                &id,
                name,
                &color,
                created_at,
                user_id,
                password_hash.as_deref(),
            )
            .await?;

        Ok(CreatedEvent {
            event: Event {
                id: record.id,
                name: record.name,
                color: record.color,
                created_at: UnixSeconds::from_seconds(record.created_at),
                created_by: record.created_by,
            },
        })
    }

    pub async fn get(&self, id: &str, user_id: &str) -> Result<EventLookup, EventsError> {
        let Some(record) = self.repository.event(id).await? else {
            return Ok(EventLookup::NotFound);
        };
        if record.password.is_some()
            && record.created_by.as_deref() != Some(user_id)
            && !self.repository.has_access(id, user_id).await?
        {
            return Ok(EventLookup::Forbidden);
        }

        let attendees = self
            .repository
            .attendees(id)
            .await?
            .into_iter()
            .map(|r| Attendee {
                id: r.id,
                user_id: r.user_id,
                username: r.username,
                created_at: UnixSeconds::from_seconds(r.created_at),
                image_id: r.image_id,
                abv: r.abv,
                drink_type: r.drink_type_id.map(|id| DrinkType {
                    id,
                    name: r.drink_type_name.expect("joined drink type has a name"),
                    description: r.drink_type_description,
                    abv: r.drink_type_abv,
                    multiplier: r.drink_type_multiplier.unwrap_or(1.0),
                }),
                drink_size: r.drink_size_id.map(|id| DrinkSize {
                    id,
                    name: r.drink_size_name.expect("joined drink size has a name"),
                    volume_ml: r
                        .drink_size_volume_ml
                        .expect("joined drink size has a volume"),
                    description: r.drink_size_description,
                }),
                user_weight: r.user_weight,
                user_gender: r.user_gender,
            })
            .collect();
        let access_users = self
            .repository
            .access_users(id)
            .await?
            .into_iter()
            .map(|r| EventUser {
                id: r.id,
                username: r.username,
                weight: r.weight,
                gender: r.gender,
            })
            .collect();
        Ok(EventLookup::Found(EventDetail {
            event: Event {
                id: record.id,
                name: record.name,
                color: record.color,
                created_at: UnixSeconds::from_seconds(record.created_at),
                created_by: record.created_by,
            },
            attendees,
            access_users,
        }))
    }

    pub async fn unlock(
        &self,
        id: &str,
        user_id: &str,
        supplied_password: &str,
    ) -> Result<UnlockResult, EventsError> {
        let Some(event) = self.repository.event(id).await? else {
            return Ok(UnlockResult::NotFound);
        };

        let Some(password_hash) = event.password else {
            return Ok(UnlockResult::Unlocked);
        };
        if event.created_by.as_deref() == Some(user_id)
            || self.repository.has_access(id, user_id).await?
        {
            return Ok(UnlockResult::Unlocked);
        }
        if !password::verify(supplied_password, &password_hash).await? {
            return Ok(UnlockResult::InvalidPassword);
        }

        self.repository
            .grant_access(id, user_id, UnixSeconds::now())
            .await?;
        Ok(UnlockResult::Unlocked)
    }
}

fn generate_event_id() -> String {
    const LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = rand::rng();
    let mut id = String::with_capacity(7);
    for _ in 0..2 {
        id.push(LETTERS[rng.random_range(0..LETTERS.len())] as char);
    }
    for _ in 0..5 {
        id.push(char::from(b'0' + rng.random_range(0..10)));
    }
    id
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::{EventLookup, EventsService, UnlockResult, generate_event_id};
    use crate::repositories::EventsRepository;

    async fn service() -> EventsService {
        let database = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&database).await.unwrap();
        for statement in [
            "INSERT INTO user (id, username, has_agreed_to_terms) VALUES ('owner', 'Owner', true), ('guest', 'Guest', true)",
            "INSERT INTO event (id, name, color, created_at, created_by, password) VALUES ('open', 'Open', '#fff', 1, 'owner', NULL), ('private', 'Private', '#000', 2, 'owner', 'hash')",
            "INSERT INTO attendee (id, event_id, user_id, created_at) VALUES ('drink', 'open', 'guest', 3)",
        ] {
            sqlx::query(statement).execute(&database).await.unwrap();
        }
        EventsService::new(EventsRepository::new(database))
    }

    #[tokio::test]
    async fn lists_events_the_user_created_or_participated_in() {
        let events = service().await.list("guest").await.unwrap().events;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].id, "open");
        assert_eq!(events[0].total_attendees, 1);
        assert_eq!(events[0].distinct_users, 1);
    }

    #[tokio::test]
    async fn protects_password_events_but_allows_the_creator() {
        let service = service().await;
        assert!(matches!(
            service.get("private", "guest").await.unwrap(),
            EventLookup::Forbidden
        ));
        let EventLookup::Found(detail) = service.get("private", "owner").await.unwrap() else {
            panic!("creator should have access");
        };
        assert_eq!(detail.event.name, "Private");
        assert_eq!(detail.access_users[0].id, "owner");
    }

    #[tokio::test]
    async fn creates_an_event_owned_by_the_current_user() {
        let service = service().await;
        let event = service
            .create("New event", None, "guest")
            .await
            .unwrap()
            .event;
        assert_eq!(event.name, "New event");
        assert_eq!(event.created_by.as_deref(), Some("guest"));
        assert!(event.color.starts_with('#'));
        assert!(matches!(
            service.get(&event.id, "guest").await.unwrap(),
            EventLookup::Found(_)
        ));
    }

    #[tokio::test]
    async fn unlocks_a_password_event_for_the_current_user() {
        let service = service().await;
        let event = service
            .create("Protected", Some("secret"), "owner")
            .await
            .unwrap()
            .event;

        assert!(matches!(
            service.unlock(&event.id, "guest", "wrong").await.unwrap(),
            UnlockResult::InvalidPassword
        ));
        assert!(matches!(
            service.get(&event.id, "guest").await.unwrap(),
            EventLookup::Forbidden
        ));
        assert!(matches!(
            service.unlock(&event.id, "guest", "secret").await.unwrap(),
            UnlockResult::Unlocked
        ));
        assert!(matches!(
            service.get(&event.id, "guest").await.unwrap(),
            EventLookup::Found(_)
        ));
    }

    #[tokio::test]
    async fn unlock_is_idempotent_for_accessible_events() {
        let service = service().await;
        assert!(matches!(
            service.unlock("open", "guest", "").await.unwrap(),
            UnlockResult::Unlocked
        ));
        assert!(matches!(
            service.unlock("private", "owner", "").await.unwrap(),
            UnlockResult::Unlocked
        ));
        assert!(matches!(
            service.unlock("missing", "guest", "secret").await.unwrap(),
            UnlockResult::NotFound
        ));
    }
}
