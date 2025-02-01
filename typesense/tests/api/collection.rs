#![allow(dead_code)]

use super::new_typesense_client;
use serde::{Deserialize, Serialize};
use typesense::document::Document;
use typesense::Typesense;
use typesense_codegen::models::{CollectionResponse, CollectionSchema};

#[derive(Typesense, Serialize, Deserialize)]
#[typesense(collection_name = "companies", default_sorting_field = "num_employees")]
struct Company {
    company_name: String,
    num_employees: i32,
    #[typesense(facet)]
    country: String,
}

async fn create_collection() {
    let mut client = new_typesense_client();
    let expected_data = Company::collection_schema();
    let res = client
        .collections()
        .create(expected_data.clone())
        .await
        .unwrap();
    assert_eq!(expected_data.name, res.name);
    assert_eq!(expected_data.fields.len(), res.fields.len());
}

// async fn get_collection() {
//     let res = collections_api::get_collection(&mut Config::get(), "companies")
//         .await
//         .unwrap();

//     assert_eq!(res.num_documents, 1250);
//     assert_eq!(schema_to_resp(Company::collection_schema(), &res), res);
// }

async fn delete_collection() {
    // let collection_schema_response =
    //     collections_api::delete_collection(&mut Config::get(), "companies")
    //         .await
    //         .unwrap();

    // assert_eq!(collection_schema_response.num_documents, 1200);
    // assert_eq!(
    //     schema_to_resp(Company::collection_schema(), &collection_schema_response),
    //     collection_schema_response
    // );
}

async fn get_all_collections() {
    let _ = new_typesense_client()
        .collections()
        .retrieve()
        .await
        .unwrap();
}

#[cfg(all(feature = "tokio_test", not(target_arch = "wasm32")))]
mod tokio_test {
    use super::*;

    #[tokio::test]
    async fn create_collection_tokio() {
        create_collection().await
    }

    // #[tokio::test]
    // async fn get_collection_tokio() {
    //     get_collection().await
    // }

    // #[tokio::test]
    // async fn delete_collection_tokio() {
    //     delete_collection().await
    // }

    #[tokio::test]
    async fn get_all_collections_tokio() {
        get_all_collections().await
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn create_collection_wasm() {
        console_error_panic_hook::set_once();

        create_collection().await
    }

    #[wasm_bindgen_test]
    async fn get_collection_wasm() {
        console_error_panic_hook::set_once();

        get_collection().await
    }

    #[wasm_bindgen_test]
    async fn delete_collection_wasm() {
        console_error_panic_hook::set_once();

        delete_collection().await
    }

    #[wasm_bindgen_test]
    async fn get_collections_wasm() {
        console_error_panic_hook::set_once();

        get_collections().await
    }
}
