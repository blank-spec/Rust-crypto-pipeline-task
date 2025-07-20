use once_cell::sync::Lazy;
use anyhow::Context;
use crate::utils::load_env_var;
pub static TABLE_URL: Lazy<anyhow::Result<String>> = Lazy::new(|| {
    load_env_var("TABLE_URL")
});

pub static TABLE_NAME: Lazy<anyhow::Result<String>> = Lazy::new(|| {
    load_env_var("TABLE_NAME")
});

pub static TRANSFER_COUNT: Lazy<anyhow::Result<usize>> = Lazy::new(|| {
    let val = load_env_var("TRANSFER_COUNT")?;
    val.parse::<usize>()
        .with_context(|| format!("Environment variable `{val}` is not a valid usize"))
});
