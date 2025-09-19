use anyhow::Context;
use deadpool::managed::Object;
use deadpool_postgres::Manager;
use indexmap::IndexSet;
use postgres_types::ToSql;
use time::Date;
use url::Url;
use uuid::Uuid;
use crate::{PgPool, TransactionError};
use ai_core::app::ports::outb::ai_entity_repository::AiEntityRepository;
use ai_core::app::use_cases::create_ai_entity::CreateAiEntityError;
use ai_core::domain::models::ai_entity::{AiEntity, AiEntityId, AiEntityName};
use ai_core::domain::models::{Birthday, Height, Weight};
use ai_core::domain::models::gender::GenderId;
use ai_core::domain::models::personality::PersonalityId;
use crate::ai::repositories::{build_insert_query, Row};
use crate::ai::tables::{ai_entities, ai_entities_personalities};

#[derive(Debug, Clone)]
pub struct PostgresAiEntityRepository {
    pool: PgPool,
}

impl PostgresAiEntityRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    async fn save(&self, client: Object<Manager>, entity: &AiEntity) -> Result<AiEntity, TransactionError> {
        let ai_entities_columns = [
            ai_entities::col::ID,
            ai_entities::col::NAME,
            ai_entities::col::HEIGHT,
            ai_entities::col::WEIGHT,
            ai_entities::col::GENDER_ID,
            ai_entities::col::BIRTHDAY,
            ai_entities::col::GLB_FILE_URL
        ];
        let sql_a = build_insert_query(
            ai_entities::TABLE,
            1,
            &ai_entities_columns,
        );
        let ai_entity_row = client.query_one(&sql_a, &[
            &entity.id().to_row(),
            &entity.name().to_row(),
            &entity.height().to_row(),
            &entity.weight().to_row(),
            &entity.gender().to_row(),
            &entity.birthday().to_row(),
            &entity.glb_file_url().as_str()
        ]).await?;

        let ai_entities_personalities_columns = [
            ai_entities_personalities::col::AI_ENTITY_ID,
            ai_entities_personalities::col::PERSONALITY_ID
        ];
        let sql_b = build_insert_query(
            ai_entities_personalities::TABLE,
            entity.personalities().len(),
            &ai_entities_personalities_columns
        );
        let mut params_b = Vec::with_capacity(
            entity.personalities().len()*ai_entities_personalities_columns.len()
        );
        for personality in entity.personalities() {
            params_b.push(entity.id().to_row());
            params_b.push(personality.to_row());
        }
        let params_b: Vec<&(dyn ToSql + Sync)> = params_b.iter().map(
            |p| p as &(dyn ToSql + Sync)
        ).collect();
        let ai_entities_personalities_rows = client.query(
            &sql_b,
            &params_b
        ).await?;

        let saved = AiEntity::new(
            AiEntityId::from_row(ai_entity_row.get(ai_entities::col::ID))?,
            AiEntityName::from_row(ai_entity_row.get(ai_entities::col::NAME))?,
            Height::from_row(ai_entity_row.get(ai_entities::col::HEIGHT))?,
            Weight::from_row(ai_entity_row.get(ai_entities::col::WEIGHT))?,
            GenderId::from_row(ai_entity_row.get(ai_entities::col::GENDER_ID))?,
            Birthday::from_row(ai_entity_row.get(ai_entities::col::BIRTHDAY))?,
            PersonalityId::from_rows(
                ai_entities_personalities_rows.iter().map(
                    |p| p.get(ai_entities_personalities::col::PERSONALITY_ID)
                ).collect()
            )?.into_iter().collect::<IndexSet<_>>(),
            Url::from_row(ai_entity_row.get(ai_entities::col::GLB_FILE_URL))?
        );

        Ok(saved)
    }
}

impl AiEntityRepository for PostgresAiEntityRepository {
    async fn create(&self, entity: &AiEntity) -> Result<AiEntity, CreateAiEntityError> {
        let client = self.pool.get().await
            .with_context(|| "failed to get postgres client from pool")?;
        let created = self.save(client, entity).await
            .with_context(|| "failed to save ai entity")?;
        Ok(created)
    }
}

impl Row<Uuid> for AiEntityId {
    fn from_row(value: Uuid) -> Result<Self, anyhow::Error> { Ok(Self::new(value)) }
    fn to_row(&self) -> Uuid { *self.as_uuid() }
}

impl Row<String> for AiEntityName {
    fn from_row(value: String) -> Result<Self, anyhow::Error> {
        Self::new(value.as_str())
            .with_context(
                || format!("Failed to parse AiEntityName from database column value: '{}'", value)
            )
    }
    fn to_row(&self) -> String { self.to_string() }
}

impl Row<Date> for Birthday {
    fn from_row(value: Date) -> Result<Self, anyhow::Error> { Ok(Self::new(value)) }
    fn to_row(&self) -> Date { *self.as_date() }
}