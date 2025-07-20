use crate::models::users_models::Transfer;
use async_trait::async_trait;

#[async_trait]
pub trait Storage {
    async fn save_transfers(&self, transfers: &[Transfer]) -> anyhow::Result<()>;
    async fn load_transfers(&self) -> anyhow::Result<Vec<Transfer>>;
}
