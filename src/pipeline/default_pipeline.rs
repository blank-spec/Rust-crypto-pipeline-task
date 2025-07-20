use crate::models::users_models::Transfer;
use crate::models::users_models::UserStats;
use crate::traits::calculate_user_stats_trait::StatsCalculator;
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Default)]
pub struct MockCalculator;

#[async_trait]
impl StatsCalculator for MockCalculator {
    async fn calculate_user_stats(&self, transfers: &[Transfer]) -> Vec<UserStats> {
        struct PriceStats {
            total_amount: f64,
            total_value: f64,
        }

        struct UserAgg {
            balance: f64,
            max_balance: f64,
            buy: PriceStats,
            sell: PriceStats,
        }

        let mut stats: HashMap<String, UserAgg> = HashMap::new();

        for t in transfers {
            let seller = stats.entry(t.from.clone()).or_insert_with(|| UserAgg {
                balance: 0.0,
                max_balance: 0.0,
                buy: PriceStats {
                    total_amount: 0.0,
                    total_value: 0.0,
                },
                sell: PriceStats {
                    total_amount: 0.0,
                    total_value: 0.0,
                },
            });

            seller.balance -= t.amount;
            if seller.balance < 0.0 {
                seller.balance = 0.0;
            }
            seller.sell.total_amount += t.amount;
            seller.sell.total_value += t.amount * t.usd_price;

            let buyer = stats.entry(t.to.clone()).or_insert_with(|| UserAgg {
                balance: 0.0,
                max_balance: 0.0,
                buy: PriceStats {
                    total_amount: 0.0,
                    total_value: 0.0,
                },
                sell: PriceStats {
                    total_amount: 0.0,
                    total_value: 0.0,
                },
            });

            buyer.balance += t.amount;
            buyer.max_balance = buyer.max_balance.max(buyer.balance);
            buyer.buy.total_amount += t.amount;
            buyer.buy.total_value += t.amount * t.usd_price;
        }

        stats
            .into_iter()
            .map(|(address, data)| {
                let total_volume = data.buy.total_amount + data.sell.total_amount;
                let avg_buy_price = if data.buy.total_amount > 0.0 {
                    data.buy.total_value / data.buy.total_amount
                } else {
                    0.0
                };
                let avg_sell_price = if data.sell.total_amount > 0.0 {
                    data.sell.total_value / data.sell.total_amount
                } else {
                    0.0
                };

                UserStats {
                    address,
                    total_volume,
                    avg_buy_price,
                    avg_sell_price,
                    max_balance: data.max_balance,
                }
            })
            .collect()
    }
}
