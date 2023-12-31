use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt};
use std::fmt::{Display, Formatter};
use anyhow::Result;
use colored::*;
use serde::Serialize;
use crate::format::human_readable_size;
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

pub async fn architecture() -> Result<Named> {
    create_named(|| async {whoami::arch().to_string()}, NamedKind::Architecture).await
}


/// CPU的描述
#[derive(Serialize)]
pub struct Cpu {

    // CPU的品牌
    pub brand: String,

    // CPU核心数量
    pub core_count: usize,

    // CPU的频率，单位为MHz
    pub frequency: u64
}

impl Display for Cpu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {} cores running at {} GHz", self.brand.bold(), self.core_count.to_string().cyan(), self.frequency.to_string().green())
    }
}

pub async fn cpus() -> Result<Cpu> {
    let mut system = System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::new().with_frequency()));
    system.refresh_cpu();

    let cpus = system.cpus();
    let reference_cpu = cpus.get(0).unwrap();

    Ok(Cpu{
        brand: reference_cpu.brand().to_string(),
        core_count:cpus.len(),
        frequency:reference_cpu.frequency(),
    })
}

/// 描述系统的RAM
#[derive(Serialize)]
pub struct Ram {

    #[serde(rename = "total_ram_bytes")]
    pub total:u64,

    #[serde(rename = "used_ram_bytes")]
    pub used:u64,

    #[serde(rename = "free_ram_bytes")]
    pub free:u64,

    #[serde(rename = "available_ram_bytes")]
    pub available:u64,
}

impl Display for Ram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let total = human_readable_size(self.total);
        let used = human_readable_size(self.used);
        let used_percentage = (self.used as f64 / self.total as f64) * 100.0;

        println!("used percentage: {}", used_percentage);

        let (used_colored, used_percentage_colored) = match used_percentage {
            _ if used_percentage > 90.0 => (used.red(), format!("{:.1}", used_percentage).to_string().red()),
            _ if used_percentage > 70.0 => (used.yellow(), format!("{:.1}", used_percentage).to_string().yellow()),
            _ => (used.green(), format!("{:.1}", used_percentage).to_string().green()),
        };

        write!(
            f,
            "{} installed, {} in use ({}%)",
            total.bold(),
            used_colored,
            used_percentage_colored,
        )
    }
}

///以RAM结构返回系统的RAM
pub async fn ram() -> Result<Ram> {
    let mut system = System::new_with_specifics(RefreshKind::new().with_memory());
    system.refresh_system();

    Ok(Ram{
        total:system.total_memory(),
        used:system.used_memory(),
        free:system.free_memory(),
        available:system.available_memory(),
    })
}
