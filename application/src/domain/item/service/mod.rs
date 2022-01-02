use super::repository::{ItemRepository, ItemRepositoryModule};
use item_service::DefaultItemService;
use openapi::models::item::Item;
use shaku::{module, Interface};
use uuid::Uuid;

mod item_service;

pub trait ItemService: Interface {
    fn find_all(&self) -> Vec<Item>;

    fn save(&self, item: &Item) -> Item;

    fn find_by_id(&self, item_id: Uuid) -> Item;
}

module! {
    pub ItemServiceModule {
        components = [DefaultItemService],
        providers = [],

        use ItemRepositoryModule {
            components = [ItemRepository],
            providers = []
        }
    }
}
