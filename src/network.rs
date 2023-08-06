use std::fmt::{Display, Formatter};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use anyhow::Result;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;
use trust_dns_resolver::{system_conf, TokioAsyncResolver, TokioHandle};
use trust_dns_resolver::config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts};


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

///列出系统的网络接口。
///
///  # Returns
///
/// 网络接口按照系统配置中定义的顺序返回。
///
///
/// # Errors
///
/// 如果无法读取系统配置
///
/// # Examples
///
/// ```
/// let interfaces = ip::list_interfaces().unwrap();
/// println!("interfaces: {:?}", interfaces);
/// ```
pub async fn interfaces()  -> Result<Vec<Interface>> {
    spawn_blocking(||  get_if_addrs::get_if_addrs())
        .await??
        .into_iter()
        .try_fold(Vec::new(), |mut acc, i|  {
            acc.push(Interface{
                name: i.name.clone(),
                ip: i.ip().to_string(),
            });
            Ok(acc)
        })
}


#[derive(Serialize)]
pub struct Interface {
    ///网络接口名称。
    name: String,

    ///网口的IP地址。
    ip: String,
}

impl Display for Interface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{}", self.name, self.ip)
    }
}

/// 保存IP地址的类别。类别可以是公共的、本地的或任意的。
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord,Serialize, ValueEnum)]
pub enum IpCategory {
    #[clap(name = "public")]
    Public,

    #[clap(name = "local")]
    Local,

    #[clap(name = "any")]
    Any,
}

impl Display for IpCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IpCategory::Public => write!(f, "public"),
            IpCategory::Local => write!(f, "local"),
            IpCategory::Any => write!(f, "*"),
        }
    }
}

///默认的DNS服务器端口
///
/// 此常量作为默认值用于查询公共IP地址
pub const DNS_DEFAULT_PORT:u16 = 53;


/// openns服务器主机
///
/// 此常量作为默认值用于查询公共IP地址
pub const OPENDNS_SERVER_HOST:&str = "208.67.222.222";

/// 从提供的dns服务器上查询公网IP地址
/// 只返回IPv4地址。
///
/// # Arguments
///
/// * `dns_server_host` - 要查询公网IP地址的DNS服务器主机
/// * `dns_server_port` - 查询公网IP地址的DNS服务器端口
///
/// # Returns
///
/// The public IP address.
///
/// # Errors
///
/// 如果无法解析DNS服务器主机，或者无法查询到DNS服务器。
///
/// # Examples
///
/// ```
/// use std::net::IpAddr;
///
/// let public_ip = ip::query_public_ip(ip::OPENDNS_SERVER_HOST, 53).unwrap();
/// println!("public ip: {}", public_ip);
/// ```
pub async fn query_public_ip(dns_server_host:&str, dns_server_port:u16) -> Result<IpAddr> {

    // 设置解析器配置
    let dns_server_addr  = SocketAddr::new(dns_server_host.parse()?, dns_server_port);
    let nameserver_config = NameServerConfig::new(dns_server_addr, Protocol::Udp);
    let resolver_config = ResolverConfig::from_parts(None,vec![], vec![nameserver_config]);


    let mut resolver_opts = ResolverOpts::default();
    resolver_opts.ndots = 1;
    resolver_opts.timeout = std::time::Duration::from_secs(5);

    //创建解析器
    let resolver = TokioAsyncResolver::new(resolver_config, resolver_opts,TokioHandle)?;

    //向OpenDNS服务器查询公网IP地址
    let ipv4_response = resolver.ipv4_lookup("myip.opendns.com").await?;

    let ipv4: &Ipv4Addr = ipv4_response.iter().next().unwrap();

    Ok(IpAddr::V4(*ipv4))

}

/// 分类IP地址
#[derive(Serialize, Deserialize, Debug)]
pub struct Ip {
    /// ip 地址
    pub address: IpAddr,

    /// ip 类别
    pub category: IpCategory,
}

impl Display for Ip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{}", self.category, self.address)
    }
}