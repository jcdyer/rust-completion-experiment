use std::collections::BTreeMap;

use opaquekeys::{CourseKey, UsageKey};

use crate::User;
use crate::ports::{Result, ServiceError};
use crate::ports::course::CourseService;
use crate::ports::enrollment::{Enrollment, EnrollmentQuery, EnrollmentService, Role, State};

pub struct StubEnrollmentAdapter {
    enrollments: Vec<Enrollment>,
}

impl StubEnrollmentAdapter {
       pub fn new(enrollments: Vec<(User, CourseKey)>) -> StubEnrollmentAdapter {
        let enrollments =
            enrollments.into_iter()
                             .map(|(user, course)| {
                                 Enrollment {
                                     user,
                                     course,
                                     role: Role::Learner,
                                     state: State::Active,
                                 }
                             }).collect();
        StubEnrollmentAdapter { enrollments }
    }
}

impl EnrollmentService for StubEnrollmentAdapter {
    fn query_enrollment(&self, query: &EnrollmentQuery) -> Result<Vec<Enrollment>> {
        Ok(self.enrollments.iter()
            .filter(|enrollment| {
                let users = query.users.clone().unwrap_or_else(Vec::new);
                if users.is_empty() {
                    true
                } else {
                    users.contains(&enrollment.user)
                }
            })
            .filter(|enrollment| {
                let courses = query.courses.clone().unwrap_or_else(Vec::new);
                if courses.is_empty() {
                    true
                } else {
                    courses.contains(&enrollment.course)
                }
            })
            .cloned()
            .collect()
        )
    }
}


pub struct StubCourseAdapter {
    coursekey: CourseKey,
    blocks: BTreeMap<UsageKey, Vec<UsageKey>>
}

impl CourseService for StubCourseAdapter {
    fn get_course(&self, coursekey: &CourseKey) -> Result<BTreeMap<UsageKey, Vec<UsageKey>>> {
        if coursekey == &self.coursekey {
            Ok(self.blocks.clone())
        } else {
            Err(ServiceError::NotFound)
        }
    }
}