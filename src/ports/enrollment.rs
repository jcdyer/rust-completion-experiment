
use opaquekeys::CourseKey;

use crate::User;
use super::{Result, ServiceError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Enrollment {
    pub course: CourseKey,
    pub user: User,
    pub role: Role,
    pub state: State,
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct EnrollmentQuery {
    pub courses: Option<Vec<CourseKey>>,
    pub users: Option<Vec<User>>,
    pub roles: Option<Vec<Role>>,
    pub states: Option<Vec<State>>,
}

impl EnrollmentQuery {
    pub fn add_courses(mut self, courses: &[CourseKey]) -> EnrollmentQuery {
        if self.courses.is_none() {
            self.courses = Some(Vec::with_capacity(courses.len()))
        }

        let coursesfilter = self.courses.get_or_insert_with({
            let courselen = courses.len();
            move || Vec::with_capacity(courselen)
        });
        coursesfilter.extend_from_slice(&courses);
        self
    }

    pub fn add_users(mut self, mut users: Vec<User>) -> EnrollmentQuery {
        if self.users.is_none() {
            self.users = Some(Vec::with_capacity(users.len()))
        }
        let _ = self.users
            .iter_mut()
            .map(|usersfilter| usersfilter.append(&mut users));
        self
    }
}

pub trait EnrollmentService {
    fn query_enrollment(&self, query: &EnrollmentQuery) -> Result<Vec<Enrollment>>;
    fn get_enrolled_users(&self, course: &CourseKey) -> Result<Vec<Enrollment>> {
        self.query_enrollment(&EnrollmentQuery::default().add_courses(&vec![course.clone()]))
    }
    fn get_enrolled_courses(&self, user: &User) -> Result<Vec<Enrollment>> {
        self.query_enrollment(&EnrollmentQuery::default().add_users(vec![user.clone()]))
    }
    fn get_enrollment(&self, user: &User, course: &CourseKey) -> Result<Option<Enrollment>> {
        let mut enrollments = self.query_enrollment(&EnrollmentQuery::default()
            .add_courses(&vec![course.clone()])
            .add_users(vec![user.clone()]))?;
        let len = enrollments.len();
        if len == 0 {
            Ok(None)
        } else if len == 1 {
            Ok(Some(enrollments.pop().unwrap()))
        } else {
            Err(ServiceError::MultipleResults)
        }
    }
    fn is_enrolled(&self, user: &User, course: &CourseKey) -> Result<bool>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Role {
    Learner,
    TA,
    Instructor,
    SuperUser,
    Other(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    Active,
    Inactive,
}
