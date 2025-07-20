mod generator;
mod globals;
mod models;
mod pipeline;
mod storage;
mod traits;
mod utils;

use crate::pipeline::default_pipeline::MockCalculator;
use crate::storage::storage::ClickHouseStorage;
use crate::traits::calculate_user_stats_trait::StatsCalculator;
use crate::traits::storage_trait::Storage;
use crate::traits::transfer_trait::TransferGenerator;
use generator::default_generator::DefaultTransferGenerator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let storage = init_table(
        globals::TABLE_URL.as_ref().map_err(|e| anyhow::anyhow!(e.to_string()))?,
        globals::TABLE_NAME.as_ref().map_err(|e| anyhow::anyhow!(e.to_string()))?,
    ).await?;

    let generator = DefaultTransferGenerator::default();
    let count = *globals::TRANSFER_COUNT
        .as_ref()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let transfers = generator.generate(count).await;
    
    storage
        .save_transfers(&transfers)
        .await?;

    let loaded_transfers = storage
        .load_transfers()
        .await?;

    let calculator = MockCalculator::default();
    let stats = calculator.calculate_user_stats(&loaded_transfers).await;

    for stat in stats.iter().take(10) {
        println!("{stat:?}");
    }

    Ok(())
}

async fn init_table(url: &str, name: &str) -> anyhow::Result<ClickHouseStorage> {
    let storage = ClickHouseStorage::new(url, name);
    storage.init_table().await?;
    Ok(storage)
}
