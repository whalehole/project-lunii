use url::Url;
use uuid::Uuid;
use crate::domain::models::ai_entity::{Gender, Height, Name, Personality, Weight};

#[derive(Debug, Clone, PartialEq)]
pub struct AiEntityView {
    pub uuid: Uuid,
    pub name: Name,
    pub height: Height,
    pub weight: Weight,
    pub gender: Gender,
    pub personality: Personality,
    pub glb_file_url: Url,
}