use typesense_codegen::{
    apis::{
        collections_api::{self, CreateCollectionError, GetCollectionsError},
        configuration::Configuration,
        Error,
    },
    models::{CollectionResponse, CollectionSchema},
};

pub struct Collections<'a> {
    pub configuration: &'a mut Configuration,
}

impl<'a> Collections<'a> {
    pub async fn create(
        &mut self,
        collection_schema: CollectionSchema,
    ) -> Result<CollectionResponse, Error<CreateCollectionError>> {
        collections_api::create_collection(self.configuration, collection_schema).await
    }

    pub async fn retrieve(
        &mut self,
    ) -> Result<Vec<CollectionResponse>, Error<GetCollectionsError>> {
        collections_api::get_collections(self.configuration).await
    }
}
