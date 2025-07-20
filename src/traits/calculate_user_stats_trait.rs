use crate::models::users_models::Transfer;
use crate::models::users_models::UserStats;
use async_trait::async_trait;

#[async_trait]
pub trait StatsCalculator {
    async fn calculate_user_stats(&self, transfers: &[Transfer]) -> Vec<UserStats>;
}
