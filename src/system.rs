use anyhow::Result;
use crate::output::{create_named, Named, NamedKind};

// 返回系统的主机名作为命名enum
pub async fn hostname() -> Result<Named> {
    create_named(|| async{ whoami::hostname().to_string() }, NamedKind::Hostname).await
}

pub async fn username() -> Result<Named> {
    create_named(|| async {whoami::username().to_string()}, NamedKind::Username).await
}

pub async fn device_name() -> Result<Named> {
    create_named(|| async {whoami::devicename().to_string()}, NamedKind::DeviceName).await
}

pub async fn os() -> Result<Named> {
    create_named(|| async {whoami::distro().to_string()}, NamedKind::Os).await
}


