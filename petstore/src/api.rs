use async_trait::async_trait;
use axum::extract::{Host};
use axum_extra::extract::cookie::CookieJar;
use bytes::Bytes;
use http::Method;
use petstoreapi::apis::pet::{AddPetResponse, DeletePetResponse, FindPetsByStatusResponse, FindPetsByTagsResponse, GetPetByIdResponse, UpdatePetResponse, UpdatePetWithFormResponse, UploadFileResponse};
use petstoreapi::apis::store::{DeleteOrderResponse, GetInventoryResponse, GetOrderByIdResponse, PlaceOrderResponse};
use petstoreapi::apis::user::{CreateUserResponse, CreateUsersWithListInputResponse, DeleteUserResponse, GetUserByNameResponse, LoginUserResponse, LogoutUserResponse, UpdateUserResponse};
use petstoreapi::models::{DeleteOrderPathParams, DeletePetHeaderParams, DeletePetPathParams, DeleteUserPathParams, FindPetsByStatusQueryParams, FindPetsByTagsQueryParams, GetOrderByIdPathParams, GetPetByIdPathParams, GetUserByNamePathParams, LoginUserQueryParams, Order, Pet, UpdatePetWithFormPathParams, UpdatePetWithFormQueryParams, UpdateUserPathParams, UploadFilePathParams, UploadFileQueryParams, User};

pub struct ServerImpl {}

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

#[allow(unused_variables)]
#[async_trait]
impl petstoreapi::apis::store::Store for ServerImpl {
    async fn delete_order(&self, method: Method, host: Host, cookies: CookieJar, path_params: DeleteOrderPathParams) -> Result<DeleteOrderResponse, String> {
        todo!()
    }

    async fn get_inventory(&self, method: Method, host: Host, cookies: CookieJar) -> Result<GetInventoryResponse, String> {
        todo!()
    }

    async fn get_order_by_id(&self, method: Method, host: Host, cookies: CookieJar, path_params: GetOrderByIdPathParams) -> Result<GetOrderByIdResponse, String> {
        todo!()
    }

    async fn place_order(&self, method: Method, host: Host, cookies: CookieJar, body: Option<Order>) -> Result<PlaceOrderResponse, String> {
        todo!()
    }
}

#[allow(unused_variables)]
#[async_trait]
impl petstoreapi::apis::user::User for ServerImpl {
    async fn create_user(&self, method: Method, host: Host, cookies: CookieJar, body: Option<User>) -> Result<CreateUserResponse, String> {
        todo!()
    }

    async fn create_users_with_list_input(&self, method: Method, host: Host, cookies: CookieJar, body: Option<Vec<User>>) -> Result<CreateUsersWithListInputResponse, String> {
        todo!()
    }

    async fn delete_user(&self, method: Method, host: Host, cookies: CookieJar, path_params: DeleteUserPathParams) -> Result<DeleteUserResponse, String> {
        todo!()
    }

    async fn get_user_by_name(&self, method: Method, host: Host, cookies: CookieJar, path_params: GetUserByNamePathParams) -> Result<GetUserByNameResponse, String> {
        todo!()
    }

    async fn login_user(&self, method: Method, host: Host, cookies: CookieJar, query_params: LoginUserQueryParams) -> Result<LoginUserResponse, String> {
        todo!()
    }

    async fn logout_user(&self, method: Method, host: Host, cookies: CookieJar) -> Result<LogoutUserResponse, String> {
        todo!()
    }

    async fn update_user(&self, method: Method, host: Host, cookies: CookieJar, path_params: UpdateUserPathParams, body: Option<User>) -> Result<UpdateUserResponse, String> {
        todo!()
    }
}