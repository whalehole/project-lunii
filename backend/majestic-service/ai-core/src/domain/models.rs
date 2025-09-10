use std::fmt::Display;
use thiserror::Error;

pub mod ai_entity;
pub mod personality;
pub mod gender;

#[derive(Debug, Clone, Error)]
#[error("Name cannot be empty")]
pub struct NameEmptyError;

#[derive(Debug, Error)]
pub enum NameError {
    #[error("Name has invalid characters")]
    InvalidCharacters,
    #[error("Name must be at least {0} characters long")]
    TooShort(usize),
    #[error("Name must not be longer than {0} characters")]
    TooLong(usize),
    #[error("Name cannot be empty")]
    Empty
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Height(Metre);

impl Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} m", self.0)
    }
}

#[derive(Debug, Clone, Error)]
#[error("Height cannot be negative")]
pub struct NegativeHeightError(Metre);

impl Height {
    pub fn new(m: Metre) -> Result<Self, NegativeHeightError> {
        if m.0 < 0.0 { Err(NegativeHeightError(m)) }
        else { Ok(Self(m)) }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Weight(Kilogram);

impl Display for Weight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} kg", self.0)
    }
}

#[derive(Debug, Clone, Error)]
#[error("Weight cannot be negative")]
pub struct NegativeWeightError(Kilogram);

impl Weight {
    pub fn new(kg: Kilogram) -> Result<Self, NegativeWeightError> {
        if kg.0 < 0.0 { Err(NegativeWeightError(kg)) }
        else { Ok(Self(kg)) }
    }
}

