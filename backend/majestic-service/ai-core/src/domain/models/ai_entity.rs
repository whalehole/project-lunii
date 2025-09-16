use std::fmt::Display;
use indexmap::IndexSet;
use time::{Date, OffsetDateTime};
use url::Url;
use uuid::Uuid;
use crate::domain::models::gender::GenderId;
use crate::domain::models::{Height, NameError, Weight};
use crate::domain::models::personality::PersonalityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AiEntityId(Uuid);

impl AiEntityId {
    pub fn new(id: Uuid) -> Self { Self(id) }
}

impl Display for AiEntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiEntityName(String);

impl AiEntityName {
    pub fn new(name: &str) -> Result<Self, NameError> {
        let trimmed = name.trim();
        if trimmed.is_empty() { Err(NameError::Empty) }
        else { Ok(AiEntityName(trimmed.to_owned())) }
    }
}

impl Display for AiEntityName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct AiEntity {
    id: AiEntityId,
    name: AiEntityName,
    height: Height,
    weight: Weight,
    gender: GenderId,
    birthday: Date,
    personalities: IndexSet<PersonalityId>,
    glb_file_url: Url,
    created_on: OffsetDateTime,
    last_modified_on: OffsetDateTime
}

impl AiEntity {
    pub fn new(
        id: AiEntityId,
        name: AiEntityName,
        height: Height,
        weight: Weight,
        gender: GenderId,
        birthday: Date,
        personalities: IndexSet<PersonalityId>,
        glb_file_url: Url
    ) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id,
            name,
            height,
            weight,
            gender,
            birthday,
            personalities,
            glb_file_url,
            created_on: now,
            last_modified_on: now
        }
    }

    pub fn id(&self) -> &AiEntityId { &self.id }
    pub fn name(&self) -> &AiEntityName { &self.name }
    pub fn height(&self) -> &Height { &self.height }
    pub fn weight(&self) -> &Weight { &self.weight }
    pub fn gender(&self) -> &GenderId { &self.gender }
    pub fn birthday(&self) -> &Date { &self.birthday }
    pub fn personalities(&self) -> &IndexSet<PersonalityId> { &self.personalities }
    pub fn glb_file_url(&self) -> &Url { &self.glb_file_url }

    fn on_modify(&mut self) {
        self.last_modified_on = OffsetDateTime::now_utc();
    }
    pub fn rename(&mut self, new_name: AiEntityName) {
        self.name = new_name;
        self.on_modify();
    }
    pub fn change_height(&mut self, new_height: Height) {
        self.height = new_height;
        self.on_modify();
    }
    pub fn change_weight(&mut self, new_weight: Weight) {
        self.weight = new_weight;
        self.on_modify();
    }
    pub fn change_gender(&mut self, new_gender: GenderId) {
        self.gender = new_gender;
        self.on_modify();
    }
    pub fn change_birthday(&mut self, new_birthday: Date) {
        self.birthday = new_birthday;
        self.on_modify();
    }
    pub fn change_personalities(&mut self, new_personalities: IndexSet<PersonalityId>) {
        self.personalities = new_personalities;
        self.on_modify();
    }
    pub fn add_personality(&mut self, personality: PersonalityId) {
        self.personalities.insert(personality);
        self.on_modify();
    }
}