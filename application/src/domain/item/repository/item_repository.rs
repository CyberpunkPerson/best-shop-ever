use super::ItemRepository;
use openapi::models::item::Item;
use shaku::Component;
use uuid::Uuid;

#[derive(Component)]
#[shaku(interface = ItemRepository)]
pub struct DefaultItemRepository;

impl ItemRepository for DefaultItemRepository {
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
        Item {
            id: item.id,
            title: String::from("title"),
            description: String::from("description"),
            price: String::from("some price"),
        }
    }
    fn find_by_id(&self, item_id: Uuid) -> Option<Item> {
        match item_id.to_hyphenated().to_string().as_ref() {
            "2a9ae427-abab-4c11-aa19-7f9870dfc03c" => Option::from(Item {
                id: item_id,
                title: String::from("title"),
                description: String::from("description"),
                price: String::from("some price"),
            }),
            _ => None,
        }
    }
}
