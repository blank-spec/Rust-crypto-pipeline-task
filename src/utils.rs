use std::env;
use anyhow::Context;
use rand::{distributions::Alphanumeric, Rng};

pub fn rand_address(rng: &mut impl Rng) -> String {
    let suffix: String = rng
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!("0x{}", suffix)
}

pub fn load_env_var(name: &str) -> anyhow::Result<String> {
    dotenv::dotenv().ok();
    env::var(name).with_context(|| format!("Environment variable `{}` is not set", name))
}
