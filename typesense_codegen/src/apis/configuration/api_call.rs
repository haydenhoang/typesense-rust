use reqwest::{Error, IntoUrl, Method, Request, RequestBuilder, Response};
use std::{
    time::{Duration, SystemTime, UNIX_EPOCH},
    vec,
};
use tokio::time::sleep;
use url::Url;

#[derive(Debug, Clone)]
pub struct Node {
    pub url: Url,
    is_healthy: bool,
    last_access_timestamp: u128,
}
#[derive(Debug, Clone)]
pub struct APICall {
    pub nearest_node: Option<Node>,
    pub nodes: Vec<Node>,
    current_node_index: isize,
    client: reqwest::Client,
    pub num_retries: usize,
    pub health_check_interval: Duration,
    pub retry_interval: Duration,
}

impl APICall {
    pub fn new(nodes: Vec<&str>) -> Self {
        Self {
            nodes: nodes
                .iter()
                .map(|url| Node {
                    url: Url::parse(url).unwrap(),
                    is_healthy: true,
                    last_access_timestamp: get_unix_mili_now(),
                })
                .collect(),
            ..Default::default()
        }
    }

    pub fn health_check_interval(&mut self, duration: Duration) {
        self.health_check_interval = duration;
    }

    pub fn retry_interval(&mut self, duration: Duration) {
        self.retry_interval = duration;
    }

    pub fn num_retries(&mut self, num_retries: usize) {
        self.num_retries = num_retries;
    }

    pub fn nearest_node(&mut self, nearest_node: &str) {
        self.nearest_node = Some(Node {
            url: Url::parse(nearest_node).unwrap(),
            is_healthy: true,
            last_access_timestamp: get_unix_mili_now(),
        });
    }
}

impl Default for APICall {
    fn default() -> Self {
        APICall {
            current_node_index: -1,
            client: reqwest::Client::new(),
            nodes: vec![],
            num_retries: 5,
            nearest_node: None,
            health_check_interval: Duration::from_secs(60),
            retry_interval: Duration::from_millis(100),
        }
    }
}

impl APICall {
    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        self.client.request(method, url)
    }

    fn clone_request(&mut self, request: &Request) -> (Request, Node, bool) {
        let (mut node, is_nearest_node) = self.get_next_node();
        let mut my_req = request
            .try_clone()
            .expect("Cannot clone the reqwest Request!");
        node.url.set_path(my_req.url().path());
        *my_req.url_mut() = node.url.clone();
        println!("{}", my_req.url());
        return (my_req, node, is_nearest_node);
    }

    fn update_node_health(&mut self, node: Node, is_nearest_node: bool, is_healthy: bool) {
        if is_nearest_node {
            self.nearest_node = Some(set_node_health(node, is_healthy));
        } else {
            self.nodes[self.current_node_index as usize] = set_node_health(node, is_healthy);
        }
    }

    fn handle_result(
        &mut self,
        result: &mut Result<Response, Error>,
        node: Node,
        is_nearest_node: bool,
    ) -> bool {
        match result {
            Ok(response) => {
                if response.status().is_server_error() {
                    self.update_node_health(node, is_nearest_node, false);
                    return false;
                } else {
                    self.update_node_health(node, is_nearest_node, true);
                    return true;
                }
            }
            Err(err) => {
                if err.is_timeout() {
                    self.update_node_health(node, is_nearest_node, false);
                }
                return false;
            }
        }
    }

    pub async fn execute(&mut self, request: Request) -> Result<Response, Error> {
        let (modified_request, node, is_nearest_node) = self.clone_request(&request);
        let mut result = self.client.execute(modified_request).await;

        if self.num_retries == 0 {
            return result;
        }

        let is_success = self.handle_result(&mut result, node, is_nearest_node);

        if is_success || self.num_retries == 0 {
            return result;
        }

        sleep(self.retry_interval).await;

        for _ in 0..self.num_retries {
            let (modified_request, node, is_nearest_node) = self.clone_request(&request);
            result = self.client.execute(modified_request).await;

            let is_success = self.handle_result(&mut result, node, is_nearest_node);
            if is_success {
                return result;
            }
            sleep(self.retry_interval).await;
        }
        return result;
    }

    fn get_next_node(&mut self) -> (Node, bool) {
        if let Some(nearest_node) = self.nearest_node.to_owned() {
            if nearest_node.is_healthy || self.node_due_for_health_check(&nearest_node) {
                return (nearest_node, true);
            }
        }
        let mut candidate_node = self.nodes[0].to_owned();
        let num_nodes = self.nodes.len() as isize;
        for _ in 0..num_nodes {
            self.current_node_index = (self.current_node_index + 1) % num_nodes;
            candidate_node = self.nodes[self.current_node_index as usize].to_owned();
            println!(
                "Node:{} Healthy: {} Due: {}",
                candidate_node.url,
                candidate_node.is_healthy,
                self.node_due_for_health_check(&candidate_node)
            );
            if candidate_node.is_healthy || self.node_due_for_health_check(&candidate_node) {
                return (candidate_node, false);
            } else {
                continue;
            }
        }
        return (candidate_node, false);
    }

    fn node_due_for_health_check(&self, node: &Node) -> bool {
        println!(
            "Now: {} Last: {} Interval:{}",
            get_unix_mili_now(),
            node.last_access_timestamp,
            self.health_check_interval.as_millis()
        );
        get_unix_mili_now() - node.last_access_timestamp >= self.health_check_interval.as_millis()
    }
}

