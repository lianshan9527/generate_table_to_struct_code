use std::error::Error;

use tokio_postgres::NoTls;

use crate::model::{Field, Structs};

pub struct Postgres;

impl Postgres {
    pub async fn fetch_postgres_schema(
        db_url: &str,
        table_name: &str,
    ) -> Result<Structs, Box<dyn Error>> {
        let (client, connection) = tokio_postgres::connect(db_url, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let rows = client
            .query(
                "SELECT column_name, data_type, character_maximum_length, is_nullable
FROM information_schema.columns
         WHERE table_name = $1",
                &[&table_name],
            )
            .await?;

        let mut fields = Vec::new();
        for row in rows {
            let name: String = row.get(0);
            let data_type: String = row.get(1);
            let character_maximum_length: Option<i32> = row.get(2);
            //is_nullable 是字符串，需要转换成布尔值
            //如果字符串是"YES"，则布尔值为true，如果字符串是"NO"，则布尔值为false
            let nullable: bool = match row.get::<_, &str>(3) {
                "YES" => true,
                "NO" => false,
                _ => false,
            };
            fields.push(Field {
                name,
                data_type: Field::to_rust_type(&data_type).to_string(),
                character_maximum_length,
                nullable,
            });
        }

        Ok(Structs {
            name: table_name.to_string(),
            fields,
        })
    }
}
