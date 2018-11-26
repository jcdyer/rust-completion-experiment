use std::error::Error;

pub enum ServiceError {
    MultipleResults,
    Other(Box<dyn Error>),
}

pub type Result<T> = std::result::Result<T, ServiceError>;

pub mod coursestructure;
pub mod enrollment;
