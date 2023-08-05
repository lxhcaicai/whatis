use anyhow::Result;
use trust_dns_resolver::{system_conf, TokioAsyncResolver, TokioHandle};


/// 列出系统配置中的DNS服务器。
/// DNS服务器以IP地址列表的形式返回。
/// 完成DNS服务器重复数据删除。
/// DNS服务器按照系统配置中定义的顺序返回。
///
/// # Returns
///
/// The DNS servers:
///   * The DNS servers are returned as a list of IP addresses.
///   * The DNS servers are deduplicated.
///   * The DNS servers are returned in the order they are defined in the system configuration.
///
/// # Errors
///
/// If the system configuration cannot be read.
///
/// # Examples
///
/// ```
/// let dns_servers = ip::list_dns_servers().unwrap();
/// println!("dns servers: {:?}", dns_servers);
pub async fn list_dns_servers() -> Result<Vec<String>> {
    let (conf, _) = system_conf::read_system_conf()?;
    let mut nameservers = conf
        .name_servers()
        .iter()
        .map(|ns| {
            ns.socket_addr
                .to_string()
                .splitn(2,':')
                .next()
                .unwrap()
                .to_owned()
        }).collect::<Vec<_>>();

    nameservers.dedup();
    Ok(nameservers)
}