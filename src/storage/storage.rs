use anyhow::Context;
use crate::models::users_models::Transfer;
use crate::models::storage_models::ClickHouseStorage;
use crate::traits::storage_trait::Storage;
use async_trait::async_trait;
use clickhouse::Client;


impl ClickHouseStorage {
    pub fn new(database_url: &str, table: &str) -> Self {
        let client = Client::default().with_url(database_url);
        Self {
            client,
            table: table.to_string(),
        }
    }

    pub async fn init_table(&self) -> anyhow::Result<()> {
        let ddl = format!(
            "
            CREATE TABLE IF NOT EXISTS {} (
                ts UInt64,
                `from` String,
                `to` String,
                amount Float64,
                usd_price Float64
            ) ENGINE = MergeTree()
            ORDER BY ts
            ",
            self.table
        );

        self.client
            .query(&ddl)
            .execute()
            .await
            .with_context(|| "Table creation failed".to_string())
    }
}

#[async_trait]
impl Storage for ClickHouseStorage {
    async fn save_transfers(&self, transfers: &[Transfer]) -> anyhow::Result<()> {
        let mut writer = self
            .client
            .insert(&self.table)
            .with_context(|| "Insert init failed")?;

        for t in transfers {
            writer
                .write(t)
                .await
                .with_context(|| format!("Write failed for transfer: {:?}", t))?;
        }

        writer
            .end()
            .await
            .with_context(|| "Insert commit failed")?;

        Ok(())
    }

    async fn load_transfers(&self) -> anyhow::Result<Vec<Transfer>> {
        let query = format!("SELECT * FROM {}", self.table);

        self.client
            .query(&query)
            .fetch_all::<Transfer>()
            .await
            .with_context(|| format!("Load failed for query: `{}`", query))
    }
}
