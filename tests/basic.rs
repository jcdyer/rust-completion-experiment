#![cfg(test)]

use completion::{Aggregator, App, BlockCompletion, User};
use completion::adapters::{db, stubs};

use opaquekeys::{CourseKey, PartialUsageKey};

#[test]
fn test_get_user_completion() {
    let user = User {
        username: "test_user".to_owned(),
    };
    let course: CourseKey = "course-v1:edX+DemoX+DemoCourse".parse().unwrap();
    let usagekeys: Vec<_> = vec![
        "block-v1:edX+DemoX+DemoCourse+type@course+block@course"
            .parse()
            .unwrap(),
        "block-v1:edX+DemoX+DemoCourse+type@chapter+block@chapter1"
            .parse()
            .unwrap(),
        "block-v1:edX+DemoX+DemoCourse+type@html+block@intro"
            .parse()
            .unwrap(),
        "block-v1:edX+DemoX+DemoCourse+type@poll+block@poll"
            .parse()
            .unwrap(),
        "block-v1:edX+DemoX+DemoCourse+type@discussion+block@chatroom"
            .parse()
            .unwrap(),
    ].into_iter()
        .map(|key: PartialUsageKey| key.try_promote().unwrap())
        .collect();

    let blockcompletion_service = stubs::StubBlockCompletionAdapter::new(vec![
        BlockCompletion {
            user: user.clone(),
            block_key: usagekeys[3].clone(),
            completion: 1.0,
        },
    ]);
    let course_service = stubs::StubCourseAdapter::new(
        course.clone(),
        vec![
            (usagekeys[0].clone(), vec![usagekeys[1].clone()]),
            (
                usagekeys[1].clone(),
                vec![
                    usagekeys[2].clone(),
                    usagekeys[3].clone(),
                    usagekeys[4].clone(),
                ],
            ),
        ].into_iter()
            .collect(),
    );

    let enrollment_service =
        stubs::StubEnrollmentAdapter::new(vec![(user.clone(), course.clone())]);

    let app = App::new(blockcompletion_service, course_service, enrollment_service);
    let result = app.get_user_completion(&user, &course).unwrap();
    assert_eq!(
        result,
        vec![
            Aggregator {
                user: user.clone(),
                block_key: usagekeys[1].clone(),
                earned: 1.0,
                possible: 2.0,
            },
            Aggregator {
                user: user.clone(),
                block_key: usagekeys[0].clone(),
                earned: 1.0,
                possible: 2.0,
            },
        ]
    )
}

#[test]
fn test_db_adapter() {
    // This test needs a configured connection to an edxapp DB.  You will need
    // to set the following variables:
    //
    // MYSQL_EDXAGG_USER
    // MYSQL_EDXAGG_PASSWORD
    // MYSQL_EDXAGG_DATABASE
    //
    // And optionally:
    //
    // MYSQL_EDXAGG_HOST (defaults to localhost)
    // MYSQL_EDXAGG_PORT (defaults to 3306)
    let user = User {
        username: "jcd".to_owned(),
    };
    let course: CourseKey = "course-v1:edX+DemoX+Demo_Course".parse().unwrap();
    let usagekeys: Vec<_> = vec![
        "block-v1:edX+DemoX+Demo_Course+type@course+block@course"
            .parse()
            .unwrap(),
        "block-v1:edX+DemoX+Demo_Course+type@chapter+block@chapter1"
            .parse()
            .unwrap(),
        "block-v1:edX+DemoX+Demo_Course+type@html+block@intro"
            .parse()
            .unwrap(),
        "block-v1:edX+DemoX+Demo_Course+type@html+block@8293139743f34377817d537b69911530"
            .parse()
            .unwrap(),
        "block-v1:edX+DemoX+Demo_Course+type@video+block@5c90cffecd9b48b188cbfea176bf7fe9"
            .parse()
            .unwrap(),
        "block-v1:edX+DemoX+Demo_Course+type@html+block@0a3b4139f51a4917a3aff9d519b1eeb6"
            .parse()
            .unwrap(),
    ].into_iter()
        .map(|key: PartialUsageKey| key.try_promote().unwrap())
        .collect();

    let conn = db::edxapp_connect().expect("mysql connect");
    let blockcompletion_service = {
        let conn = conn.clone();
        db::MySqlBlockCompletionAdapter::new(conn)
    };
    let enrollment_service = {
        let conn = conn.clone();
        db::MySqlEnrollmentAdapter::new(conn)
    };
    let course_service = stubs::StubCourseAdapter::new(
        course.clone(),
        vec![
            (usagekeys[0].clone(), vec![usagekeys[1].clone()]),
            (
                usagekeys[1].clone(),
                vec![
                    usagekeys[2].clone(),
                    usagekeys[3].clone(),
                    usagekeys[4].clone(),
                    usagekeys[5].clone(),
                ],
            ),
        ].into_iter()
            .collect(),
    );

    let app = App::new(blockcompletion_service, course_service, enrollment_service);
    let result = app.get_user_completion(&user, &course).unwrap();
    assert_eq!(
        result,
        vec![
            Aggregator {
                user: user.clone(),
                block_key: usagekeys[1].clone(),
                earned: 3.0,
                possible: 4.0,
            },
            Aggregator {
                user: user.clone(),
                block_key: usagekeys[0].clone(),
                earned: 3.0,
                possible: 4.0,
            },
        ]
    )
}
