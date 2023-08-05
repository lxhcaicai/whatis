use std::fmt::Display;
use anyhow::Result;
use serde::ser::{Serialize, SerializeMap, Serializer};

/// Named是表示命名值的enum。
///用于表示命令的输出，同时也
///为值提供一个名称。
///也就是说，它用来表示命令的输出
///返回单个值，但也为该值提供一个名称
//值。以便可以将输出序列化为JSON
///例如:
pub enum Named {
    Hostname(String),
    Username(String),
    DeviceName(String),
}

pub enum NamedKind {
    Hostname,
    Username,
    DeviceName,
}

impl Named {
    fn value(&self) -> &str {
        match self {
            Named::Hostname(value)
            | Named::Username(value)
            | Named::DeviceName(value) => value,
        }
    }
}

impl Display for Named {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl Serialize for Named {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: Serializer {
        let mut map = serializer.serialize_map(Some(1))?;
        match self {
            Named::Hostname(value) => map.serialize_entry("hostname", value)?,
            Named::Username(value) => map.serialize_entry("username", value)?,
            Named::DeviceName(value) => map.serialize_entry("device_name", value)?,
        }
        map.end()
    }
}

/// create_named是一个从函数创建命名enum的函数
///返回一个字符串。
pub async fn create_named<F,Fut>(func: F, data_type:NamedKind) -> Result<Named>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = String>
{
    let value = func().await;
    match data_type {
        NamedKind::Hostname => Ok(Named::Hostname(value)),
        NamedKind::Username => Ok(Named::Username(value)),
        NamedKind::DeviceName => Ok(Named::DeviceName(value)),
    }
}