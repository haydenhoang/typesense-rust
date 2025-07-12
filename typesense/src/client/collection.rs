mod documents;

use documents::Documents;
use typesense_codegen::{
    apis::{
        collections_api::{self, DeleteCollectionError, GetCollectionError},
        configuration::Configuration,
        Error,
    },
    models::CollectionResponse,
};

pub struct Collection<'a> {
    pub configuration: &'a mut Configuration,
    pub collection_name: String,
}

impl<'a> Collection<'a> {
    pub async fn retrieve(&mut self) -> Result<CollectionResponse, Error<GetCollectionError>> {
        collections_api::get_collection(self.configuration, &self.collection_name).await
    }

    pub async fn delete(&mut self) -> Result<CollectionResponse, Error<DeleteCollectionError>> {
        collections_api::delete_collection(self.configuration, &self.collection_name).await
    }

    pub fn documents(&mut self) -> Documents {
        Documents {
            configuration: &mut self.configuration,
            collection_name: self.collection_name.to_owned(),
        }
    }
}
