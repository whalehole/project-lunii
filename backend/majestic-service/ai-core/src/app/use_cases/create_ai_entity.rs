use thiserror::Error;
use url::Url;
use crate::domain::models::ai_entity::AiEntityName;
use crate::domain::models::{Height, Weight};
use crate::domain::models::gender::GenderId;
use crate::domain::models::personality::PersonalityId;

#[derive(Debug, Clone, PartialEq)]
pub struct CreateAiEntityRequest {
    pub name: AiEntityName,
    pub height: Height,
    pub weight: Weight,
    pub gender: GenderId,
    pub personalities: Vec<PersonalityId>,
    pub glb_file_url: Url,
}

#[derive(Debug, Error)]
pub enum CreateAiEntityError {
    #[error("AI entity with name {name} already exists")]
    Duplicate { name: AiEntityName },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}