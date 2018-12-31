use std::error::Error;

#[derive(Debug)]
pub enum ServiceError {
    NotFound,
    MultipleResults,
    Other(Box<Error>),
}

impl ServiceError {
    pub fn from_error<E: 'static + Error>(err: E) -> ServiceError {
        ServiceError::Other(Box::new(err))
    }
}
impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ServiceError: {:?}", self)
    }
}

pub type Result<T> = std::result::Result<T, ServiceError>;

pub mod blockcompletions;
pub mod course;
pub mod enrollment;
