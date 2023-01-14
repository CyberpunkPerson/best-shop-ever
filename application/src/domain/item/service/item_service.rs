use std::sync::Arc;
use std::vec::Vec;

use openapi::models::Item;
use uuid::Uuid;

use crate::domain::item::repository::stub_item_repository::StubItemRepository;
use crate::domain::item::repository::ItemRepository;

use super::ItemService;

pub struct DefaultItemService {
    repository: Arc<dyn ItemRepository + Send + Sync>,
}

impl ItemService for DefaultItemService {
    fn find_all(&self) -> Vec<Item> {
        self.repository.find_all()
    }

    fn save(&self, item: &Item) -> Item {
        self.repository.save(item)
    }

    fn find_by_id(&self, item_id: Uuid) -> Item {
        self.repository.find_by_id(item_id).unwrap()
    }
}

impl Default for DefaultItemService {
    fn default() -> Self {
        DefaultItemService {
            repository: Arc::new(StubItemRepository {}),
        }
    }
}
