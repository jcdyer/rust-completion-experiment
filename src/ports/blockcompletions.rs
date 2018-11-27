use std::collections::BTreeMap;

use opaquekeys::{CourseKey, UsageKey};

use crate::{BlockCompletion, User};
use super::Result;

pub trait BlockCompletionService {
    fn get_course_blockcompletions(
        &self,
        coursekey: &CourseKey,
    ) -> Result<BTreeMap<(User, UsageKey), BlockCompletion>>;
    fn get_user_blockcompletions(
        &self,
        user: &User,
        coursekey: &CourseKey,
    ) -> Result<BTreeMap<(User, UsageKey), BlockCompletion>>;
}
