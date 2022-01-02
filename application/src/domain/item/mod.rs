use actix_web::web;
use controller::{get_item_by_id, get_items, put_item};
use service::ItemServiceModule;
use repository::ItemRepositoryModuleImpl;
use std::sync::Arc;

extern crate openapi;
pub use openapi::models::item::Item;

mod controller;
mod service;
mod repository;

pub fn item_service_config(cfg: &mut web::ServiceConfig) {
    let item_repository_module = Arc::new(ItemRepositoryModuleImpl::builder().build());

    let item_service_module = Arc::new(ItemServiceModule::builder(item_repository_module).build());
    cfg.app_data(item_service_module.clone())
        // .app_data(item_repository_module.clone())
        .service(get_items)
        .service(put_item)
        .service(get_item_by_id);
}
