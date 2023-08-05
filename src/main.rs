use std::fmt::{Display, Pointer};

use anyhow::{Result, Context};
use clap::{Parser, Subcommand, ValueEnum};
use human_panic::setup_panic;
use serde::{Serialize, Serializer};
use crate::Commands::Time;

mod country;
mod datetime;

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
}


impl Display for CommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandResult::Date(date) => date.fmt(f),
            CommandResult::Time(time) => time.fmt(f),
        }
    }
}

impl serde::Serialize for CommandResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        match self {
            CommandResult::Date(date) => date.serialize(serializer),
            CommandResult::Time(time) => time.serialize(serializer)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    Json,
    Text,
}