fn set_node_health(mut node: Node, is_healthy: bool) -> Node {
    node.is_healthy = is_healthy;
    node.last_access_timestamp = get_unix_mili_now();
    node
}

type GetUnixMiliFn = fn() -> u128;

fn default_get_unix_mili() -> u128 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis(),
        Err(_) => panic!("System time before UNIX epoch!"),
    }
}

// for mocking in tests
static mut TIME_FN: GetUnixMiliFn = default_get_unix_mili;

fn get_unix_mili_now() -> u128 {
    unsafe { TIME_FN() }
}
#[cfg(test)]
mod test {
    use super::{get_unix_mili_now, TIME_FN};
    use std::time::Duration;

    use httpmock::{prelude::*, Mock, Then, When};
    use serial_test::serial;

    use crate::{
        apis::{
            collections_api::create_collection,
            configuration::{api_call::default_get_unix_mili, Configuration},
        },
        models::{CollectionSchema, Field},
    };

    struct TestTearDown;
    //https://stackoverflow.com/a/38254435
    impl Drop for TestTearDown {
        fn drop(&mut self) {
            // reset the fn to use time now
            unsafe {
                TIME_FN = default_get_unix_mili;
            }
        }
    }

    #[tokio::test]
    #[serial] // run test sequentially

    async fn api_call_do_not_retry_status_4xx() {
        let handlers: Vec<fn(When, Then)> = vec![
            |when, then| {
                when.method(POST).path("/collections");
                then.status(400);
            },
            |when, then| {
                when.method(POST).path("/collections");
                then.status(200);
            },
        ];
        let (servers, base_urls) = spawn_servers(handlers.len());
        let mocks = start_mocks(&servers, handlers);

        let mut config: Configuration =
            Configuration::new("xyz", base_urls.iter().map(|url| url.as_str()).collect());

        create_typesense_collection(&mut config).await;

        mocks[0].assert();
        mocks[1].assert_calls(0);
    }

    #[tokio::test]
    #[serial]
    async fn api_call_retry_status_5xx() {
        let handlers: Vec<fn(When, Then)> = vec![
            |when, then| {
                when.method(POST).path("/collections");
                then.status(500);
            },
            |when, then| {
                when.method(POST).path("/collections");
                then.status(201);
            },
            |when, then| {
                when.method(POST).path("/collections");
                then.status(200);
            },
        ];
        let (servers, base_urls) = spawn_servers(handlers.len());
        let mocks = start_mocks(&servers, handlers);

        let mut config: Configuration =
            Configuration::new("xyz", base_urls.iter().map(|url| url.as_str()).collect())
                .retry_interval(Duration::from_secs(0))
                .build();

        create_typesense_collection(&mut config).await;

        mocks[0].assert();
        mocks[1].assert();
        mocks[2].assert_calls(0);
    }

    #[tokio::test]
    #[serial]
    async fn api_call_add_node_into_rotation() {
        let _ = TestTearDown;
        let handlers: Vec<fn(When, Then)> = vec![
            |when, then| {
                when.method(POST).path("/collections");
                then.status(500);
            },
            |when, then| {
                when.method(POST).path("/collections");
                then.status(201);
            },
            |when, then| {
                when.method(POST).path("/collections");
                then.status(200);
            },
        ];
        let (servers, base_urls) = spawn_servers(handlers.len());
        let mocks = start_mocks(&servers, handlers);

        unsafe {
            TIME_FN = || 1;
        }
        let mut config: Configuration =
            Configuration::new("xyz", base_urls.iter().map(|url| url.as_str()).collect())
                .health_check_interval(Duration::from_millis(50))
                .retry_interval(Duration::from_secs(0))
                .build();

        create_typesense_collection(&mut config).await; // node 1 fails, node 2 succeeds
        mocks[0].assert_calls(1);
        mocks[1].assert_calls(1);
        mocks[2].assert_calls(0);

        unsafe {
            TIME_FN = || 21;
        }
        create_typesense_collection(&mut config).await; // request will be made to node 3
        assert_eq!(mocks[0].calls(), 1);
        assert_eq!(mocks[1].calls(), 1);
        assert_eq!(mocks[2].calls(), 1);

        create_typesense_collection(&mut config).await; // request should be made to node 2 since node 1 is unhealthy
        assert_eq!(mocks[0].calls(), 1);
        assert_eq!(mocks[1].calls(), 2);
        assert_eq!(mocks[2].calls(), 1);

        unsafe {
            TIME_FN = || 51;
        }
        create_typesense_collection(&mut config).await; // request will be made to node 3
        assert_eq!(mocks[0].calls(), 1);
        assert_eq!(mocks[1].calls(), 2);
        assert_eq!(mocks[2].calls(), 2);

        create_typesense_collection(&mut config).await; // node 1 added back into rotation but fails, node 2 suceeeds
        assert_eq!(mocks[0].calls(), 2);
        assert_eq!(mocks[1].calls(), 3);
        assert_eq!(mocks[2].calls(), 2);
    }

