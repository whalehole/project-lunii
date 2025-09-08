use anyhow::Context;
use deadpool::managed::Object;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod, Runtime};
use dotenvy::dotenv;
use serde::Deserialize;
use tokio_postgres::NoTls;

mod ai;
mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

// pub type PgPool = deadpool_postgres::Pool;
#[derive(Debug, Clone)]
pub struct PgPool {
    pool: Pool
}

impl PgPool {
    pub async fn get_client(&self) -> anyhow::Result<Object<Manager>>{
        self.pool.get().await.with_context(|| "failed to get postgres client from pool")
    }
}

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
    let mut cfg = Config::from_env().with_context(|| "failed to load config")?;
    cfg.pg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool = cfg.pg.create_pool(Some(Runtime::Tokio1), NoTls)
        .with_context(|| "couldn't create postgres connection pool")?;
    Ok(PgPool { pool })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
