#[cfg(test)]
mod tests {
    use std::clone::Clone;
    use uuid::Uuid;

    use tokio;

    use mycrate::globals;
    use mycrate::models::users_models::Transfer;
    use mycrate::models::storage_models::ClickHouseStorage;
    use mycrate::traits::storage_trait::Storage;

    fn test_table() -> String {
        format!("transfers_test_{}", Uuid::new_v4())
    }

    fn sample_transfer(ts: u64) -> Transfer {
        Transfer {
            ts,
            from: format!("13132from{}", ts),
            to: format!("13132to{}", ts),
            amount: ts as f64 * 1.1,
            usd_price: 3000.0 + ts as f64,
        }
    }

    #[tokio::test]
    async fn test_table_initialization_twice() -> anyhow::Result<()> {
        let table = test_table();
        let url = globals::TABLE_URL.as_ref().map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let storage = ClickHouseStorage::new(url, &table);

        storage.init_table().await?;
        storage.init_table().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_save_and_load_single_transfer() -> anyhow::Result<()> {
        let table = test_table();
        let url = globals::TABLE_URL.as_ref().map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let storage = ClickHouseStorage::new(url, &table);

        storage.init_table().await?;

        let tx = sample_transfer(1);
        storage.save_transfers(&[tx.clone()]).await?;

        let loaded = storage.load_transfers().await?;
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].from, tx.from);
        assert_eq!(loaded[0].to, tx.to);
        Ok(())
    }

    #[tokio::test]
    async fn test_bulk_insert_and_load() -> anyhow::Result<()> {
        let table = test_table();
        let url = globals::TABLE_URL.as_ref().map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let storage = ClickHouseStorage::new(url, &table);

        storage.init_table().await?;

        let transfers: Vec<_> = (1..=100).map(sample_transfer).collect();
        storage.save_transfers(&transfers).await?;

        let loaded = storage.load_transfers().await?;
        assert_eq!(loaded.len(), 100);
        Ok(())
    }

    #[tokio::test]
    async fn test_empty_table_returns_empty_vec() -> anyhow::Result<()> {
        let table = test_table();
        let url = globals::TABLE_URL.as_ref().map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let storage = ClickHouseStorage::new(url, &table);

        storage.init_table().await?;
        let loaded = storage.load_transfers().await?;
        assert!(loaded.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_insert_zero_transfers_does_nothing() -> anyhow::Result<()> {
        let table = test_table();
        let url = globals::TABLE_URL.as_ref().map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let storage = ClickHouseStorage::new(url, &table);

        storage.init_table().await?;
        storage.save_transfers(&[]).await?;

        let loaded = storage.load_transfers().await?;
        assert!(loaded.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_connection_fails() {
        let storage = ClickHouseStorage::new("http://localhost:9999", "some_table");
        let result = storage.load_transfers().await;

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Load failed") || err.contains("error") || err.contains("refused"));
    }

}
