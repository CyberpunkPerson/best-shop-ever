use uuid::Uuid;
use openapi::models::item::Item

pub mod repository {

    pub Vec<Item> find_all();

    pub Item save(item: Item);

    pub Option<Item> find_by_id(item_id: Uuid);

}