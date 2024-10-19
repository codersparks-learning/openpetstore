use async_trait::async_trait;
use axum::extract::Host;
use axum_extra::extract::CookieJar;
use http::Method;
use petstoreapi::apis::user::{CreateUserResponse, CreateUsersWithListInputResponse, DeleteUserResponse, GetUserByNameResponse, LoginUserResponse, LogoutUserResponse, UpdateUserResponse};
use petstoreapi::models::{DeleteUserPathParams, GetUserByNamePathParams, LoginUserQueryParams, UpdateUserPathParams, User};
use crate::api::ServerImpl;

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