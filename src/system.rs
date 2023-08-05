use anyhow::Result;
use crate::output::{create_named, Named, NamedKind};

// 返回系统的主机名作为命名enum
pub async fn hostname() -> Result<Named> {
    create_named(|| async{ whoami::hostname().to_string() }, NamedKind::Hostname).await
}


