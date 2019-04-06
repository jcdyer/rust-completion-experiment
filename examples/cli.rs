use std::error::Error;

use opaquekeys::CourseKey;

use completion::{App, User};
use completion::adapters::{db, rest};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rust-completion-experiment")]

struct CliOptions {
    #[structopt(parse(try_from_str))]
    user: User,
    #[structopt(parse(try_from_str))]
    course_key: CourseKey,
}

fn main() -> Result<(), Box<Error>> {
    let CliOptions { user, course_key } = CliOptions::from_args();
    dbg!(&course_key);
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
    let result = app.get_user_completion(&user, &course_key).unwrap();
    for agg in result {
        println!(
            "{}: {}/{} ({:.2}%)",
            agg.block_key,
            agg.earned,
            agg.possible,
            agg.percent() * 100.0
        );
    }
    Ok(())
}
