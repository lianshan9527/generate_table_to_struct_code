use std::error::Error;

use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

pub async fn write_to_struct_file(
    table_name: &str,
    struct_code: &str,
) -> Result<(), Box<dyn Error>> {
    //获取项目根路径
    let mut path = std::env::current_dir().unwrap();
    //在根路径下创建models文件夹
    // 在根路径下创建models文件夹
    path.push("models");
    if !path.exists() {
        fs::create_dir(&path).await?;
    }
    // 创建以table_name命名的子文件夹
    path.push(table_name);
    if !path.exists() {
        fs::create_dir(&path).await?;
    }
    /* path.push("src");
    path.push("models"); */
    let filename = format!("{}.rs", table_name);
    //创建文件
    let mut file = File::create(path.join(filename)).await?;
    file.write_all(struct_code.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}

pub async fn write_to_service_file(
    table_name: &str,
    struct_code: &str,
) -> Result<(), Box<dyn Error>> {
    todo!("write_to_service_file")
}
