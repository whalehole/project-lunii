use anyhow::Context;
use deadpool::managed::Object;
use deadpool_postgres::{Manager, Transaction};
use crate::{PgPool, TransactionError, Transactional};
use ai_core::app::ports::outb::ai_entity_repository::AiEntityRepository;
use ai_core::app::use_cases::create_ai_entity::CreateAiEntityError;
use ai_core::domain::models::ai_entity::{AiEntity, AiEntityId};
use crate::ai::tables::ai_entities;

#[derive(Debug, Clone)]
pub struct PostgresAiEntityRepository {
    pool: PgPool,
}

impl PostgresAiEntityRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    async fn save(&self, client: Object<Manager>, entity: &AiEntity) -> Result<AiEntity, TransactionError> {
        self.with_tx(|tx| async move {
            let sql = format!(
                "INSERT INTO {} ({}, {}, {}, {}, {}, {}, {}) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                ai_entities::TABLE,
                ai_entities::col::ID,
                ai_entities::col::NAME,
                ai_entities::col::HEIGHT,
                ai_entities::col::WEIGHT,
                ai_entities::col::GENDER_ID,
                ai_entities::col::BIRTHDAY,
                ai_entities::col::GLB_FILE_URL
            );

            let stmt = client.prepare_cached(&sql).await?;
            let rows = client.execute(&stmt, &[
                entity.id(),
                entity.name(),
                entity.height(),
                entity.weight(),
                entity.gender(),
                entity.birthday(),
                entity.glb_file_url()
            ]);

            Ok(AiEntity::new())
        })
    }
}

impl Transactional for PostgresAiEntityRepository {
    fn pool(&self) -> &PgPool { &self.pool }
}

impl AiEntityRepository for PostgresAiEntityRepository {
    async fn create(&self, entity: &AiEntity) -> Result<AiEntity, CreateAiEntityError> {
        let mut tx = self.get_tx().await?;
        self.save(client, entity)
    }
}