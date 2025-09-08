use crate::PgPool;
use ai_core::app::ports::outb::ai_repository::AiEntityRepository;
use ai_core::app::use_cases::create_ai_entity::CreateAiEntityError;
use ai_core::domain::models::ai_entity::AiEntity;

#[derive(Debug, Clone)]
pub struct PostgresAiEntityRepository {
    pool: PgPool,
}

impl PostgresAiEntityRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }
}

impl AiEntityRepository for PostgresAiEntityRepository {
    async fn create(&self, entity: &AiEntity) -> Result<AiEntity, CreateAiEntityError> {
        let client = self.pool.get_client().await?;

        let stmt = client.prepare(
            "INSERT INTO ai_entities()"
        )
    }
}