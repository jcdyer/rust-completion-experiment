use std::collections::BTreeMap;

use opaquekeys::{CourseKey, UsageKey};

use super::Result;

pub trait CourseService {
    /// Returns a BTreeMap mapping each UsageKey in the course graph to a vec of its children.
    fn get_course(&self, coursekey: &CourseKey) -> Result<BTreeMap<UsageKey, Vec<UsageKey>>>;
}
