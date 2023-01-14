use openapi::models::item::Item;
use uuid::Uuid;

pub mod item_service;

pub trait ItemService {
    fn find_all(&self) -> Vec<Item>;

    fn save(&self, item: &Item) -> Item;

    fn find_by_id(&self, item_id: Uuid) -> Item;
}
