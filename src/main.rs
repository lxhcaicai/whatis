use std::fmt::{Display, Pointer};

use anyhow::{Result, Context};
use clap::{Parser, Subcommand, ValueEnum};
use human_panic::setup_panic;
use serde::{Serialize, Serializer};
use crate::Commands::Time;

mod country;
mod datetime;
mod network;
mod output;
mod system;

#[derive(Debug,Parser)]
#[command(name = "what")]
#[command(about = "Get essential information about your device")]
#[command(long_about = "Easily access important details about your device, such as IP addresses, DNS servers, date, time, and more.")]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    format:OutputFormat,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(name = "date")]
    #[command(about = "Display your system's date")]
    #[command(long_about = "Show the current date on your system in a human-readable format.\n\
    Example: Saturday, 8 April, 2023, week 14")]
    Date,


    #[command(name = "time")]
    #[command(about = "Display your system's current time")]
    #[command(long_about = "Show the current time on your system, along with the offset from the central NTP\n\
    clock server, in a 24-hour human-readable format.\n
    Example: 20:20:2 UTC +02:00 ±0.0672 seconds")]
    Time,

    #[command(name = "datetime")]
    #[command(about = "Display your system's current date and time")]
    #[command(long_about = "Show the current date and time on your system, along with the offset from\n\
    the central NTP clock server, in a human-readable format.\n\
    Example: Saturday, 8 April, 2023, week 14 20:20:2 UTC +02:00 ±0.0684 seconds")]
    Datetime,

    #[command(name = "dns")]
    #[command(about = "Display your system's DNS servers")]
    #[command(long_about = "Show the DNS servers configured on your system, listed in the order they are used.")]
    Dns,

    #[command(name = "hostname")]
    #[command(about = "Display your system's hostname")]
    #[command(long_about = "Show the hostname assigned to your system.")]
    Hostname,

    #[command(name = "username")]
    #[command(about = "Display your current system user's username")]
    #[command(long_about = "Show the username of the currently logged-in system user.")]
    Username,

    #[command(name = "device-name")]
    #[command(about = "Display your device's name")]
    #[command(long_about = "Show the configured name of your device.")]
    DeviceName,

    #[command(name = "os")]
    #[command(about = "Display your system's OS name and version")]
    #[command(long_about = "Show the name and version of the operating system installed on your system.")]
    Os,

    #[command(name = "architecture")]
    #[command(about = "Display your system's CPU architecture")]
    #[command(long_about = "Show the architecture of the CPU installed on your system.")]
    Architecture,

    #[command(name = "cpu")]
    #[command(about = "Display your system's CPU")]
    #[command(long_about = "Show the name of the CPU installed on your system.")]
    Cpu,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 启用人类可读的紧急信息
    setup_panic!();
    // 解析CLI参数
    let cli = Cli::parse();

    if let Some(command) = &cli.command{
        let result: CommandResult = match command {
            Commands::Date => CommandResult::Date(
                datetime::date().await
                    .with_context(|| "looking up the system's date failed")?
            ),
            Commands::Time => CommandResult::Time(
                datetime::time().await
                    .with_context(||"looking up the system's time failed" )?
            ),
            Commands::Datetime => CommandResult::Datetime(
                datetime::dateTime().await
                    .with_context(|| "looking up the system's datetime failed")?
            ),
            Commands::Dns => CommandResult::Dns(
                network::list_dns_servers().await
                    .with_context(|| "listing the system's dns servers failed")?
            ),
            Commands::Hostname => CommandResult::Hostname(
                system::hostname().await
                    .with_context(|| "looking up the system's hostname failed")?
            ),
            Commands::Username => CommandResult::Username(
                system::username().await
                    .with_context(|| "looking up the user's username failed")?
            ),
            Commands::DeviceName => CommandResult::DeviceName(
                system::device_name().await
                    .with_context(|| "looking up the systems' device name failed")?
            ),
            Commands::Os => CommandResult::Os(
                system::os().await
                    .with_context(|| "looking up the system's OS name failed")?
            ),
            Commands::Architecture => CommandResult::Architecture(
                system::architecture().await
                    .with_context(|| "looking up the CPU's architecture fialed")?
            ),
            Commands::Cpu => CommandResult::Cpu(
                system::cpus().await
                    .with_context(|| "looking up the system's CPU information failed")?
            )
        };

        match cli.format {
            OutputFormat::Json => {
                let json_repr = serde_json::to_string_pretty(&result)?;
                println!("{}", json_repr);
            }
            OutputFormat::Text => {
                println!("{}", result);
            }
        }
    }

    Ok(())
}

/// CommandResult保存命令的结果
/// 这是为了方便分解命令执行，
/// 并允许将结果序列化为所需的输出格式
enum CommandResult {
    Date(datetime::Date),
    Time(datetime::Time),
    Datetime(datetime::Datetime),
    Dns(Vec<String>),
    Hostname(output::Named),
    Username(output::Named),
    DeviceName(output::Named),
    Os(output::Named),
    Architecture(output::Named),
    Cpu(system::Cpu)
}


impl Display for CommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandResult::Date(date) => date.fmt(f),
            CommandResult::Time(time) => time.fmt(f),
            CommandResult::Datetime(datetime) => datetime.fmt(f),
            CommandResult::Dns(dns) => {
                write!(f, "{}", dns.join("\n"))
            },
            CommandResult::Hostname(hostname) => hostname.fmt(f),
            CommandResult::Username(username) => username.fmt(f),
            CommandResult::DeviceName(device_name) => device_name.fmt(f),
            CommandResult::Os(os) => os.fmt(f),
            CommandResult::Architecture(architecture) => architecture.fmt(f),
            CommandResult::Cpu(cpu) => cpu.fmt(f),
        }
    }
}

impl serde::Serialize for CommandResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        match self {
            CommandResult::Date(date) => date.serialize(serializer),
            CommandResult::Time(time) => time.serialize(serializer),
            CommandResult::Datetime(datetime) => datetime.serialize(serializer),
            CommandResult::Dns(dns) => dns.serialize(serializer),
            CommandResult::Hostname(hostname) => hostname.serialize(serializer),
            CommandResult::Username(username) => username.serialize(serializer),
            CommandResult::DeviceName(device_name) => device_name.serialize(serializer),
            CommandResult::Os(os) => os.serialize(serializer),
            CommandResult::Architecture(architecture) => architecture.serialize(serializer),
            CommandResult::Cpu(cpu) => cpu.serialize(serializer),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    Json,
    Text,
}