/*
 * Typesense API
 *
 * This generated file is modified. Do not overwrite this file.
 */

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: APICall,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl Configuration {
    pub fn new(api_key: impl Into<String>, nodes: Vec<&str>) -> Self {
        if nodes.len() == 0 {
            panic!("Nodes must not be empty!")
        }
        Self {
            base_path: nodes[0].to_string(),
            api_key: Some(ApiKey {
                prefix: None,
                key: api_key.into(),
            }),
            client: APICall::new(nodes),
            ..Default::default()
        }
    }

    pub fn health_check_interval(mut self, duration: Duration) -> Self {
        self.client.health_check_interval(duration);
        self
    }

    pub fn num_retries(mut self, num_retries: usize) -> Self {
        self.client.num_retries(num_retries);
        self
    }

    pub fn nearest_node(mut self, nearest_node: &str) -> Self {
        self.client.nearest_node(nearest_node);
        self
    }

    pub fn build(self) -> Configuration {
        Configuration {
            base_path: self.base_path,
            api_key: self.api_key,
            client: self.client,
            ..Default::default()
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "http://localhost:8108".to_owned(),
            user_agent: Some("OpenAPI-Generator/27.0/rust".to_owned()),
            client: APICall::new(vec!["http://localhost:8108"]),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}

use std::time::Duration;

use api_call::APICall;
mod api_call;
