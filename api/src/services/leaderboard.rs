use crate::domain::time::LeaderboardYear;
use std::{cmp::Ordering, collections::HashMap};

use serde::Serialize;
use thiserror::Error;

use crate::{
    domain::scoring::{calculate_drink_points, round_to_one_decimal},
    repositories::LeaderboardRepository,
};

#[derive(Debug, Error)]
pub enum LeaderboardError {
    #[error("failed to query leaderboard")]
    Database(#[from] sqlx::Error),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardEntry {
    pub user_id: String,
    pub username: String,
    pub points: f64,
    pub drink_count: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Leaderboard {
    pub leaderboard: Vec<LeaderboardEntry>,
    pub selected_year: LeaderboardYear,
    pub available_years: Vec<LeaderboardYear>,
}

#[derive(Clone)]
pub struct LeaderboardService {
    repository: LeaderboardRepository,
}

impl LeaderboardService {
    pub fn new(repository: LeaderboardRepository) -> Self {
        Self { repository }
    }

    pub async fn get(&self, year: LeaderboardYear) -> Result<Leaderboard, LeaderboardError> {
        let records = self.repository.records(year).await?;
        let available_years = self.repository.available_years().await?;
        let mut users = HashMap::<String, LeaderboardEntry>::new();

        for record in records {
            let points = calculate_drink_points(
                record.volume_ml.map(|value| value as f64),
                record.abv.or(record.fallback_abv.map(|value| value as f64)),
                record.multiplier,
            );
            let entry = users
                .entry(record.user_id.clone())
                .or_insert_with(|| LeaderboardEntry {
                    user_id: record.user_id,
                    username: record.username,
                    points: 0.0,
                    drink_count: 0,
                });

            entry.points += points;
            entry.drink_count += 1;
        }

        let mut leaderboard: Vec<_> = users.into_values().collect();
        for entry in &mut leaderboard {
            entry.points = round_to_one_decimal(entry.points);
        }
        leaderboard.sort_by(|left, right| {
            right
                .points
                .partial_cmp(&left.points)
                .unwrap_or(Ordering::Equal)
                .then_with(|| left.username.cmp(&right.username))
                .then_with(|| left.user_id.cmp(&right.user_id))
        });
        leaderboard.truncate(10);

        Ok(Leaderboard {
            leaderboard,
            selected_year: year,
            available_years,
        })
    }
}
