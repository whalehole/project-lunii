use anyhow::Context;
use deadpool::managed::PoolError;
use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod, Runtime};
use dotenvy::dotenv;
use serde::Deserialize;
use thiserror::Error;
use tokio_postgres::NoTls;

mod ai;

pub type PgPool = Pool;

#[derive(Debug, Deserialize)]
struct Config {
    pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

pub async fn create_pool_from_env() -> anyhow::Result<PgPool> {
    dotenv().ok();
    let mut cfg = Config::from_env().with_context(|| "failed to load config for deadpool postgres")?;
    cfg.pg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    Ok(cfg.pg.create_pool(Some(Runtime::Tokio1), NoTls)
        .with_context(|| "couldn't create postgres connection pool")?)
}

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("Postgres connection pool error: {0}")]
    Pool(#[from] PoolError<tokio_postgres::Error>),
    #[error("Postgres transaction operation failed: {0}")]
    Operation(#[from] tokio_postgres::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
