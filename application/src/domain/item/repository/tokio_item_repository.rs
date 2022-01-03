use async_trait::async_trait;
use tokio_postgres::{Client, NoTls};
use uuid::Uuid;

use openapi::models::item::Item;

use super::ItemRepository;

pub struct TokioItemRepository;

#[async_trait]
impl ItemRepository for TokioItemRepository {
    async fn find_all(&self) -> Vec<Item> {
        let client = connect().await.unwrap();
        let rows = client
            .query("select * from best_shop_ever.item", &[])
            .await
            .unwrap();

        rows.into_iter()
            .map(|row| Item {
                id: row.get("id"),
                title: row.get("title"),
                description: row.get("description"),
                price: row.get("price"),
            })
            .collect()
    }
    async fn save(&self, item: &Item) -> Item {
        let client = connect().await.unwrap();
        let row = client
            .query_one(
                "
            insert into best_shop_ever.item (id, title, description)
            values ($1, $2, $3)
            on conflict (id)
            do update
               set
                   title = excluded.title,
                   description = excluded.description
            returning id, title, description, price;
        ",
                &[&item.id, &item.title, &item.description],
            )
            .await
            .unwrap();

        Item {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            price: row.get("price"),
        }
    }
    async fn find_by_id(&self, item_id: Uuid) -> Option<Item> {
        let client = connect().await.unwrap();
        let row = client
            .query_one(
                "
        select * from best_shop_ever.item
        where id = $1
        ",
                &[&item_id],
            )
            .await
            .unwrap();

        Some(Item {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            price: row.get("price"),
        })
    }
}

async fn connect() -> Option<Client> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=postgres port=5432",
        NoTls,
    )
    .await
    .unwrap();
    tokio::spawn(connection);
    Some(client)
}
