//! # Collection
//!
//! In Typesense, a group of related documents is called a collection. A collection
//! is roughly equivalent to a table in a relational database.
//!
//!
mod collection;
mod collections;

use collection::Collection;
use collections::Collections;
use typesense_codegen::apis::configuration::Configuration;

/// Typesense is
pub struct TypesenseClient {
    configuration: Configuration,
}

impl TypesenseClient {
    /// Typesense Collections API
    pub fn collections(&mut self) -> Collections {
        Collections {
            configuration: &mut self.configuration,
        }
    }

    /// Typesense Collection API
    pub fn collection(&mut self, name: impl Into<String>) -> Collection {
        Collection {
            configuration: &mut self.configuration,
            collection_name: name.into(),
        }
    }
}

impl TypesenseClient {
    /// Create a new [TypesenseClient]
    pub fn new(configuration: Configuration) -> TypesenseClient {
        TypesenseClient { configuration }
    }
}

#[cfg(test)]
mod test {
    use crate::client::TypesenseClient;
    use std::time::Duration;
    use typesense_codegen::apis::configuration::Configuration;

    #[test]
    fn initialize_typesense_client() {
        let configuration = Configuration::new("xyz", vec!["http://localhost:123"])
            .nearest_node("http://nearestnode:123")
            .health_check_interval(Duration::from_secs(60))
            .num_retries(5)
            .build();
        let client = TypesenseClient::new(configuration);

        assert_eq!(
            client
                .configuration
                .client
                .nearest_node
                .unwrap()
                .url
                .as_str(),
            "http://nearestnode:123/"
        );
        assert_eq!(
            client.configuration.client.health_check_interval,
            Duration::from_secs(60)
        );
        assert_eq!(client.configuration.client.num_retries, 5);
    }
    #[test]
    #[should_panic]
    fn initialize_typesense_client_panic_when_no_node_specified() {
        Configuration::new("xyz", vec![])
            .nearest_node("http://nearestnode:123")
            .health_check_interval(Duration::from_secs(60))
            .num_retries(5)
            .build();
    }
}
