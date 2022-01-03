use async_trait::async_trait;
use openapi::models::item::Item;
use uuid::Uuid;

pub mod item_service;

#[async_trait]
pub trait ItemService {
    async fn find_all(&self) -> Vec<Item>;

    async fn save(&self, item: &Item) -> Item;

    async fn find_by_id(&self, item_id: Uuid) -> Item;
}
