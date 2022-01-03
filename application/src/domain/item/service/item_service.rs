use std::sync::Arc;
use std::vec::Vec;

use async_trait::async_trait;
use openapi::models::Item;
use uuid::Uuid;

use crate::domain::item::repository::tokio_item_repository::TokioItemRepository;
use crate::domain::item::repository::ItemRepository;

use super::ItemService;

pub struct DefaultItemService {
    repository: Arc<dyn ItemRepository + Send + Sync>,
}

#[async_trait]
impl ItemService for DefaultItemService {
    async fn find_all(&self) -> Vec<Item> {
        self.repository.find_all().await
    }

    async fn save(&self, item: &Item) -> Item {
        self.repository.save(item).await
    }

    async fn find_by_id(&self, item_id: Uuid) -> Item {
        self.repository.find_by_id(item_id).await.unwrap()
    }
}

impl Default for DefaultItemService {
    fn default() -> Self {
        DefaultItemService {
            repository: Arc::new(TokioItemRepository {}),
        }
    }
}
