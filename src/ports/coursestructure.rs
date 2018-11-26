    use opaquekeys::{CourseKey, UsageKey};

    use crate::aggregator::Course;
    use super::{Error, Result};


    pub trait CourseService {
        fn get_course(coursekey: &CourseKey) -> Result<Course>;
        fn get_subgraph(coursekey: &CourseKey, rootblockkey: &UsageKey) -> Result<Course>;
    }

