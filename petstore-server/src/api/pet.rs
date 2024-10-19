use async_trait::async_trait;
use axum::extract::Host;
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use petstoreapi::apis::pet::{AddPetResponse, DeletePetResponse, FindPetsByStatusResponse, FindPetsByTagsResponse, GetPetByIdResponse, UpdatePetResponse, UpdatePetWithFormResponse, UploadFileResponse};
use petstoreapi::models::{DeletePetHeaderParams, DeletePetPathParams, FindPetsByStatusQueryParams, FindPetsByTagsQueryParams, GetPetByIdPathParams, Pet, UpdatePetWithFormPathParams, UpdatePetWithFormQueryParams, UploadFilePathParams, UploadFileQueryParams};
use crate::api::ServerImpl;

#[allow(unused_variables)]
#[async_trait]
impl petstoreapi::apis::pet::Pet for ServerImpl {
    async fn add_pet(&self, method: http::method::Method, host:Host, cookies: CookieJar, body: Pet) -> Result<AddPetResponse, String> {
        todo!()
    }

    async fn delete_pet(&self, method: http::method::Method, host: Host, cookies: CookieJar, header_params: DeletePetHeaderParams, path_params: DeletePetPathParams) -> Result<DeletePetResponse, String> {
        todo!()
    }

    async fn find_pets_by_status(&self, method: http::method::Method, host: Host, cookies: CookieJar, query_params: FindPetsByStatusQueryParams) -> Result<FindPetsByStatusResponse, String> {
        todo!()
    }

    async fn find_pets_by_tags(&self, method: http::method::Method, host: Host, cookies: CookieJar, query_params: FindPetsByTagsQueryParams) -> Result<FindPetsByTagsResponse, String> {
        todo!()
    }

    async fn get_pet_by_id(&self, method: http::method::Method, host: Host, cookies: CookieJar, path_params: GetPetByIdPathParams) -> Result<GetPetByIdResponse, String> {
        todo!()
    }

    async fn update_pet(&self, method: http::method::Method, host: Host, cookies: CookieJar, body: Pet) -> Result<UpdatePetResponse, String> {
        todo!()
    }

    async fn update_pet_with_form(&self, method: http::method::Method, host: Host, cookies: CookieJar, path_params: UpdatePetWithFormPathParams, query_params: UpdatePetWithFormQueryParams) -> Result<UpdatePetWithFormResponse, String> {
        todo!()
    }

    async fn upload_file(&self, method: http::method::Method, host: Host, cookies: CookieJar, path_params: UploadFilePathParams, query_params: UploadFileQueryParams, body: Bytes) -> Result<UploadFileResponse, String> {
        todo!()
    }
}