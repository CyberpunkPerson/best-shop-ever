use async_trait::async_trait;
use openapi::models::item::Item;
use uuid::Uuid;

mod stub_item_repository;
pub mod tokio_item_repository;

#[async_trait]
pub trait ItemRepository {
    async fn find_all(&self) -> Vec<Item>;

    async fn save(&self, item: &Item) -> Item;

    async fn find_by_id(&self, item_id: Uuid) -> Option<Item>;
}
