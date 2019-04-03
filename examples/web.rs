#![feature(proc_macro_hygiene, decl_macro)]

use std::path::PathBuf;

use rocket;
use rocket_contrib::json::Json;

use completion::{App, User};
use completion::adapters::{db, rest};

#[rocket::get("/<username>/<coursekey..>")]
fn index(username: String, coursekey: PathBuf) -> Json<serde_json::Value>
{

    let user = User { username };
    let coursekey = coursekey.to_string_lossy();
    let coursekey = coursekey.parse().unwrap();
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
    let result = app.get_user_completion(&user, &coursekey).expect("Could not fetch user completions");
    Json(serde_json::to_value(result).unwrap())
}

fn main() {
    rocket::ignite().mount("/", rocket::routes![index]).launch();
}
