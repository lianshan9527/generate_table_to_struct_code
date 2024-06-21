use std::{cell::RefCell, sync::Arc};

use anyhow::Result;
use tokio;

mod data_base;
mod file;
mod model;

#[tokio::main(worker_threads = 4)]
async fn main() -> Result<()> {
    use data_base::Mysql;
    use data_base::Postgres;
    use file::{write_to_service_file, write_to_struct_file};
    use model::Structs;
    let db_type = "postgres"; // 修改此处使用 "postgres" 或 "mysql"
    let host = "localhost"; // 数据库地址
    let port = "5432"; // 数据库端口
    let user = "postgres"; // 数据库用户名
    let password = "root"; // 数据库密码
    let db_name = "sunshine"; // 数据库名
    let table_name = "user"; //  数据库表名称

    let table_struct = match db_type {
        "postgres" => {
            let db_url = format!(
                "host={} user={} password={} dbname={} port={} sslmode=disable",
                host, user, password, db_name, port
            );
            //let db_url = "host=localhost user=postgres password=root dbname=sunshine port=5432 sslmode=disable";
            Postgres::fetch_postgres_schema(db_url.as_str(), table_name)
                .await
                .unwrap_or_else(|_| {
                    println!("拉取postgres 数据库schema失败!");
                    Structs::default()
                })
        }
        "mysql" => {
            let db_url = format!("mysql://{}:{}@{}:{}/{}?characterEncoding=utf8&serverTimezone=GMT%2B8&userSSL=false", user, password, host, port, db_name);
            Mysql::fetch_mysql_schema(db_url.as_str(), db_name, table_name)
                .await
                .unwrap_or_else(|_| {
                    println!("拉取mysql 数据库schema失败!");
                    Structs::default()
                })
        }
        _ => panic!("暂不支持该数据库!"),
    };

    let table_info = Arc::new(tokio::sync::RwLock::new(table_struct));
    let table_info_clone = table_info.clone();

    //生成struct层代码
    let struct_task = tokio::spawn(async move {
        let struct_code = table_info
            .clone()
            .read()
            .await
            .generate_struct_code()
            .await
            .unwrap_or_else(|e| {
                println!("生成struct层代码失败! {:#?}", e);
                "".to_string()
            });
        write_to_struct_file(table_name, &struct_code)
            .await
            .unwrap_or_else(|e| {
                println!("生成struct文件失败! {:#?}", e);
                ()
            });
    });

    //生成service层代码
    let service_task = tokio::spawn(async move {
        let service_code = table_info_clone
            .read()
            .await
            .generate_service_code()
            .await
            .unwrap_or_else(|e| {
                println!("生成service层代码失败! {:#?}", e);
                "".to_string()
            });
        write_to_service_file(table_name, &service_code)
            .await
            .unwrap_or_else(|_| {
                println!("生成service文件失败!");
                ()
            });
    });

    let (res_strcut, _) = tokio::join!(struct_task, service_task);
    res_strcut?;
    Ok(())
}
