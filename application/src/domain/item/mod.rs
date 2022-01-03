extern crate openapi;

use actix_web::web;
pub use openapi::models::item::Item;

use controller::{get_item_by_id, get_items, put_item};

mod controller;
mod repository;
mod service;

pub fn item_service_config(cfg: &mut web::ServiceConfig) {
    cfg
        // .app_data(item_repository_module.clone())
        .service(get_items)
        .service(put_item)
        .service(get_item_by_id);
}
