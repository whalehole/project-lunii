use anyhow::Context;
use deadpool::managed::PoolError;
use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod, Runtime, Transaction};
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

pub trait Transactional {
    fn pool(&self) -> &PgPool;

    async fn with_tx<T, Fut, F>(&self, f: F) -> Result<T, TransactionError>
    where
        F: FnOnce(&mut Transaction) -> Fut + Send,
        Fut: Future<Output = Result<T, TransactionError>> + Send,
        T: Send
    {
        let mut client = self.pool().get().await?;
        let mut tx = client.build_transaction().start().await?;
        match f(&mut tx).await {
            Ok(res) => { tx.commit().await?; Ok(res) },
            Err(err) => Err(err)
        }
    }
}

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("Postgres connection pool error: {0}")]
    Pool(#[from] PoolError<tokio_postgres::Error>),
    #[error("Postgres transaction operation failed: {0}")]
    Operation(#[from] tokio_postgres::Error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
