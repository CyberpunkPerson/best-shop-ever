use openapi::models::item::Item;
use uuid::Uuid;

pub mod stub_item_repository;
// pub mod tokio_item_repository;

pub trait ItemRepository {
    fn find_all(&self) -> Vec<Item>;

    fn save(&self, item: &Item) -> Item;

    fn find_by_id(&self, item_id: Uuid) -> Option<Item>;
}
