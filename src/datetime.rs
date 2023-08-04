use std::fmt::{Display, Formatter};

use anyhow::Result;
use colored::*;
use chrono::{DateTime, Local};
use rsntp::AsyncSntpClient;
use serde::Serialize;

/// 返回系统时间
pub async fn date() -> Result<Date> {

    let dt = Local::now();
    let now_with_tz = dt.with_timezone(&Local);
    Ok(now_with_tz.into())
}

#[derive(Serialize)]
pub struct Date {
    day_name:String,
    day_number:u8,
    month_name:String,
    year:i32,
    week_number:u8
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.day_name)?;
        write!(f, ", {} {}", self.day_number, self.month_name)?;
        write!(f, ", {}", self.year)?;
        write!(f, ", week {}", self.week_number)
    }
}

impl From<DateTime<Local>> for Date {
    fn from(dt: DateTime<Local>) -> Self {
        Date {
            day_name:dt.format("%A").to_string(),
            day_number:dt.format("%d").to_string().parse::<u8>().unwrap(),
            month_name: dt.format("%B").to_string(),
            year: dt.format("%Y").to_string().parse::<i32>().unwrap(),
            week_number: dt.format("%U").to_string().parse::<u8>().unwrap(),
        }
    }
}