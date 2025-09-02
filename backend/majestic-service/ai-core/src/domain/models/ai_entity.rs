use std::fmt::Display;
use thiserror::Error;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(String);

#[derive(Debug, Clone, Error)]
#[error("Name cannot be empty")]
pub struct NameEmptyError;

impl Name {
    pub fn new(s: &str) -> Result<Self, NameEmptyError> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(NameEmptyError)
        } else {
            Ok(Name(trimmed.to_owned()))
        }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Metre(f32);

impl Metre {
    pub fn new(m: f32) -> Self {
        Metre(m)
    }
}

impl Display for Metre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} m", self.0)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Kilogram(f32);

impl Kilogram {
    pub fn new(k: f32) -> Self {
        Kilogram(k)
    }
}

impl Display for Kilogram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} kg", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
    Other
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Personality {
    Playful
}

#[derive(Debug, Clone, PartialEq)]
pub struct AIEntity {
    uuid: Uuid,
    name: Name,
    height: Metre,
    weight: Kilogram,
    gender: Gender,
    personality: Personality,
    glb_file_url: Url,
}

impl AIEntity {
    pub fn new(uuid: Uuid, name: Name, height: Metre, weight: Kilogram, gender: Gender, personality: Personality, glb_file_url: Url) -> Self {
        Self { uuid, name, height, weight, gender, personality, glb_file_url }
    }

    pub fn uuid(&self) -> &Uuid { &self.uuid }
    pub fn name(&self) -> &Name { &self.name }
    pub fn height(&self) -> &Metre { &self.height }
    pub fn weight(&self) -> &Kilogram { &self.weight }
    pub fn gender(&self) -> &Gender { &self.gender }
    pub fn personality(&self) -> &Personality { &self.personality }
    pub fn glb_file_url(&self) -> &Url { &self.glb_file_url }
}

pub struct CreateAIEntityRequest {
    pub name: Name,
    pub height: Metre,
    pub weight: Kilogram,
    pub gender: Gender,
    pub personality: Personality,
    pub glb_file_url: Url,
}

#[derive(Debug, Error)]
pub enum CreateAIEntityError {
    #[error("AI entity with name {name} already exists")]
    Duplicate { name: Name },
    #[error("AI entity must have a name")]
    NoName,
    #[error("AI entity's height cannot be less than zero")]
    NegativeHeight,
    #[error("AI entity's weight cannot be less than zero")]
    NegativeWeight,
}