use std::ops::DerefMut;
use adapter_out_db_postgres::create_pool_from_env;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
    let pg_client = create_pool_from_env().await?
        .get_client().await?
        .deref_mut().deref_mut();
    migrations::runner().run_async(pg_client).await?;
    Ok(())
}