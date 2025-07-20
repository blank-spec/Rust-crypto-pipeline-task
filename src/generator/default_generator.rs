use crate::generator::generator_config::TransferGenConfig;
use crate::models::users_models::Transfer;
use crate::traits::transfer_trait::TransferGenerator;
use crate::utils::rand_address;
use async_trait::async_trait;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct DefaultTransferGenerator {
    pub config: TransferGenConfig,
}

#[async_trait]
impl TransferGenerator for DefaultTransferGenerator {
    async fn generate(&self, count: usize) -> Vec<Transfer> {
        let mut rng = rand::thread_rng();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|dur| dur.as_secs())
            .unwrap_or(0);

        (0..count)
            .map(|_| {
                let from = rand_address(&mut rng);
                let to = rand_address(&mut rng);
                let amount = rng.gen_range(self.config.min_amount..self.config.max_amount);
                let usd_price = rng.gen_range(self.config.min_price..self.config.max_price);
                let ts = now - rng.gen_range(0..self.config.max_age_secs);

                Transfer {
                    ts,
                    from,
                    to,
                    amount,
                    usd_price,
                }
            })
            .collect()
    }
}

impl Default for DefaultTransferGenerator {
    fn default() -> Self {
        Self {
            config: TransferGenConfig::default(),
        }
    }
}
