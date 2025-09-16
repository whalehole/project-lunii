// use crate::app::dto::ai_entity_view::AiEntityView;
// use crate::app::use_cases::create_ai_entity::{CreateAiEntityError, CreateAiEntityRequest};
//
// pub trait AiService: Clone + Send + Sync + 'static {
//     fn create_entity(&self, req: CreateAiEntityRequest) -> impl Future<Output = Result<AiEntityView, CreateAiEntityError>> + Send;
// }