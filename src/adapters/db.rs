use std::collections::BTreeMap;

use mysql;
use opaquekeys::{CourseKey, PartialUsageKey, UsageKey};

use crate::ports::blockcompletions::BlockCompletionService;
use crate::ports::enrollment::{Enrollment, EnrollmentQuery, EnrollmentService};
use crate::ports::Result;
use crate::{BlockCompletion, User};

pub fn edxapp_connect() -> Option<mysql::Pool> {
    match mysql::Pool::new(format!(
        "mysql://{}:{}@{}:{}/{}",
        std::env::var("EDXAGG_MYSQL_USER").expect("EDXAGG_MYSQL_USER not provided"),
        std::env::var("EDXAGG_MYSQL_PASSWORD").expect("EDXAGG_MYSQL_PASSWORD not provided"),
        std::env::var("EDXAGG_MYSQL_HOST").unwrap_or_else(|_| "localhost".to_string()),
        std::env::var("EDXAGG_MYSQL_PORT").unwrap_or_else(|_| "3306".to_string()),
        std::env::var("EDXAGG_MYSQL_DATABASE").expect("EDXAGG_MYSQL_DATABASE not provided"),
    )) {
        Ok(conn) => Some(conn),
        Err(error) => panic!("{}", error),
    }
}

pub struct MySqlBlockCompletionAdapter {
    conn: mysql::Pool,
}

impl MySqlBlockCompletionAdapter {
    pub fn new(conn: mysql::Pool) -> MySqlBlockCompletionAdapter {
        MySqlBlockCompletionAdapter { conn }
    }
}

impl BlockCompletionService for MySqlBlockCompletionAdapter {
    fn get_course_blockcompletions(
        &self,
        coursekey: &CourseKey,
    ) -> Result<BTreeMap<(User, UsageKey), BlockCompletion>> {
        let query = "SELECT username, block_key, completion
            FROM completion_blockcompletion
                JOIN auth_user
                    ON user_id = auth_user.id
            WHERE course_key = :coursekey";
        let completions = self.conn
            .prep_exec(
                query,
                params!{
                    "coursekey" => format!("{}", coursekey),
                },
            )
            .map(|result| {
                result
                    .map(|rowresult| rowresult.unwrap())
                    .map(|row| {
                        let (username, blockkeyraw, completion): (
                            _,
                            String,
                            _,
                        ) = mysql::from_row(row);
                        let block_key: UsageKey = blockkeyraw
                            .parse::<PartialUsageKey>()
                            .unwrap()
                            .map_into_course(coursekey.clone());
                        let user = User { username };
                        let user2 = user.clone();
                        let blockkey2 = block_key.clone();
                        (
                            (user2, blockkey2),
                            BlockCompletion {
                                user,
                                block_key,
                                completion,
                            },
                        )
                    })
                    .collect()
            })
            .unwrap();
        Ok(completions)
    }
    fn get_user_blockcompletions(
        &self,
        user: &User,
        coursekey: &CourseKey,
    ) -> Result<BTreeMap<(User, UsageKey), BlockCompletion>> {
        let query = "SELECT username, course_key, block_key, completion
            FROM completion_blockcompletion
                JOIN auth_user
                    ON user_id = auth_user.id
            WHERE course_key = :coursekey
                AND username = :username";
        let completions = self.conn
            .prep_exec(
                query,
                params!{
                    "coursekey" => coursekey.to_string(),
                    "username" => user.username.to_string(),
                },
            )
            .map(|result| {
                result
                    .map(|rowresult| rowresult.unwrap())
                    .map(|row| {
                        let (username, _coursekey, blockkeyraw, completion) =
                            mysql::from_row::<(String, String, String, f64)>(row);
                        let block_key: UsageKey = blockkeyraw
                            .parse::<PartialUsageKey>()
                            .unwrap()
                            .map_into_course(coursekey.clone());
                        let user = User { username };
                        let user2 = user.clone();
                        let blockkey2 = block_key.clone();
                        (
                            (user2, blockkey2),
                            BlockCompletion {
                                user,
                                block_key,
                                completion,
                            },
                        )
                    })
                    .collect()
            })
            .unwrap();
        Ok(completions)
    }
}

pub struct MySqlEnrollmentAdapter {
    conn: mysql::Pool,
}

impl MySqlEnrollmentAdapter {
    pub fn new(conn: mysql::Pool) -> MySqlEnrollmentAdapter {
        MySqlEnrollmentAdapter { conn }
    }
    fn build_sql_from_query(&self, query: &EnrollmentQuery) -> String {
        let mut qstr = String::from("SELECT username, course_id FROM student_courseenrollment JOIN auth_user ON auth_user.id = user_id");
        let mut where_clauses = vec![];
        if let Some(ref courses) = query.courses {
            where_clauses.push(format!(
                " course_id IN ({})",
                courses
                    .iter()
                    .map(|course_key| format!("'{}'", course_key))
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }
        if let Some(ref users) = query.users {
            where_clauses.push(format!(
                " username IN ('{}')",
                users
                    .iter()
                    .map(|user| user.username.as_ref())
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }
        if !where_clauses.is_empty() {
            let mut first = true;
            qstr.push_str(" WHERE");
            for clause in where_clauses {
                if !first {
                    qstr.push_str(" AND");
                } else {
                    first = false;
                }
                qstr.push_str(&clause);
            }
        }
        qstr
    }
}

impl EnrollmentService for MySqlEnrollmentAdapter {
    fn query_enrollment(&self, query: &EnrollmentQuery) -> Result<Vec<Enrollment>> {
        let qstr = self.build_sql_from_query(query);
        let enrollments = self.conn
            .prep_exec(qstr, mysql::Params::Empty)
            .map(|result| {
                result
                    .map(|rowresult| rowresult.unwrap())
                    .map(|row| {
                        let (username, coursekey) = mysql::from_row::<(String, String)>(row);
                        let user = User { username };
                        let course = coursekey.parse().unwrap();
                        Enrollment { user, course }
                    })
                    .collect()
            })
            .unwrap();
        Ok(enrollments)
    }
}
