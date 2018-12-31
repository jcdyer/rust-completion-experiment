use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::{self, Write};

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
        let resp = self.client.post(&self.oauth_token_url)
            .form(&form)
            .send()
            .map_err(ServiceError::from_error)?;
        resp.status();
        let data: serde_json::Value = serde_json::from_reader(resp).map_err(ServiceError::from_error)?;
        Ok(data["access_token"].as_str().ok_or_else(|| ServiceError::NotFound)?.to_owned())
    }
}

impl CourseService for CourseAdapter {
    fn get_course(&self, coursekey: &CourseKey) -> Result<BTreeMap<UsageKey, Vec<UsageKey>>> {
        let has_token = self.access_token.borrow().is_some();
        if !has_token {
            self.access_token.replace(self.get_new_token().ok());
        }
        let mut response = self.client.get(&format!(
            "{}blocks/?course_id={}&requested_fields=children&all_blocks=true&depth=10",
            self.api_root_url,
            coursekey,
        )).bearer_auth(self.access_token.borrow().to_owned().unwrap()).send().map_err(ServiceError::from_error)?;
        std::io::copy(&mut response, &mut std::io::stdout()).map_err(ServiceError::from_error)?;
        Ok(BTreeMap::new())
    }
}
