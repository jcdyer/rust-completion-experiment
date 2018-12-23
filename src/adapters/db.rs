use std::collections::BTreeMap;

use mysql;
use opaquekeys::{CourseKey, PartialUsageKey, UsageKey};

use crate::ports::blockcompletions::BlockCompletionService;
use crate::ports::Result;
use crate::{BlockCompletion, User};

fn connect() -> Option<mysql::Pool> {
    mysql::Pool::new(format!(
        "mysql://{}:{}@{}:{}/completion",
        env!("EDXAGG_MYSQL_USER"),
        env!("EDXAGG_MYSQL_PASSWORD"),
        option_env!("EDXAGG_MYSQL_HOST").unwrap_or("localhost"),
        option_env!("EDXAGG_MYSQL_PORT").unwrap_or("3306"),
    )).ok()
}

pub struct MySqlBlockCompletionAdapter {
    conn: mysql::Pool,
}

impl MySqlBlockCompletionAdapter {
    pub fn new() -> Option<MySqlBlockCompletionAdapter> {
        connect().map(|conn| MySqlBlockCompletionAdapter { conn })
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
}
