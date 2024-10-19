use async_trait::async_trait;
use axum::extract::Host;
use axum_extra::extract::CookieJar;
use http::Method;
use petstoreapi::apis::store::{DeleteOrderResponse, GetInventoryResponse, GetOrderByIdResponse, PlaceOrderResponse};
use petstoreapi::models::{DeleteOrderPathParams, GetOrderByIdPathParams, Order};
use crate::api::ServerImpl;

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