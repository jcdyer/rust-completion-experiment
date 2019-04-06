#[macro_use]
extern crate mysql;

use opaquekeys::{CourseKey, UsageKey};
use serde_derive::{Serialize};

use crate::aggregator::Course;
use crate::ports::blockcompletions::BlockCompletionService;
use crate::ports::course::CourseService;
use crate::ports::enrollment::EnrollmentService;

pub mod adapters;
pub mod aggregator;
pub mod ports;
pub mod xblock;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct User {
    pub username: String,
}

impl std::str::FromStr for User {
    type Err = String;
    fn from_str(username: &str) -> Result<User, String> {
        let known_users = vec!["cliff", "cliff2", "jcd", "staff", "mcka_admin_user", "mcka_ta_user"];
        if known_users.contains(&username) {
            Ok(User { username: username.to_owned() })
        } else {
            Err("User does not exist".into())
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockCompletion {
    pub user: User,
    pub block_key: UsageKey,
    pub completion: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Aggregator {
    pub user: User,
    pub block_key: UsageKey,
    pub earned: f64,
    pub possible: f64,
}

impl Aggregator {
    pub fn percent(&self) -> f64 {
        if self.possible == 0.0 {
            0.0
        } else {
            self.earned / self.possible
        }
    }
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
            let structure = self.course_service.get_course(coursekey).ok()?;
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
