use async_trait::async_trait;
use rust_decimal_macros::dec;
use uuid::Uuid;

use openapi::models::item::Item;

use super::ItemRepository;

pub struct StubItemRepository;

#[async_trait]
impl ItemRepository for StubItemRepository {
    async fn find_all(&self) -> Vec<Item> {
        let item = Item {
            id: Uuid::new_v4(),
            title: String::from("title"),
            description: String::from("description"),
            price: dec!(202.2),
        };
        vec![item]
    }
    async fn save(&self, item: &Item) -> Item {
        Item {
            id: item.id,
            title: String::from("title"),
            description: String::from("description"),
            price: dec!(202.2),
        }
    }
    async fn find_by_id(&self, item_id: Uuid) -> Option<Item> {
        match item_id.to_hyphenated().to_string().as_ref() {
            "2a9ae427-abab-4c11-aa19-7f9870dfc03c" => Option::from(Item {
                id: item_id,
                title: String::from("title"),
                description: String::from("description"),
                price: dec!(202.2),
            }),
            _ => None,
        }
    }
}