use anyhow::Result;
use tokio;

mod data_base;
mod file;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    use data_base::Mysql;
    use data_base::Postgres;
    use file::{write_to_service_file, write_to_struct_file};
    use model::Structs;

    let db_type = "postgres"; // 修改此处使用 "postgres" 或 "mysql"
    let db_url = "host=localhost user=postgres password=root dbname=sunshine port=5432"; // 对应的数据库连接URL
    let db_name = "sunshine"; // 对MySQL来说使用的数据库名
    let table_name = "user"; // 目标表名称

    let table_struct = match db_type {
        "postgres" => Postgres::fetch_postgres_schema(db_url, table_name)
            .await
            .unwrap_or_else(|_| {
                println!("拉取postgres 数据库schema失败!");
                Structs::default()
            }),
        "mysql" => Mysql::fetch_mysql_schema(db_url, db_name, table_name)
            .await
            .unwrap_or_else(|_| {
                println!("拉取mysql 数据库schema失败!");
                Structs::default()
            }),
        _ => panic!("暂不支持该数据库!"),
    };

    let struct_code = table_struct.generate_struct_code().await;

    //生成service层代码
    let service_code = table_struct.generate_service_code().await;
    tokio::spawn(async move {
        write_to_service_file(table_name, &service_code)
            .await
            .unwrap_or_else(|_| {
                println!("生成service文件失败!");
                ()
            });
    });
    tokio::spawn(async move {
        write_to_struct_file(table_name, &struct_code)
            .await
            .unwrap_or_else(|_| {
                println!("生成struct文件失败!");
                ()
            });
    });
    Ok(())
}
