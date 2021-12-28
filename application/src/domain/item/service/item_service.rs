use super::ItemService;
use openapi::models::Item;
use shaku::Component;
use std::vec::Vec;
use uuid::Uuid;

#[derive(Component)]
#[shaku(interface = ItemService)]
pub struct DefaultItemService;

impl ItemService for DefaultItemService {
    fn find_all(&self) -> Vec<Item> {
        let item = Item {
            id: Uuid::new_v4(),
            title: String::from("title"),
            description: String::from("description"),
            price: String::from("some price"),
        };
        vec![item]
    }
    fn save(&self, item: &Item) -> Item {
        let return_item = Item {
            id: item.id,
            title: String::from("title"),
            description: String::from("description"),
            price: String::from("some price"),
        };
        return_item
    }
    fn find_by_id(&self, item_id: Uuid) -> Item {
        Item {
            id: item_id,
            title: String::from("title"),
            description: String::from("description"),
            price: String::from("some price"),
        }
    }
}
