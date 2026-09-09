use beer_storage::ImageStorage;

use crate::{
    config::Config,
    database::Database,
    repositories::{AuthRepository, EventsRepository, HealthRepository, LeaderboardRepository},
    services::{
        AuthService, EventsService, HealthService, LeaderboardService, drinks::DrinksService,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub auth: AuthService,
    pub drinks: DrinksService,
    pub events: EventsService,
    pub health: HealthService,
    pub leaderboard: LeaderboardService,
    pub secure_cookies: bool,
}

impl AppState {
    pub fn new(database: Database, config: &Config, images: ImageStorage) -> Self {
        let drinks = DrinksService::new(database.clone(), images);
        let auth = AuthService::new(AuthRepository::new(database.clone()));
        let events = EventsService::new(EventsRepository::new(database.clone()));
        let health = HealthService::new(HealthRepository::new(database.clone()));
        let leaderboard = LeaderboardService::new(LeaderboardRepository::new(database));

        Self {
            auth,
            drinks,
            events,
            health,
            leaderboard,
            secure_cookies: !config.development,
        }
    }
}
