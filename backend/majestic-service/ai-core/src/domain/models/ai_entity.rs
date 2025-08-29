use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(String);
#[derive(Debug, Clone, )]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Metre(f32);
#[derive(Debug, Clone, PartialEq)]
pub struct Kilogram(f32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gender {
    Male,
    Female,
    Other
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Personality {
    Playful
}

#[derive(Debug, Clone, PartialEq)]
pub struct AIEntity {
    name: Name,
    height: Metre,
    weight: Kilogram,
    gender: Gender,
    personality: Personality,
    glb_file_url: Url,
}