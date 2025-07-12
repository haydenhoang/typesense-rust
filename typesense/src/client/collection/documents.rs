use typesense_codegen::{
    apis::{
        configuration::Configuration,
        documents_api::{self, ImportDocumentsError, ImportDocumentsParams},
        Error,
    },
    models::SearchParameters,
};

pub struct Documents<'a> {
    pub configuration: &'a mut Configuration,
    pub collection_name: String,
}

impl<'a> Documents<'a> {
    pub async fn import(&mut self, params: ImportDocumentsParams) -> Result<String, Error<ImportDocumentsError>> {
        documents_api::import_documents(&mut self.configuration, params).await
    }

    // pub async fn search<T: for<'d> serde::Deserialize<'d>>(
    //     &mut self,
    //     search_parameters: SearchParameters,
    // ) -> Result<crate::models::SearchResult<T>, Error<SearchCollectionError>> {
    //     documents_api::search_collection(
    //         self.configuration,
    //         &self.collection_name,
    //         search_parameters,
    //     )
    //     .await
    // }
}
