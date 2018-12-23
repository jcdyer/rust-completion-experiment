#[macro_use]
extern crate mysql;

use opaquekeys::{CourseKey, UsageKey};

use crate::aggregator::Course;
use crate::ports::blockcompletions::BlockCompletionService;
use crate::ports::course::CourseService;
use crate::ports::enrollment::EnrollmentService;

pub mod adapters;
pub mod aggregator;
pub mod ports;
pub mod xblock;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct User {
    pub username: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockCompletion {
    pub user: User,
    pub block_key: UsageKey,
    pub completion: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Aggregator {
    pub user: User,
    pub block_key: UsageKey,
    pub earned: f64,
    pub possible: f64,
}

pub struct App<B, C, E>
where
    B: BlockCompletionService,
    C: CourseService,
    E: EnrollmentService,
{
    blockcompletion_service: B,
    course_service: C,
    enrollment_service: E,
}

impl<B, C, E> App<B, C, E>
where
    B: BlockCompletionService,
    C: CourseService,
    E: EnrollmentService,
{
    pub fn new(
        blockcompletion_service: B,
        course_service: C,
        enrollment_service: E,
    ) -> App<B, C, E> {
        App {
            blockcompletion_service,
            course_service,
            enrollment_service,
        }
    }

    pub fn get_user_completion(
        &self,
        user: &User,
        coursekey: &CourseKey,
    ) -> Option<Vec<Aggregator>> {
        if self.enrollment_service
            .is_enrolled(user, coursekey)
            .unwrap_or(true)
        {
            let structure = self.course_service.get_course(coursekey).unwrap(); // CRASH!!!
            let course = Course::from_structure(&structure);
            let blockcompletions = self.blockcompletion_service
                .get_user_blockcompletions(user, coursekey)
                .unwrap_or_default();
            Some(course.aggregate(user, &blockcompletions))
        } else {
            None
        }
    }
}
