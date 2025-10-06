use std::ops::DerefMut;
use anyhow::Context;
use adapter_out_db_postgres::create_pool_from_env;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
    let mut conn = create_pool_from_env().await?
        .get().await?;
    migrations::runner().run_async(conn.deref_mut().deref_mut()).await
        .with_context(|| "failed to run postgres database migration")?;
    Ok(())
}