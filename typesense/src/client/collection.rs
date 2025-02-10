use typesense_codegen::{
    apis::{
        collections_api::{self, DeleteCollectionError, GetCollectionError},
        configuration::Configuration,
        Error,
    },
    models::CollectionResponse,
};

pub struct Collection<'a, 'b> {
    pub configuration: &'a mut Configuration,
    pub name: &'b str,
}

impl<'a, 'b> Collection<'a, 'b> {
    pub async fn retrieve(&mut self) -> Result<CollectionResponse, Error<GetCollectionError>> {
        collections_api::get_collection(self.configuration, &self.name).await
    }

    pub async fn delete(&mut self) -> Result<CollectionResponse, Error<DeleteCollectionError>> {
        collections_api::delete_collection(self.configuration, &self.name).await
    }
}
