use std::error::Error;

#[derive(Debug)]
pub enum ServiceError {
    NotFound,
    MultipleResults,
    Other(Box<Error>),
}

pub type Result<T> = std::result::Result<T, ServiceError>;

pub mod blockcompletions;
pub mod course;
pub mod enrollment;
