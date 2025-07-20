use clickhouse::Client;

#[derive(Clone)]
pub struct ClickHouseStorage {
    pub client: Client,
    pub table: String,
}
