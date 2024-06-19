use super::super::Field;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default, Clone, Deserialize)]
pub struct Structs {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Structs {
    pub async fn generate_struct_code(&self) -> String {
        //对名字进行处理，如果名字中有下划线，则将下划线替换为驼峰式命名法
        let struct_name = self.name.clone();
        // 遍历字符串中的字符，将 '_' 替换为对应的大写字符
        let struct_name = capitalize_struct_name(&struct_name);
        let mut struct_code = format!(
            "use serde::{{Deserialize, Serialize}};\n\nuse serde_json;\n\n #[derive(Debug, Serialize,,Clone,Deserialize,Default)]\npub struct {} {{\n",
            struct_name
        );

        //判断nullable字段，如果有，则添加Option
        for field in &self.fields {
            if field.nullable {
                struct_code.push_str(&format!(
                    "    pub {}: Option<{}>,\n",
                    field.name, field.data_type
                ));
            } else {
                struct_code.push_str(&format!("    pub {}: {},\n", field.name, field.data_type));
            }
        }
        struct_code.push_str("}\n");

        //为该对象生成to_json_string方法和from_json_str方法
        struct_code.push_str(&format!("impl {} {{\n", struct_name));
        struct_code.push_str(&format!(
            "    pub fn from_json_str(json_str: &str) -> Result<{}, serde_json::Error> {{\n        serde_json::from_str(json_str)\n    }}\n",
            struct_name
        ));
        struct_code.push_str(&format!(
            "    pub fn to_json_string(&self) -> String {{\n        serde_json::to_string(&self).unwrap()\n    }}\n",
        ));
        struct_code.push_str("}\n");

        //生成实现display trait的to_string方法
        struct_code.push_str(&format!("impl std::fmt::Display for {} {{\n", struct_name));
        struct_code.push_str(&format!(
            "    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{\n        write!(f, \"{{}}\", serde_json::to_string(&self).unwrap())\n    }}\n",
        ));
        struct_code.push_str("}\n");

        struct_code
    }

    pub async fn generate_service_code(&self) -> String {
        //对名字进行处理，如果名字中有下划线，则将下划线替换为驼峰式命名法
        let service_name = self.name.clone();
        // 遍历字符串中的字符，将 '_' 替换为对应的大写字符
        let mut service_name = capitalize_struct_name(&service_name);
        service_name.push_str("Service");

        let mut _service_code = format!("use crate::model::table::structs::{};\n\n", service_name);

        todo!("generate_service_code")
    }
}

//生成驼峰式命名法的结构名
fn capitalize_struct_name(name: &str) -> String {
    // 创建一个新的字符串以存储处理后的结果
    let mut result = String::with_capacity(name.len());
    // 获取字符串的字符迭代器
    let mut chars_iter = name.chars().peekable();
    // 用于标记下一个字符是否应该大写
    let mut to_uppercase = true;
    while let Some(c) = chars_iter.next() {
        if c == '_' {
            to_uppercase = true;
        } else if to_uppercase {
            // 如果需要大写
            result.push_str(&c.to_uppercase().to_string());
            to_uppercase = false;
        } else {
            result.push(c);
        }
    }
    result
}
