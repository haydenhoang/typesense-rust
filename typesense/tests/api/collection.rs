#![allow(dead_code)]

use crate::test_utils::{create_new_collection, new_collection_schema};

use super::new_typesense_client;

async fn create_collection() {
    let expected_data = new_collection_schema("companies");
    let res = new_typesense_client()
        .collections()
        .create(expected_data.clone())
        .await
        .unwrap();
    assert_eq!(expected_data.name, res.name);
    assert_eq!(expected_data.fields.len(), res.fields.len());
}

async fn get_collection() {
    let expected = create_new_collection("companies").await;
    let res = new_typesense_client()
        .collection(&expected.name)
        .retrieve()
        .await
        .unwrap();

    assert_eq!(res.name, expected.name);
    assert_eq!(res.fields, expected.fields);
}

async fn delete_collection() {
    let expected = create_new_collection("companies").await;
    let res = new_typesense_client()
        .collection(&expected.name)
        .delete()
        .await
        .unwrap();

    assert_eq!(res.name, expected.name);
    assert_eq!(res.fields, expected.fields);
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

    #[tokio::test]
    async fn get_collection_tokio() {
        get_collection().await
    }

    #[tokio::test]
    async fn delete_collection_tokio() {
        delete_collection().await
    }

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
