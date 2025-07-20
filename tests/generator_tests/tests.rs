use mycrate::generator::default_generator::DefaultTransferGenerator;
use mycrate::generator::generator_config::TransferGenConfig;
use mycrate::traits::transfer_trait::TransferGenerator;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn test_generate_transfer_count() {
    let generator = DefaultTransferGenerator {config: TransferGenConfig {
        min_amount: 1.0,
        max_amount: 100.0,
        min_price: 0.5,
        max_price: 10.0,
        max_age_secs: 60 * 60 * 24,
    }};

    let count = 1000;
    let transfers = generator.generate(count).await;

    assert_eq!(
        transfers.len(),
        count,
        "There must be exactly {} transfers",
        count
    );
}

#[tokio::test]
async fn test_generate_transfer_ranges() {
    let config = TransferGenConfig {
        min_amount: 10.0,
        max_amount: 50.0,
        min_price: 1.0,
        max_price: 5.0,
        max_age_secs: 3600,
    };

    let generator = DefaultTransferGenerator {config: config.clone()};

    let transfers = generator.generate(500).await;

    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(dur) => dur.as_secs(),
        Err(e) => panic!("System tyme error: {e}"),
    };

    for t in &transfers {
        assert!(
            t.amount >= config.min_amount && t.amount <= config.max_amount,
            "amount {} out of range",
            t.amount
        );
        assert!(
            t.usd_price >= config.min_price && t.usd_price <= config.max_price,
            "usd_price {} out of range",
            t.usd_price
        );
        assert!(
            t.ts <= now && t.ts >= now - config.max_age_secs,
            "timestamp {} out of range",
            t.ts
        );
        assert_ne!(t.from, t.to, "from и to совпадают: {}", t.from);
    }
}

#[tokio::test]
async fn test_generate_addresses_are_randomized() {
    let generator = DefaultTransferGenerator {config: TransferGenConfig {
        min_amount: 1.0,
        max_amount: 100.0,
        min_price: 0.5,
        max_price: 10.0,
        max_age_secs: 1000,
    }};

    let transfers = generator.generate(100).await;

    let mut addresses = HashSet::new();
    for t in &transfers {
        addresses.insert(t.from.clone());
        addresses.insert(t.to.clone());
    }

    assert!(
        addresses.len() > 10,
        "Too few unique addresses: {:?}",
        addresses
    );
}
