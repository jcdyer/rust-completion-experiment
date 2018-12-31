use std::cell::RefCell;
use std::collections::BTreeMap;

use reqwest;
use serde_derive;
use serde_json;

use opaquekeys::{CourseKey, UsageKey};

use crate::ports::{Result, ServiceError};
use crate::ports::course::CourseService;

pub struct CourseAdapter {
    api_root_url: String,
    oauth_token_url: String,
    client: reqwest::Client,
    access_token: RefCell<Option<String>>,
}

static CLIENT_ID: Option<&'static str> = option_env!("EDXAGG_OAUTH_CLIENT_ID");
static CLIENT_SECRET: Option<&'static str> = option_env!("EDXAGG_OAUTH_CLIENT_ID");

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
struct TokenRequest<'a> {
    grant_type: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
}

impl CourseAdapter {
    pub fn new() -> CourseAdapter {
        CourseAdapter {
            client: reqwest::Client::new(),
            api_root_url: "http://localhost:8000/api/courses/v1/".to_owned(),
            oauth_token_url: "http://localhost:8000/oauth2/access_token/".to_owned(),
            access_token: RefCell::default(),
        }
    }

    fn get_new_token(&self) -> Result<String> {
        let form = TokenRequest {
            grant_type: "client_credentials",
            client_id: CLIENT_ID.unwrap_or("open"),
            client_secret: CLIENT_SECRET.unwrap_or("sesame"),
        };
        let resp = self.client
            .post(&self.oauth_token_url)
            .form(&form)
            .send()
            .map_err(ServiceError::from_error)?;
        resp.status();
        let data: serde_json::Value =
            serde_json::from_reader(resp).map_err(ServiceError::from_error)?;
        Ok(data["access_token"]
            .as_str()
            .ok_or_else(|| ServiceError::NotFound)?
            .to_owned())
    }
}

impl Default for CourseAdapter {
    fn default() -> CourseAdapter {
        CourseAdapter::new()
    }
}

impl CourseService for CourseAdapter {
    fn get_course(&self, coursekey: &CourseKey) -> Result<BTreeMap<UsageKey, Vec<UsageKey>>> {
        let has_token = self.access_token.borrow().is_some();
        if !has_token {
            self.access_token.replace(self.get_new_token().ok());
        }
        let params = {
            let mut params = BTreeMap::new();
            params.insert("course_id", format!("{}", coursekey));
            params.insert("requested_fields", "children".into());
            params.insert("all_blocks", "true".into());
            params.insert("depth", "10".into());
            params
        };
        let response = self.client
            .get(&format!(
                "{}blocks/?course_id={}&requested_fields=children&all_blocks=true&depth=10",
                self.api_root_url, coursekey,
            ))
            .bearer_auth(self.access_token.borrow().to_owned().unwrap())
            .query(&params)
            .send()
            .map_err(ServiceError::from_error)?;

        let data: serde_json::Value =
            serde_json::from_reader(response).map_err(ServiceError::from_error)?;
        let blocks = data["blocks"].as_object().unwrap();
        let mut output = BTreeMap::new();
        for (block, value) in blocks {
            let blockkey = UsageKey::new(coursekey.clone(), block.clone());
            if let Some(children) = value["children"].as_array() {
                let children = children
                    .into_iter()
                    .map(|child| UsageKey::new(coursekey.clone(), child.as_str().unwrap().into()))
                    .collect();
                output.insert(blockkey, children);
            } else {
                output.insert(blockkey, Vec::new());
            }
        }
        Ok(output)
    }
}
