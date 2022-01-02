use openapi::models::item::Item;
use postgres::{Client, NoTls};
use shaku::Component;
use uuid::Uuid;

use super::ItemRepository;

#[derive(Component)]
#[shaku(interface = ItemRepository)]
pub struct TokioItemRepository;

impl ItemRepository for TokioItemRepository {
    fn find_all(&self) -> Vec<Item> {
        let mut client = Client::connect(
            "host=localhost user=postgres password=postgres dbname=postgres port=5432",
            NoTls,
        )
        .unwrap();
        let rows = client
            .query("select * from best_shop_ever.item", &[])
            .unwrap();
        // let items: Vec<Item> = serde_postgres::from_rows(&rows).unwrap();
        let mut items: Vec<Item> = vec![];
        for row in rows {
            items.push(Item {
                id: row.get("id"),
                title: row.get("title"),
                description: row.get("description"),
                price: String::from("Some price"),
            });
        }
        items
    }
    fn save(&self, item: &Item) -> Item {
        todo!()
    }
    fn find_by_id(&self, item_id: Uuid) -> Option<Item> {
        todo!()
    }
}

// async fn connect() -> Option<Client> {
//     let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=postgres dbname=postgres port=5432", NoTls).await.unwrap();
//     tokio::spawn(connection);
//     Some(client)
// }
