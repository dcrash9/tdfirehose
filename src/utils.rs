// extern crate chrono;
use chrono::{DateTime, NaiveDate, TimeZone};
use csv::WriterBuilder;
use serde::Serialize;
use std::fs::OpenOptions;
use std::path::Path;
use time::{Duration, Time};

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

pub fn write_to_csv<T: Serialize>(file_path: &str, record: T) -> Result<(), csv::Error> {
    // Check if the file already exists
    let path = Path::new(file_path);
    let file_exists = path.exists();
    // Open or create the file
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;
    // let file = OpenOptions::new().write(true).create(true).append(true).open(file_path)?;
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

// pub fn is_third_friday(date: i32) -> bool {
//     let date_str = date.to_string();
//     if date_str.len() != 8 {
//         return false; // Return false if the date format is not valid
//     }
//
//     // Parsing year, month, and day
//     let year = &date_str[0..4].parse::<i32>().unwrap();
//     let month = &date_str[4..6].parse::<u32>().unwrap();
//     let day = &date_str[6..8].parse::<u32>().unwrap();
//
//     // Create a NaiveDate object
//     match NaiveDate::from_ymd_opt(*year, *month, *day) {
//         Some(date) => {
//             // Check if it's a Friday
//             if date.weekday() == Weekday::Fri {
//                 // Check if it's the third Friday
//                 let first_day_of_month = NaiveDate::from_ymd(*year, *month, 1);
//                 let first_friday = match first_day_of_month.weekday() {
//                     Weekday::Mon => first_day_of_month + chrono::Duration::days(4),
//                     Weekday::Tue => first_day_of_month + chrono::Duration::days(3),
//                     Weekday::Wed => first_day_of_month + chrono::Duration::days(2),
//                     Weekday::Thu => first_day_of_month + chrono::Duration::days(1),
//                     Weekday::Fri => first_day_of_month,
//                     Weekday::Sat => first_day_of_month + chrono::Duration::days(6),
//                     Weekday::Sun => first_day_of_month + chrono::Duration::days(5),
//                 };
//                 return date == first_friday + chrono::Duration::days(14);
//             }
//             false
//         },
//         None => false, // Invalid date
//     }
// }

// pub fn is_third_friday(date: i32) -> bool {
//     let date_str = date.to_string();
//     if date_str.len() != 8 {
//         return false; // Return false if the date format is not valid
//     }
//
//     // Parsing year, month, and day
//     let year = &date_str[0..4].parse::<i32>().unwrap();
//     let month = &date_str[4..6].parse::<u32>().unwrap();
//     let day = &date_str[6..8].parse::<u32>().unwrap();
//
//     // Create a NaiveDate object
//     match NaiveDate::from_ymd_opt(*year, *month, *day) {
//         Some(date) => {
//             // Check if it's a Friday
//             if date.weekday() == Weekday::Fri {
//                 // Check if it's the third Friday
//                 let first_day_of_month = NaiveDate::from_ymd(*year, *month, 1);
//                 let first_friday = first_day_of_month
//                     + chrono::Duration::days((11 - first_day_of_month.weekday().num_days_from_monday() as i64) % 7);
//                 return date == first_friday + chrono::Duration::days(14);
//             }
//             false
//         },
//         None => false, // Invalid date
//     }
// }
