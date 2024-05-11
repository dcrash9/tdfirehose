// extern crate chrono;
use chrono::{DateTime, NaiveDate, TimeZone};
use csv::WriterBuilder;
use serde::Serialize;
use std::path::Path;
use time::{Duration, Time};

use std::fs::OpenOptions;
use std::io::BufWriter;

pub fn write_to_csv<T: Serialize>(file_path: &str, record: T) -> Result<(), csv::Error> {
    // Check if the file already exists
    let path = Path::new(file_path);
    let file_exists = path.exists();
    // Open or create the file
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;
    // Create a buffered writer
    let mut writer = BufWriter::new(file);
    // Create a CSV writer with or without headers based on file existence
    let mut csv_writer = if file_exists {
        // Create a writer that does not write headers
        WriterBuilder::new().has_headers(false).from_writer(&mut writer)
    } else {
        // Create a writer that writes headers
        WriterBuilder::new().has_headers(true).from_writer(&mut writer)
    };
    // Serialize the record and write to the CSV file
    csv_writer.serialize(record)?;
    csv_writer.flush()?;
    Ok(())
}

pub fn combine_date_time(
    date_int: i32,
    ms_of_day: u64,
) -> Result<DateTime<chrono::Utc>, Box<dyn std::error::Error>> {
    let date_str = date_int.to_string();
    // Parse the date
    let date = NaiveDate::parse_from_str(&date_str, "%Y%m%d")?;
    // Create a time object from milliseconds
    let total_seconds = ms_of_day / 1000;
    let nanoseconds = (ms_of_day % 1000) * 1_000_000;
    let time = Time::MIDNIGHT
        + Duration::seconds(total_seconds as i64)
        + Duration::nanoseconds(nanoseconds as i64);
    // Construct a DateTime object
    let datetime = date.and_hms_nano_opt(
        time.hour() as u32,
        time.minute() as u32,
        time.second() as u32,
        time.nanosecond(),
    );
    match datetime {
        Some(dt) => Ok(chrono::Utc.from_utc_datetime(&dt)),
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid date time",
        ))),
    }
}
// 
// pub fn write_to_csv<T: Serialize>(file_path: &str, record: T) -> Result<(), csv::Error> {
//     // Check if the file already exists
//     let path = Path::new(file_path);
//     let file_exists = path.exists();
//     // Open or create the file
//     let file = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open(file_path)?;
//     // Create a CSV writer with or without headers based on file existence
//     let mut writer = if file_exists {
//         // Create a writer that does not write headers
//         WriterBuilder::new().has_headers(false).from_writer(file)
//     } else {
//         // Create a writer that writes headers
//         WriterBuilder::new().has_headers(true).from_writer(file)
//     };
//     // Serialize the record and write to the CSV file
//     writer.serialize(record)?;
//     writer.flush()?;
//     Ok(())
// }

pub fn to_occ_contract(
    root: &String,
    expiration: i32,
    right: &String,
    strike: i64,
) -> Result<String, Box<dyn std::error::Error>> {
    let occ_strike = format!("{:08}", strike);
    let exp_date = NaiveDate::parse_from_str(&expiration.to_string(), "%Y%m%d")?;
    let year2digits = exp_date.format("%y");
    let month = exp_date.format("%m");
    let day = exp_date.format("%d");
    let symbol = format!(
        "{}{}{}{}{}{}",
        root, year2digits, month, day, right, occ_strike
    );
    Ok(symbol)
}
