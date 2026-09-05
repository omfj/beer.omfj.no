use crate::{
    config::Config,
    database::Database,
    repositories::{AuthRepository, HealthRepository, LeaderboardRepository},
    services::{AuthService, HealthService, LeaderboardService},
};

#[derive(Clone)]
pub struct AppState {
    pub auth: AuthService,
    pub health: HealthService,
    pub leaderboard: LeaderboardService,
    pub secure_cookies: bool,
}

impl AppState {
    pub fn new(database: Database, config: &Config) -> Self {
        let auth = AuthService::new(AuthRepository::new(database.clone()));
        let health = HealthService::new(HealthRepository::new(database.clone()));
        let leaderboard = LeaderboardService::new(LeaderboardRepository::new(database));

        Self {
            auth,
            health,
            leaderboard,
            secure_cookies: !config.development,
        }
    }
}
