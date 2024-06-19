use std::error::Error;

use mysql_async::{prelude::Queryable, Row};

use crate::model::{Field, Structs};

pub struct Mysql;

impl Mysql {
    pub async fn fetch_mysql_schema(
        db_url: &str,
        db_name: &str,
        table_name: &str,
    ) -> Result<Structs, Box<dyn Error>> {
        let pool = mysql_async::Pool::new(db_url);
        let mut conn = pool.get_conn().await?;
        //mysql数据库获取表列名 数据类型 长度 是否可为空
        //SELECT COLUMN_NAME, DATA_TYPE, CHARACTER_MAXIMUM_LENGTH, IS_NULLABLE
        let table_schema_query = format!(
            "SELECT COLUMN_NAME, DATA_TYPE，CHARACTER_MAXIMUM_LENGTH, IS_NULLABLE
         FROM INFORMATION_SCHEMA.COLUMNS
         WHERE TABLE_SCHEMA = '{}' AND TABLE_NAME = '{}'",
            db_name, table_name
        );
        //let result: Vec<(String, String)> = conn.query(table_schema_query).await?;
        let rows: Vec<Row> = conn.query(table_schema_query).await?;
        //CHARACTER_MAXIMUM_LENGTH会返回null，需要处理
        //nullable需要处理
        let mut fields = Vec::new();
        for row in rows {
            let name: String = row.get("COLUMN_NAME").expect("未查询到name字段");
            let data_type: String = row.get("DATA_TYPE").expect("未查询到data_type字段");
            let character_maximum_length: Option<i32> = row.get("CHARACTER_MAXIMUM_LENGTH");
            let nullable_str: String = row.get("IS_NULLABLE").expect("未查询到is_nullable字段");
            //is_nullable 是字符串，需要转换成布尔值 如果不设置返回的也是"YES"
            //如果字符串是"YES"，则布尔值为true，如果字符串是"NO"，则布尔值为false
            let nullable: bool = match nullable_str.as_str() {
                "Yes" => true,
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
