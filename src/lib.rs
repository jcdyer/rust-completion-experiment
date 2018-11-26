use opaquekeys::{UsageKey};
use crate::ports::coursestructure::CourseService;
use crate::ports::enrollment::EnrollmentService;

pub mod aggregator;
pub mod ports;
pub mod xblock;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    username: String,
    email: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockCompletion {
    user: User,
    block_key: UsageKey,
    completion: f64,
}


pub struct App<C, E>
where
    C: CourseService,
    E: EnrollmentService,
{

    course_service: C,
    enrollment_service: E,
}


impl<C, E> App<C, E>
where
    C: CourseService,
    E: EnrollmentService,
{
    pub fn new(course_service: C, enrollment_service: E) -> App<C, E> {
        App { course_service, enrollment_service }
    }
}
