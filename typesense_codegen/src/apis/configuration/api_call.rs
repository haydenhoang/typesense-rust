use reqwest::{Error, IntoUrl, Method, Request, RequestBuilder, Response};
use std::{
    time::{Duration, SystemTime, UNIX_EPOCH},
    vec,
};
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
            health_check_interval: Duration::from_secs(5),
        }
    }
}

impl APICall {
    pub fn request<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        self.client.request(method, url)
    }
    pub async fn execute(&mut self, request: Request) -> Result<Response, Error> {
        for _ in 0..self.num_retries {
            let (node, is_nearest_node) = self.get_next_node();
            let my_req: Option<Request> = request.try_clone();

            if let Some(mut req) = my_req {
                *req.url_mut() = node.url.clone();
                let result = self.client.execute(req).await;
                match result {
                    Ok(_) => return result,
                    Err(_) => {
                        if is_nearest_node {
                            self.nearest_node = Some(set_node_health(node, false))
                        } else {
                            self.nodes[self.current_node_index as usize] =
                                set_node_health(node, false)
                        }
                    }
                }
            }
        }
        panic!()
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

            if candidate_node.is_healthy || self.node_due_for_health_check(&candidate_node) {
                return (candidate_node, false);
            } else {
                continue;
            }
        }
        return (candidate_node, false);
    }

    fn node_due_for_health_check(&self, node: &Node) -> bool {
        get_unix_mili_now() - node.last_access_timestamp >= self.health_check_interval.as_millis()
    }
}

fn set_node_health(mut node: Node, is_healthy: bool) -> Node {
    node.is_healthy = is_healthy;
    node.last_access_timestamp = get_unix_mili_now();
    node
}

fn get_unix_mili_now() -> u128 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_millis(),
        Err(_) => panic!("System time before UNIX epoch!"),
    }
}

// let update it after cloning and reassign it in the vector
