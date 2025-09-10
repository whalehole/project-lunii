use std::fmt::Display;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::domain::models::NameError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PersonalityId(Uuid);

impl PersonalityId {
    fn new(id: Uuid) -> Self { Self(id) }
}

impl Display for PersonalityId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonalityName(String);

impl PersonalityName {
    fn new(name: &str) -> Result<Self, NameError> {
        let trimmed = name.trim();
        if trimmed.is_empty() { Err(NameError::Empty) }
        else { Ok(Self(name.to_string())) }
    }
}

impl Display for PersonalityName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Personality {
    id: PersonalityId,
    name: PersonalityName,
    created_on: OffsetDateTime
}

impl Personality {
    pub fn new(id: PersonalityId, name: PersonalityName) -> Self {
        Self { id, name, created_on: OffsetDateTime::now_utc() }
    }
    
    pub fn id(&self) -> &PersonalityId { &self.id }
    pub fn name(&self) -> &PersonalityName { &self.name }
    pub fn created_on(&self) -> &OffsetDateTime { &self.created_on }
}