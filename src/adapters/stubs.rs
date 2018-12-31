use std::collections::BTreeMap;

use opaquekeys::{CourseKey, UsageKey};

use crate::{BlockCompletion, User};
use crate::ports::{Result, ServiceError};
use crate::ports::blockcompletions::BlockCompletionService;
use crate::ports::course::CourseService;
use crate::ports::enrollment::{Enrollment, EnrollmentQuery, EnrollmentService};

pub struct StubBlockCompletionAdapter {
    blockcompletions: Vec<BlockCompletion>,
}

impl StubBlockCompletionAdapter {
    pub fn new(blockcompletions: Vec<BlockCompletion>) -> StubBlockCompletionAdapter {
        StubBlockCompletionAdapter { blockcompletions }
    }
}

impl BlockCompletionService for StubBlockCompletionAdapter {
    fn get_course_blockcompletions(
        &self,
        coursekey: &CourseKey,
    ) -> Result<BTreeMap<(User, UsageKey), BlockCompletion>> {
        Ok(self.blockcompletions
            .iter()
            .filter(|bc| bc.block_key.course_key() == coursekey)
            .map(|bc| ((bc.user.clone(), bc.block_key.clone()), bc.clone()))
            .collect())
    }
    fn get_user_blockcompletions(
        &self,
        user: &User,
        coursekey: &CourseKey,
    ) -> Result<BTreeMap<(User, UsageKey), BlockCompletion>> {
        Ok(self.blockcompletions
            .iter()
            .filter(|bc| bc.block_key.course_key() == coursekey)
            .filter(|bc| &bc.user == user)
            .map(|bc| ((bc.user.clone(), bc.block_key.clone()), bc.clone()))
            .collect())
    }
}
pub struct StubEnrollmentAdapter {
    enrollments: Vec<Enrollment>,
}

impl StubEnrollmentAdapter {
    pub fn new(enrollments: Vec<(User, CourseKey)>) -> StubEnrollmentAdapter {
        let enrollments = enrollments
            .into_iter()
            .map(|(user, course)| Enrollment {
                user,
                course,
            })
            .collect();
        StubEnrollmentAdapter { enrollments }
    }
}

impl EnrollmentService for StubEnrollmentAdapter {
    fn query_enrollment(&self, query: &EnrollmentQuery) -> Result<Vec<Enrollment>> {
        Ok(self.enrollments
            .iter()
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
            .collect())
    }
}

pub struct StubCourseAdapter {
    coursekey: CourseKey,
    blocks: BTreeMap<UsageKey, Vec<UsageKey>>,
}

impl StubCourseAdapter {
    /// This does not check that the usage keys actually belong to the right
    /// course, or even the same course.
    pub fn new(
        coursekey: CourseKey,
        blocks: BTreeMap<UsageKey, Vec<UsageKey>>,
    ) -> StubCourseAdapter {
        StubCourseAdapter { coursekey, blocks }
    }
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
