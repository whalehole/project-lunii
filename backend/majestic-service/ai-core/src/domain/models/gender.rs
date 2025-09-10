use std::fmt::Display;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::domain::models::NameError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenderId(Uuid);

impl GenderId {
    pub fn new(id: Uuid) -> Self { Self(id) }
}

impl Display for GenderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenderName(String);

impl GenderName {
    pub fn new(name: &str) -> Result<Self, NameError> {
        let trimmed = name.trim();
        if trimmed.is_empty() { Err(NameError::Empty) }
        else { Ok(GenderName(trimmed.to_string())) }
    }
}

impl Display for GenderName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gender {
    id: GenderId,
    name: GenderName,
    created_on: OffsetDateTime
}

impl Gender {
    pub fn new(id: GenderId, name: GenderName) -> Self {
        Self { id, name, created_on: OffsetDateTime::now_utc() }
    }

    pub fn id(&self) -> &GenderId { &self.id }
    pub fn name(&self) -> &GenderName { &self.name }
    pub fn created_on(&self) -> &OffsetDateTime { &self.created_on }
}


