## 项目结构说明
```
/  generate_table_to_struct_code    
├─ models                 # 生成文件的文件夹,可以不设置，默认生成                   
├─ src                    
│  ├─ data_base           # 数据库相关
│  │  ├─ mysql            # 对mysql的支持
│  │  └─ postgres         # 对postgres的支持
│  ├─ model               # 映射模型相关代码
│  │  ├─ table              
│  │  │  ├─ field.rs      # 表结构信息映射模型
│  │  │  └─ structs.rs    # 数据库表映射模型(包含表结构信息和表名)及相关方法
│  │  └─ sql              # 接收文件信息生成文件相关代码
│  ├─ file                # 文件生成相关
│  └─ main.rs             # 入口文件
├─ Cargo.toml             # cargo文件
└─ README.md              # 项目说明文件

```


