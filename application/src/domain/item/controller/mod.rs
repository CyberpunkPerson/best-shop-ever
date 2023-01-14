use actix_web::{get, put, web::Json, web::Path, Responder};
use openapi::models::{item::Item, GetItemResponse, GetItemsResponse, PutItemResponse};
use uuid::Uuid;

use crate::domain::item::service::item_service::DefaultItemService;
use crate::domain::item::service::ItemService;

#[put("/items")]
pub async fn put_item(item: Json<Item>) -> impl Responder {
    let service = DefaultItemService::default();
    PutItemResponse {
        payload: Box::new(service.save(&item)),
    }
}

#[get("/items")]
pub async fn get_items() -> impl Responder {
    let service = DefaultItemService::default();
    GetItemsResponse {
        items: service.find_all(),
    }
}

#[get("/items/{item_id}")]
pub async fn get_item_by_id(Path((item_id,)): Path<(Uuid,)>) -> impl Responder {
    let service = DefaultItemService::default();
    GetItemResponse {
        payload: Box::new(service.find_by_id(item_id)),
    }
}
