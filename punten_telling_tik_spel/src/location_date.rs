use serde::{Serialize};

// Date and Location structures
#[derive(Serialize, Debug, Clone)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

#[allow(unused)]
impl Date
{
    pub fn new(year: u32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> Self {
        Date {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    pub fn from_timestamp(timestamp: u64) -> Self {
        // Simple conversion assuming timestamp is in seconds since epoch
        use chrono::{DateTime, Datelike, Timelike};
        let dt = DateTime::from_timestamp(timestamp as i64, 0).unwrap();
        let naive = dt.naive_utc();
        Date {
            year: naive.year() as u32,
            month: naive.month(),
            day: naive.day(),
            hour: naive.hour(),
            minute: naive.minute(),
            second: naive.second(),
        }
    }  

    pub fn from_tijdstempel(tijdstempel: &str) -> Self {
     // 19-10-2025 14:12:22
     // 20-10-2025 13:45:19
        let parts: Vec<&str> = tijdstempel.split(' ').collect();
        if parts.len() != 2 {
            return Date::new(0, 0, 0, 0, 0, 0);
        }
        let date_parts: Vec<&str> = parts[0].split('-').collect();
        let time_parts: Vec<&str> = parts[1].split(':').collect();
        if date_parts.len() != 3 || time_parts.len() != 3 {
            return Date::new(0, 0, 0, 0, 0, 0);
        }
        let day = date_parts[0].parse::<u32>().unwrap_or(0);
        let month = date_parts[1].parse::<u32>().unwrap_or(0);
        let year = date_parts[2].parse::<u32>().unwrap_or(0);
        let hour = time_parts[0].parse::<u32>().unwrap_or(0);
        let minute = time_parts[1].parse::<u32>().unwrap_or(0);
        let second = time_parts[2].parse::<u32>().unwrap_or(0);
        Date::new(year, month, day, hour, minute, second)
    }
}

#[derive(Serialize, Debug)]
pub struct Locatie {
    pub latitude: f64,
    pub longitude: f64,
    pub date: Date,
}

