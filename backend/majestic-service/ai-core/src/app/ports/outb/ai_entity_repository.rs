use crate::app::use_cases::create_ai_entity::CreateAiEntityError;
use crate::domain::models::ai_entity::AiEntity;

pub trait AiEntityRepository: Clone + Send + Sync + 'static {
    fn create(&self, entity: &AiEntity) -> impl Future<Output = Result<AiEntity, CreateAiEntityError>> + Send;
}