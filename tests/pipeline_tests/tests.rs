#[cfg(test)]
mod tests {
    use mycrate::models::users_models::Transfer;
    use mycrate::pipeline::default_pipeline::MockCalculator;
    use mycrate::traits::calculate_user_stats_trait::StatsCalculator;

    #[tokio::test]
    async fn test_single_transfer() {
        let calculator = MockCalculator::default();

        let transfers = vec![Transfer {
            ts: 0,
            from: "alice".to_string(),
            to: "bob".to_string(),
            amount: 100.0,
            usd_price: 2.0,
        }];

        let stats = calculator.calculate_user_stats(&transfers).await;

        assert_eq!(stats.len(), 2);
        
        let (alice_stats, bob_stats) = match (
            stats.iter().find(|s| s.address == "alice"),
            stats.iter().find(|s| s.address == "bob"),
        ) {
            (Some(alice_stats), Some(bob_stats)) => (alice_stats, bob_stats),
            _ => panic!("Missing one or more expected addresses"),
        };

        assert_eq!(alice_stats.total_volume, 100.0);
        assert_eq!(alice_stats.avg_sell_price, 2.0);
        assert_eq!(alice_stats.avg_buy_price, 0.0);
        assert_eq!(alice_stats.max_balance, 0.0);

        assert_eq!(bob_stats.total_volume, 100.0);
        assert_eq!(bob_stats.avg_buy_price, 2.0);
        assert_eq!(bob_stats.avg_sell_price, 0.0);
        assert_eq!(bob_stats.max_balance, 100.0);
    }

    #[tokio::test]
    async fn test_multiple_transfers() {
        let calculator = MockCalculator::default();

        let transfers = vec![
            Transfer {
                ts: 0,
                from: "alice".to_string(),
                to: "bob".to_string(),
                amount: 50.0,
                usd_price: 3.0,
            },
            Transfer {
                ts: 1,
                from: "bob".to_string(),
                to: "carol".to_string(),
                amount: 30.0,
                usd_price: 4.0,
            },
            Transfer {
                ts: 2,
                from: "carol".to_string(),
                to: "alice".to_string(),
                amount: 20.0,
                usd_price: 5.0,
            },
        ];

        let stats = calculator.calculate_user_stats(&transfers).await;

        let (alice, bob, carol) = match (
            stats.iter().find(|s| s.address == "alice"),
            stats.iter().find(|s| s.address == "bob"),
            stats.iter().find(|s| s.address == "carol"),
        ) {
            (Some(alice), Some(bob), Some(carol)) => (alice, bob, carol),
            _ => panic!("Missing one or more expected addresses"),
        };

        assert_eq!(alice.total_volume, 70.0);
        assert_eq!(alice.avg_sell_price, 3.0);
        assert_eq!(alice.avg_buy_price, 5.0);
        assert_eq!(alice.max_balance, 20.0);

        assert_eq!(bob.total_volume, 80.0);
        assert_eq!(bob.avg_buy_price, 3.0);
        assert_eq!(bob.avg_sell_price, 4.0);
        assert_eq!(bob.max_balance, 50.0);

        assert_eq!(carol.total_volume, 50.0);
        assert_eq!(carol.avg_buy_price, 4.0);
        assert_eq!(carol.avg_sell_price, 5.0);
        assert_eq!(carol.max_balance, 30.0);
    }

    #[tokio::test]
    async fn test_zero_amount_transfer() {
        let calculator = MockCalculator::default();

        let transfers = vec![Transfer {
            ts: 123,
            from: "a".to_string(),
            to: "b".to_string(),
            amount: 0.0,
            usd_price: 1000.0,
        }];

        let stats = calculator.calculate_user_stats(&transfers).await;

        assert_eq!(stats.len(), 2);
        for stat in stats {
            assert_eq!(stat.total_volume, 0.0);
            assert_eq!(stat.avg_buy_price, 0.0);
            assert_eq!(stat.avg_sell_price, 0.0);
            assert_eq!(stat.max_balance, 0.0);
        }
    }

    #[tokio::test]
    async fn test_empty_transfers() {
        let calculator = MockCalculator::default();
        let transfers = vec![];
        let stats = calculator.calculate_user_stats(&transfers).await;
        assert!(stats.is_empty());
    }
}
