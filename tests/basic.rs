#![cfg(test)]

use completion::{Aggregator, App, BlockCompletion, User};
use completion::adapters::stubs;

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
