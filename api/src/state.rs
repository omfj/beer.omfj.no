use crate::{
    database::Database,
    repositories::{HealthRepository, LeaderboardRepository},
    services::{HealthService, LeaderboardService},
};

#[derive(Clone)]
pub struct AppState {
    pub health: HealthService,
    pub leaderboard: LeaderboardService,
}

impl AppState {
    pub fn new(database: Database) -> Self {
        let health = HealthService::new(HealthRepository::new(database.clone()));
        let leaderboard = LeaderboardService::new(LeaderboardRepository::new(database));

        Self {
            health,
            leaderboard,
        }
    }
}
