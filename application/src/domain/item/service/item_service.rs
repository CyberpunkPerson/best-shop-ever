use super::super::repository::ItemRepository;
use super::ItemService;
use openapi::models::Item;
use shaku::Component;
use std::sync::Arc;
use std::vec::Vec;
use uuid::Uuid;

#[derive(Component)]
#[shaku(interface = ItemService)]
pub struct DefaultItemService {
    #[shaku(inject)]
    repository: Arc<dyn ItemRepository>,
}

impl ItemService for DefaultItemService {
    fn find_all(&self) -> Vec<Item> {
        self.repository.find_all()
    }
    fn save(&self, item: &Item) -> Item {
        self.repository.save(item)
    }
    fn find_by_id(&self, item_id: Uuid) -> Item {
        match self.repository.find_by_id(item_id) {
            Some(item) => item,
            None => panic!("Item by id {} not found!", item_id),
        }
    }
}
