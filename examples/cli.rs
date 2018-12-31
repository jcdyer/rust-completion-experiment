use std::error::Error;

use opaquekeys::CourseKey;

use completion::{App, User};
use completion::adapters::{db, rest};

fn main() -> Result<(), Box<Error>> {
    let args = clap::App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            clap::Arg::with_name("user")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("course_key")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let username = args.value_of("user").unwrap().to_owned();
    let user = User { username };
    let course: CourseKey = args.value_of("course_key").unwrap().parse()?;

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
    let result = app.get_user_completion(&user, &course).unwrap();
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
