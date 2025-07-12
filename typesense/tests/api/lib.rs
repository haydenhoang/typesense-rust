mod collection;
mod documents;
mod test_utils;

use typesense::client::TypesenseClient;
use typesense_codegen::apis::configuration::Configuration;

#[cfg(not(target_arch = "wasm32"))]
fn new_typesense_client() -> TypesenseClient {
    let _ = dotenvy::dotenv();

    let base_path = std::env::var("URL").expect("URL must be present in .env");
    let api_key = std::env::var("API_KEY").expect("API_KEY must be present in .env");

    TypesenseClient::new(
        Configuration::new(api_key, vec![base_path.as_str()])
            .num_retries(0)
            .build(),
    )
}

#[cfg(target_arch = "wasm32")]
fn init() -> Configuration {
    let base_path = "http://localhost:5000".to_owned();
    let key = "VerySecretKey".to_owned();

    Configuration {
        base_path,
        api_key: Some(ApiKey { prefix: None, key }),
        ..Default::default()
    }
}
