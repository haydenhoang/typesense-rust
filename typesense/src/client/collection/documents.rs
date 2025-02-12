use typesense_codegen::{
    apis::{
        configuration::Configuration,
        documents_api::{self, ImportDocumentsError, SearchCollectionError},
        Error,
    },
    models::{ImportDocumentsImportDocumentsParametersParameter, SearchParameters},
};

pub struct Documents<'a, 'b> {
    pub configuration: &'a mut Configuration,
    pub collection_name: &'b str,
}

impl<'a, 'b> Documents<'a, 'b> {
    pub async fn import(
        &mut self,
        documents: String,
        options: Option<ImportDocumentsImportDocumentsParametersParameter>,
    ) -> Result<String, Error<ImportDocumentsError>> {
        documents_api::import_documents(
            &mut self.configuration,
            &self.collection_name,
            documents,
            options,
        )
        .await
    }

    pub async fn search<T: for<'d> serde::Deserialize<'d>>(
        &mut self,
        search_parameters: SearchParameters,
    ) -> Result<crate::models::SearchResult<T>, Error<SearchCollectionError>> {
        documents_api::search_collection(
            self.configuration,
            &self.collection_name,
            search_parameters,
        )
        .await
    }
}
