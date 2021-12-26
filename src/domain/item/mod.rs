use actix_web::web;
use controller::{get_item_by_id, get_items, put_item};
use service::ItemServiceModule;
use std::sync::Arc;

extern crate openapi;
pub use openapi::models::item::Item;

mod controller;
mod service;

pub fn item_service_config(cfg: &mut web::ServiceConfig) {
    let module = Arc::new(ItemServiceModule::builder().build());
    cfg.app_data(module.clone())
        .service(get_items)
        .service(put_item)
        .service(get_item_by_id);
}
