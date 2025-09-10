use deadpool::managed::Object;
use deadpool_postgres::Manager;
use crate::PgPool;
use ai_core::app::ports::outb::ai_repository::AiEntityRepository;
use ai_core::app::use_cases::create_ai_entity::CreateAiEntityError;
use ai_core::domain::models::ai_entity::AiEntity;
use crate::ai::tables::ai_entities;

#[derive(Debug, Clone)]
pub struct PostgresAiEntityRepository {
    pool: PgPool,
}

impl PostgresAiEntityRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
    async fn save(&self, client: &Object<Manager>, entity: )
}

impl AiEntityRepository for PostgresAiEntityRepository {
    async fn create(&self, entity: &AiEntity) -> Result<AiEntity, CreateAiEntityError> {
        let client = self.pool.get_client().await?;

        let sql = format!(
            "INSERT INTO {table}",
            table = ai_entities::TABLE
        );

        let stmt = client.prepare(&sql).await?;


    }
}