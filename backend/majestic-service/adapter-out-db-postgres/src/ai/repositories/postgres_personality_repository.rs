use uuid::Uuid;
use ai_core::domain::models::personality::PersonalityId;
use crate::ai::repositories::Row;

impl Row<Uuid> for PersonalityId {
    fn from_row(value: Uuid) -> Result<Self, anyhow::Error> { Ok(Self::new(value)) }
    fn to_row(&self) -> Uuid { *self.as_uuid() }
}