use crate::{
    config::Config,
    database::Database,
    repositories::{AuthRepository, EventsRepository, HealthRepository, LeaderboardRepository},
    services::{AuthService, EventsService, HealthService, LeaderboardService},
};

#[derive(Clone)]
pub struct AppState {
    pub auth: AuthService,
    pub events: EventsService,
    pub health: HealthService,
    pub leaderboard: LeaderboardService,
    pub secure_cookies: bool,
}

impl AppState {
    pub fn new(database: Database, config: &Config) -> Self {
        let auth = AuthService::new(AuthRepository::new(database.clone()));
        let events = EventsService::new(EventsRepository::new(database.clone()));
        let health = HealthService::new(HealthRepository::new(database.clone()));
        let leaderboard = LeaderboardService::new(LeaderboardRepository::new(database));

        Self {
            auth,
            events,
            health,
            leaderboard,
            secure_cookies: !config.development,
        }
    }
}
