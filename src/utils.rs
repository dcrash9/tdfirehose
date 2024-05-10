use chrono::{DateTime, NaiveDate, TimeZone};
use csv::WriterBuilder;
use serde::Serialize;
use std::fs::OpenOptions;
use std::path::Path;
use time::{Duration, Time};

pub fn combine_date_time(date_int: i32, ms_of_day: u64) -> Result<DateTime<chrono::Utc>, Box<dyn std::error::Error>> {
    let date_str = date_int.to_string();
    // Parse the date
    let date = NaiveDate::parse_from_str(&date_str, "%Y%m%d")?;
    // Create a time object from milliseconds
    let total_seconds = ms_of_day / 1000;
    let nanoseconds = (ms_of_day % 1000) * 1_000_000;
    let time = Time::MIDNIGHT + Duration::seconds(total_seconds as i64) + Duration::nanoseconds(nanoseconds as i64);
    // Construct a DateTime object
    let datetime = date.and_hms_nano_opt(
        time.hour() as u32,
        time.minute() as u32,
        time.second() as u32,
        time.nanosecond() as u32,
    );
    match datetime {
        Some(dt) => Ok(chrono::Utc.from_utc_datetime(&dt)),
        None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid date time"))),
    }
}

pub fn write_to_csv<T: Serialize>(file_path: &str, record: T) -> Result<(), csv::Error> {
    // Check if the file already exists
    let path = Path::new(file_path);
    let file_exists = path.exists();
    // Open or create the file
    let file = OpenOptions::new().write(true).create(true).append(true).open(file_path)?;
    // Create a CSV writer with or without headers based on file existence
    let mut writer = if file_exists {
        // Create a writer that does not write headers
        WriterBuilder::new().has_headers(false).from_writer(file)
    } else {
        // Create a writer that writes headers
        WriterBuilder::new().has_headers(true).from_writer(file)
    };
    // Serialize the record and write to the CSV file
    writer.serialize(record)?;
    writer.flush()?;
    Ok(())
}


// pub fn is_third_friday(date:i32) -> bool {
//     let expdate = NaiveDate::parse_from_str(&date.to_string(), "%Y%m%d");
//     // Check if the day of the week is Friday
//     if expdate.weekday() != Weekday::Fri {
//         return false;
//     }
//     // Check if it's the third week of the month
//     let day = expdate.day();
//     if day >= 15 && day <= 21 {
//         return true;
//     }
//     false
// }


// fn to_iso_contract(root: &str, expiration: &i32, right: &str, strike: f64) -> Result<String, Box<dyn std::error::Error>> {
        // to_iso_contract('AAPL', '20230519', 'C', 192500)
        // 'AAPL230519C92500000'
//     // Calculate iso strike
//     let (a, b) = strike.fract().abs().mul_add(1000.0, strike.trunc()).div_rem(1000.0);
//     let occ_strike = format!("{:05}{:03}", a, b * 1000.0);
//
//     // Parse expiration date
//     let exp_date = NaiveDate::parse_from_str(expiration, "%Y%m%d")
//         .or_else(|_| NaiveDate::parse_from_str(expiration, "%Y-%m-%d"))?;
//
//     // Format expiration date
//     let year2digits = exp_date.format("%y");
//     let month = exp_date.format("%m");
//     let day = exp_date.format("%d");
//
//     // Construct contract name
//     let contract = format!("{}{}{}{}{} {}", root, year2digits, month, day, right, occ_strike);
//
//     Ok(contract)
// }


