use serde::{Deserialize, Serialize};
use typesense::{
    collection_schema::CollectionSchemaBuilder,
    field::FieldBuilder,
    models::{CollectionResponse, CollectionSchema},
};
use typesense_derive::Typesense;
use uuid::Uuid;

use crate::new_typesense_client;

#[derive(Typesense, Serialize, Deserialize)]
#[typesense(collection_name = "companies", default_sorting_field = "num_employees")]
pub struct Company {
    company_name: String,
    num_employees: i32,
    #[typesense(facet)]
    country: String,
}

fn new_uuid_name(name: &str) -> String {
    format!("{}-{}", name, Uuid::new_v4().to_string())
}

pub async fn create_new_collection(name: &str) -> CollectionResponse {
    let res = new_typesense_client()
        .collections()
        .create(new_collection_schema(name))
        .await
        .unwrap();
    res
}

pub fn new_collection_schema(name: &str) -> CollectionSchema {
    let fields = [
        ("company_name", "string".to_owned(), None),
        ("num_employees", "int32".to_owned(), None),
        ("country", "string".to_owned(), Some(true)),
    ]
    .map(|(name, typesense_type, facet)| {
        FieldBuilder::new(name, typesense_type).facet(facet).build()
    })
    .to_vec();

    return CollectionSchemaBuilder::new(&new_uuid_name(name), fields)
        .default_sorting_field("num_employees".to_owned())
        .build();
}
