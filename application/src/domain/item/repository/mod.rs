use openapi::models::item::Item;
use shaku::{module, HasComponent, Interface};
use uuid::Uuid;

use tokio_item_repository::TokioItemRepository;

mod item_repository;
mod tokio_item_repository;

pub trait ItemRepositoryModule: HasComponent<dyn ItemRepository> {}

module! {
    pub ItemRepositoryModuleImpl : ItemRepositoryModule {
        components = [TokioItemRepository],
        providers = []
    }
}

pub trait ItemRepository: Interface {
    fn find_all(&self) -> Vec<Item>;

    fn save(&self, item: &Item) -> Item;

    fn find_by_id(&self, item_id: Uuid) -> Option<Item>;
}
