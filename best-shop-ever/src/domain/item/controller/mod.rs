use super::service::{ItemService, ItemServiceModule};
use actix_web::{get, put, web::Json, web::Path, Responder};
use openapi::models::{
    get_item_response::GetItemResponse, get_items_response::GetItemsResponse, item::Item,
    put_item_response::PutItemResponse,
};
use shaku_actix::Inject;
use uuid::Uuid;

#[put("/items")]
async fn put_item(
    item_service: Inject<ItemServiceModule, dyn ItemService>,
    item: Json<Item>,
) -> impl Responder {
    PutItemResponse {
        payload: Box::new(item_service.save(&item)),
    }
}

#[get("/items")]
async fn get_items(item_service: Inject<ItemServiceModule, dyn ItemService>) -> impl Responder {
    GetItemsResponse {
        items: item_service.find_all(),
    }
}

#[get("/items/{item_id}")]
async fn get_item_by_id(
    item_service: Inject<ItemServiceModule, dyn ItemService>,
    Path((item_id,)): Path<(Uuid,)>,
) -> impl Responder {
    GetItemResponse {
        payload: Box::new(item_service.find_by_id(item_id)),
    }
}
