use std::error::Error;

use opaquekeys::CourseKey;

use completion::{App, User};
use completion::ports::ServiceError;
use completion::adapters::{db, rest};

fn main() -> Result<(), Box<Error>> {
    let user = User {
        username: "test_user".to_owned(),
    };
    let course: CourseKey = "course-v1:edX+DemoX+DemoCourse".parse()?;

    let conn = db::edxapp_connect().expect("mysql connect");
    let blockcompletion_service = {
        let conn = conn.clone();
        db::MySqlBlockCompletionAdapter::new(conn)
    };
    let enrollment_service = {
        let conn = conn.clone();
        db::MySqlEnrollmentAdapter::new(conn)
    };
    let course_service = rest::CourseAdapter::new();

    let app = App::new(blockcompletion_service, course_service, enrollment_service);
    let result = app.get_user_completion(&user, &course);
    println!("result: {:?}", result);
    Ok(())
}
