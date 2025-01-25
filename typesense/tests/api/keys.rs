#![allow(dead_code)]

// use super::Config;
// use typesense::new_client;

// async fn test_generate_scoped_search_key() {
//     let client = new_client();
// }

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
    async fn get_collections_tokio() {
        get_collections().await
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
