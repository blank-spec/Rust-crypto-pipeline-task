use async_trait::async_trait;
use crate::models::users_models::Transfer;

#[async_trait]
pub trait TransferGenerator {
    async fn generate(&self, count: usize) -> Vec<Transfer>;
}