    #[tokio::test]
    #[serial]
    async fn api_call_with_healthy_nearest_node() {
        let _ = TestTearDown;
        let handlers: Vec<fn(When, Then)> = vec![
            |when, then| {
                when.method(POST).path("/collections");
                then.status(200);
            },
            |when, then| {
                when.method(POST).path("/collections");
                then.status(200);
            },
        ];
        let (servers, base_urls) = spawn_servers(handlers.len());
        let mocks = start_mocks(&servers, handlers);

        let mut config: Configuration = Configuration::new(
            "xyz",
            base_urls[1..].iter().map(|url| url.as_str()).collect(),
        )
        .health_check_interval(Duration::from_millis(50))
        .nearest_node(base_urls[0].as_str())
        .retry_interval(Duration::from_secs(0))
        .build();

        create_typesense_collection(&mut config).await;
        mocks[0].assert_calls(1);
        mocks[1].assert_calls(0);

        create_typesense_collection(&mut config).await; // request should still be made to nearest node
        mocks[0].assert_calls(2);
        mocks[1].assert_calls(0);
    }

    #[tokio::test]
    #[serial]
    async fn api_call_with_unhealthy_nearest_node() {
        let _ = TestTearDown;
        let handlers: Vec<fn(When, Then)> = vec![
            |when, then| {
                when.method(POST).path("/collections");
                then.status(500);
            },
            |when, then| {
                when.method(POST).path("/collections");
                then.status(501);
            },
            |when, then| {
                when.method(POST).path("/collections");
                then.status(200);
            },
        ];
        let (servers, base_urls) = spawn_servers(handlers.len());
        let mocks = start_mocks(&servers, handlers);

        unsafe {
            TIME_FN = || 1;
        }
        let mut config: Configuration = Configuration::new(
            "xyz",
            base_urls[1..].iter().map(|url| url.as_str()).collect(),
        )
        .health_check_interval(Duration::from_millis(50))
        .nearest_node(base_urls[0].as_str())
        .retry_interval(Duration::from_secs(0))
        .build();

        create_typesense_collection(&mut config).await; // nearest node fails, node 2 succeeds
        assert_eq!(mocks[0].calls(), 1);
        assert_eq!(mocks[1].calls(), 1);
        assert_eq!(mocks[2].calls(), 1);

        unsafe {
            TIME_FN = || 21;
        }

        create_typesense_collection(&mut config).await; // request should be made to node 2 since node 1 is unhealthy
        assert_eq!(mocks[0].calls(), 1);
        assert_eq!(mocks[1].calls(), 1);
        assert_eq!(mocks[2].calls(), 2);

        unsafe {
            TIME_FN = || 51;
        }
        create_typesense_collection(&mut config).await; // request will be made to nearest node
        assert_eq!(mocks[0].calls(), 2);
        assert_eq!(mocks[1].calls(), 2);
        assert_eq!(mocks[2].calls(), 3);

        unsafe {
            TIME_FN = || 101;
        }
        create_typesense_collection(&mut config).await; // nearest node added back into rotation but fails, node 2 suceeeds
        assert_eq!(mocks[0].calls(), 3);
        assert_eq!(mocks[1].calls(), 3);
        assert_eq!(mocks[2].calls(), 4);

        // reset the fn to use time now
        unsafe {
            TIME_FN = default_get_unix_mili;
        }
    }

    #[test]
    #[serial]
    fn test_mock_get_unix_mili_now() {
        let _ = TestTearDown;
        assert!(get_unix_mili_now() > 1737131176403);
        unsafe {
            TIME_FN = || 1;
        }
        assert!(get_unix_mili_now() == 1);
        unsafe {
            TIME_FN = || 10;
        }
        assert!(get_unix_mili_now() == 10);
    }

    fn spawn_servers(num_servers: usize) -> (Vec<MockServer>, Vec<String>) {
        let mut servers = Vec::new();
        let mut base_urls = Vec::new();
        for _ in 0..num_servers {
            let server = MockServer::start();
            base_urls.push(server.base_url());
            servers.push(server);
        }
        return (servers, base_urls);
    }
    async fn create_typesense_collection(config: &mut Configuration) {
        let _ = create_collection(
            config,
            CollectionSchema::new(
                "test-collection".to_owned(),
                vec![Field::new("num-employees".to_owned(), "int".to_owned())],
            ),
        )
        .await;
    }
    fn start_mocks<'a, F>(servers: &'a Vec<MockServer>, handlers: Vec<F>) -> Vec<Mock<'a>>
    where
        F: FnOnce(When, Then),
    {
        let mut mocks = Vec::new();
        for (i, handler) in handlers.into_iter().enumerate() {
            mocks.push(servers[i].mock(handler));
        }
        mocks
    }
}
