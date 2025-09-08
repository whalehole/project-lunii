use thiserror::Error;
use url::Url;
use crate::domain::models::ai_entity::{Gender, Height, Name, Personality, Weight};

pub struct CreateAiEntityRequest {
    pub name: Name,
    pub height: Height,
    pub weight: Weight,
    pub gender: Gender,
    pub personality: Personality,
    pub glb_file_url: Url,
}

#[derive(Debug, Error)]
pub enum CreateAiEntityError {
    #[error("AI entity with name {name} already exists")]
    Duplicate { name: Name },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}