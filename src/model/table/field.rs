use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub struct Field {
    pub name: String,
    pub data_type: String,
    pub character_maximum_length: Option<i32>,
    pub nullable: bool,
}

impl Field {
    pub fn to_rust_type(data_type: &str) -> &str {
        match data_type {
            "integer" | "int4" | "int" => "i32",
            "bigint" | "int8" => "i64",
            "smallint" | "int2" => "i16",
            "varchar" | "text" | "char" | "bpchar" => "String",
            "boolean" | "bool" => "bool",
            "real" | "float4" => "f32",
            "double precision" | "float8" => "f64",
            //"timestamp" | "timestamp without time zone" => "chrono::NaiveDateTime",
            "timestamp" | "timestamp without time zone" => "chrono::DateTime",
            _ => "String",
        }
    }

}
