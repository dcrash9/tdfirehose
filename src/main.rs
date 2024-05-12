mod enums;
mod stats;
mod utils;

use crate::utils::to_occ_contract;
use chrono::{NaiveDate, Utc};
use chrono_tz::America::New_York;
use clap::{Arg, Command};
use futures_util::{SinkExt, StreamExt};
use std::collections::{HashMap, VecDeque};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WebSocketMessage};
use url::Url;

// Constants
const BUFFER_SIZE: usize = 100;
const LOGFILE: &str = "log.csv";
const QUOTEFILE: &str = "quote.csv";
const TRADEFILE: &str = "trade.csv";
const OHLCFILE: &str = "ohlc.csv";
const AGGFILE: &str = "agg.csv";
// Constants



fn handle_message(
    message: &str,
    trade_buffer: &Arc<Mutex<VecDeque<enums::TradeData>>>,
    quote_buffer: &Arc<Mutex<VecDeque<enums::QuoteData>>>,
    ohlc_buffer: &Arc<Mutex<VecDeque<enums::OhlcData>>>,
    latest_quotes: &Arc<Mutex<HashMap<(String, i32, i64, String), enums::QuoteData>>>,
    latest_ohlc: &Arc<Mutex<HashMap<(String, i32, i64, String), enums::OhlcData>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let msg: enums::AppMessage = serde_json::from_str(message)?;

    match msg.header.r#type.as_str() {
        "QUOTE" => {
            if let (Some(contract), Some(quote)) = (msg.contract, msg.quote) {
                let datetime = utils::combine_date_time(quote.date, quote.ms_of_day.into())?;
                let expiration_date =
                    NaiveDate::parse_from_str(&contract.expiration.to_string(), "%Y%m%d")?;
                let daytoexp = expiration_date
                    .signed_duration_since(NaiveDate::parse_from_str(
                        &quote.date.to_string(),
                        "%Y%m%d",
                    )?)
                    .num_days();
                // let today = Utc::now().naive_utc();
                let contractsymbol = to_occ_contract(
                    &contract.root,
                    contract.expiration,
                    &contract.right,
                    contract.strike,
                )
                .unwrap()
                .to_string();

                let quote_data = enums::QuoteData {
                    timestamp: datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                    // security_type: contract.security_type,
                    root: contract.root.clone(),
                    dte: daytoexp,
                    expiration: contract.expiration,
                    strike: contract.strike,
                    right: contract.right.clone(),
                    symbol: contractsymbol,
                    bid_condition: quote.bid_condition,
                    bid_exchange: quote.bid_exchange,
                    bid_size: quote.bid_size,
                    bid: quote.bid,
                    ask: quote.ask,
                    ask_size: quote.ask_size,
                    ask_exchange: quote.ask_exchange,
                    ask_condition: quote.ask_condition,
                    ms_of_day: quote.ms_of_day,
                    date: quote.date,
                };

                let mut latest_quotes = latest_quotes.lock().unwrap();
                latest_quotes.insert(
                    (
                        contract.root.clone(),
                        contract.expiration,
                        contract.strike,
                        contract.right.clone(),
                    ),
                    quote_data.clone(),
                );

                // println!("\n{:?}\n", latest_quotes);
                // println!("{:?}\n", quote_data);

                let mut quote_buffer = quote_buffer.lock().unwrap();
                quote_buffer.push_back(quote_data);

                if quote_buffer.len() > BUFFER_SIZE {
                    let quote_data = quote_buffer.pop_front().unwrap();
                    utils::write_to_csv(QUOTEFILE, quote_data)?; // STOP saving them to CSV
                };
            }
        }
        "TRADE" => {
            let latest_quotes_clone = Arc::clone(&latest_quotes);
            let latest_ohlc_clone = Arc::clone(&latest_ohlc);
            if let (Some(contract), Some(trade)) = (msg.contract, msg.trade) {
                let datetime = utils::combine_date_time(trade.date, trade.ms_of_day.into())?;
                let expiration_date =
                    NaiveDate::parse_from_str(&contract.expiration.to_string(), "%Y%m%d")?;
                let daytoexp = expiration_date
                    .signed_duration_since(NaiveDate::parse_from_str(
                        &trade.date.to_string(),
                        "%Y%m%d",
                    )?)
                    .num_days();
                let cntrct = to_occ_contract(
                    &contract.root,
                    contract.expiration,
                    &contract.right,
                    contract.strike,
                )
                .unwrap()
                .to_string();

                let key = (
                    contract.root.clone(),
                    contract.expiration,
                    contract.strike,
                    contract.right.clone(),
                );

                // let latest_quotes_guard = latest_quotes_clone.lock().unwrap();
                // let x = latest_quotes_guard.get(&key);

                // let latest_ohlc_guard = latest_ohlc_clone.lock().unwrap();
                // let y = latest_ohlc_guard.get(&key);

                let x = match latest_quotes_clone.try_lock() {
                    Ok(guard) => guard.get(&key).cloned(),
                    Err(_) => None,
                };

                let y = match latest_ohlc_clone.try_lock() {
                    Ok(guard) => guard.get(&key).cloned(),
                    Err(_) => None,
                };

                let trade_data = enums::TradeData {
                    timestamp: datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                    // security_type: contract.security_type,
                    root: contract.root.clone(),
                    dte: daytoexp,
                    expiration: contract.expiration,
                    strike: contract.strike,
                    right: contract.right.clone(),
                    symbol: cntrct.clone(),
                    size: trade.size,
                    price: trade.price,
                    exchange: trade.exchange,
                    sequence: trade.sequence,
                    condition: trade.condition,
                    ms_of_day: trade.ms_of_day,
                    date: trade.date,
                };

                // println!("\n---------------------------------------------------------------------");
                // println!("TRADE: {:?}\nQUOTE: {:?} \n OHLC:{:?}", trade_data, x, y);
                // println!("---------------------------------------------------------------------");

                let agg_data = enums::AggData {
                    timestamp: datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                    // security_type: contract.security_type,
                    root: contract.root.to_string(),
                    dte: daytoexp,
                    expiration: contract.expiration,
                    strike: contract.strike,
                    right: contract.right.to_string(),
                    symbol: cntrct.to_string(),
                    size: trade.size,
                    price: trade.price,
                    exchange: trade.exchange,
                    sequence: trade.sequence,
                    condition: trade.condition,
                    // QUOTES DATA
                    bid_condition: x.as_ref().map_or(0, |q| q.bid_condition.clone()),
                    bid_exchange: x.as_ref().map_or(0, |q| q.bid_exchange),
                    bid_size: x.as_ref().map_or(0, |q| q.bid_size),
                    bid: x.as_ref().map_or(0.0, |q| q.bid),
                    ask: x.as_ref().map_or(0.0, |q| q.ask),
                    ask_size: x.as_ref().map_or(0, |q| q.ask_size),
                    ask_exchange: x.as_ref().map_or(0, |q| q.ask_exchange),
                    ask_condition: x.as_ref().map_or(0, |q| q.ask_condition.clone()),
                    // OHLC DATA
                    open: y.as_ref().map_or(0.0, |o| o.open),
                    high: y.as_ref().map_or(0.0, |o| o.high),
                    low: y.as_ref().map_or(0.0, |o| o.low),
                    close: y.as_ref().map_or(0.0, |o| o.close),
                    volume: y.as_ref().map_or(0, |o| o.volume),
                    count: y.as_ref().map_or(0, |o| o.count),
                    //
                    ms_of_day: trade.ms_of_day,
                    date: trade.date,
                };
                // println!("\n---------------------------------------------------------------------");
                // println!("AGG DATA: {:?}", agg_data);
                // println!("---------------------------------------------------------------------");
                utils::write_to_csv(AGGFILE, agg_data)?;

                let mut trade_buffer = trade_buffer.lock().unwrap();
                trade_buffer.push_back(trade_data);

                if trade_buffer.len() > BUFFER_SIZE {
                    let trade_data = trade_buffer.pop_front().unwrap();
                    utils::write_to_csv(TRADEFILE, trade_data)?;
                }
            }
        }
        "OHLC" => {
            if let (Some(contract), Some(ohlc)) = (msg.contract, msg.ohlc) {
                let datetime = utils::combine_date_time(ohlc.date, ohlc.ms_of_day.into())?;
                let expiration_date =
                    NaiveDate::parse_from_str(&contract.expiration.to_string(), "%Y%m%d")?;
                let daytoexp = expiration_date
                    .signed_duration_since(NaiveDate::parse_from_str(
                        &ohlc.date.to_string(),
                        "%Y%m%d",
                    )?)
                    .num_days();
                let cntrct = to_occ_contract(
                    &contract.root,
                    contract.expiration,
                    &contract.right,
                    contract.strike,
                )
                .unwrap()
                .to_string();

                let ohlc_data = enums::OhlcData {
                    timestamp: datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                    // security_type: contract.security_type,
                    root: contract.root.clone(),
                    dte: daytoexp,
                    expiration: contract.expiration,
                    strike: contract.strike,
                    right: contract.right.clone(),
                    symbol: cntrct,
                    open: ohlc.open,
                    high: ohlc.high,
                    low: ohlc.low,
                    close: ohlc.close,
                    volume: ohlc.volume,
                    count: ohlc.count,
                    ms_of_day: ohlc.ms_of_day,
                    date: ohlc.date,
                };

                let mut latest_ohlc = latest_ohlc.lock().unwrap();
                latest_ohlc.insert(
                    (
                        contract.root.clone(),
                        contract.expiration,
                        contract.strike,
                        contract.right.clone(),
                    ),
                    ohlc_data.clone(),
                );

                let mut ohlc_buffer = ohlc_buffer.lock().unwrap();
                ohlc_buffer.push_back(ohlc_data);

                if ohlc_buffer.len() > BUFFER_SIZE {
                    let ohlc_data = ohlc_buffer.pop_front().unwrap();
                    utils::write_to_csv(OHLCFILE, ohlc_data)?;
                }
            }
        }
        _ => (),
        // _ => println!("Unhandled message type: {}", msg.header.r#type),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create buffers to store messages
    let ohlc_buffer: Arc<Mutex<VecDeque<enums::OhlcData>>> = Arc::new(Mutex::new(VecDeque::new()));
    let quote_buffer: Arc<Mutex<VecDeque<enums::QuoteData>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    let trade_buffer: Arc<Mutex<VecDeque<enums::TradeData>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    let latest_quotes: Arc<Mutex<HashMap<(String, i32, i64, String), enums::QuoteData>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let latest_ohlc: Arc<Mutex<HashMap<(String, i32, i64, String), enums::OhlcData>>> =
        Arc::new(Mutex::new(HashMap::new()));

    // Define a counter for the number of messages received and processed
    let mut message_count: u32 = 0;

    let matches = Command::new("tdfirehose")
        .version("0.1.3")
        .about("A cross platform options data client.")
        .arg(
            Arg::new("ws_url")
                .short('u')
                .long("url")
                .default_value("ws://127.0.0.1:25520/v1/events")
                .value_name("URL")
                .takes_value(true),
        )
        .arg(Arg::new("debug").short('d').long("debug").takes_value(true))
        .get_matches();

    let ws_url = matches
        .value_of("ws_url")
        .unwrap_or("ws://10.0.0.5:8080/v1/events");
    // let url = Url::parse("ws://10.0.0.5:8080/v1/events")?;
    // let url = Url::parse("ws://127.0.0.1:25520/v1/events")?;

    let url = Url::parse(ws_url)?;

    loop {
        match connect_async(url.clone()).await {
            Ok((ws_stream, _)) => {
                println!("Connected to: {}", ws_url);
                let (mut write, mut read) = ws_stream.split();

                let subscribe_msg = WebSocketMessage::Text(
                    r#"{
                    "msg_type": "STREAM_BULK",
                    "sec_type": "OPTION",
                    "req_type": "TRADE",
                    "add": true,
                    "id": 0
                }"#
                    .to_string(),
                );

                write.send(subscribe_msg).await?;

                let start_time = Instant::now(); // for logging statistics
                let mut last_log_time = Utc::now().with_timezone(&New_York); // for logging statistics

                // let average_delay = 0; // for logging statistics // unused you say ???
                let mut total_delay = 0;
                let mut message_count_with_delay = 0;

                while let Some(message) = read.next().await {
                    match message {
                        Ok(WebSocketMessage::Text(text)) => {
                            let text_clone = text.clone();
                            let trade_buffer = Arc::clone(&trade_buffer);
                            let quote_buffer = Arc::clone(&quote_buffer);
                            let ohlc_buffer = Arc::clone(&ohlc_buffer);
                            let latest_quotes = Arc::clone(&latest_quotes);
                            let latest_ohlc = Arc::clone(&latest_ohlc);
                            tokio::spawn(async move {
                                if let Err(e) = handle_message(
                                    &text_clone,
                                    &trade_buffer,
                                    &quote_buffer,
                                    &ohlc_buffer,
                                    &latest_quotes,
                                    &latest_ohlc,
                                ) {
                                    eprintln!("Error handling message: {}", e);
                                }
                            });

                            // -> Logging the statistics
                            if !Path::new(LOGFILE).exists() {
                                OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .truncate(true)
                                    .open(LOGFILE)?;
                                let header = "timestamp,msg_count,avg_msg_per_sec,avg_msg_per_min,avg_delay\n";
                                let mut file =
                                    OpenOptions::new().create(true).write(true).open(LOGFILE)?;
                                file.write_all(header.as_bytes())?;
                            }

                            message_count += 1;

                            // let mydatetime = Utc::now().with_timezone(&New_York);
                            let mut delay = 0; // unused you say ???

                            // println!("Received message: {}", text);
                            let mytestmsg: enums::AppMessage = serde_json::from_str(&text)?;

                            if mytestmsg.header.r#type == "QUOTE" {
                                // Check if the quote object is present
                                if let Some(quote) = mytestmsg.quote {
                                    // Access the ms_of_day and date fields of the quote object
                                    let ms_of_day = quote.ms_of_day;
                                    let date = quote.date;

                                    // Print the ms_of_day and date
                                    let mydatetime =
                                        utils::combine_date_time(date, ms_of_day.into())?
                                            .with_timezone(&New_York);
                                    let mynow = Utc::now().with_timezone(&New_York);
                                    delay = mynow.signed_duration_since(mydatetime).num_seconds();
                                    total_delay += delay;
                                    message_count_with_delay += 1;
                                    // println!("\ntimestamp message: {} | timestamp local: {} | Delay: {} seconds", mydatetime, mynow, delay);
                                }
                            }

                            // Calculate the elapsed time in seconds and minutes
                            let elapsed_seconds = start_time.elapsed().as_secs();
                            let elapsed_minutes = elapsed_seconds / 60;
                            // Calculate the average number of messages per second and per minute
                            let avg_per_second = message_count as f64 / elapsed_seconds as f64;
                            let avg_per_minute = message_count as f64 / elapsed_minutes as f64;
                            // Get the current time
                            let now = Utc::now().with_timezone(&New_York);

                            let average_delay =
                                total_delay as f64 / message_count_with_delay as f64;

                            // If a minute has passed since the last log
                            if (now - last_log_time).num_minutes() >= 1 {
                                let average_delay =
                                    total_delay as f64 / message_count_with_delay as f64;

                                // Write the statistics to the log
                                let mut file =
                                    OpenOptions::new().append(true).open(LOGFILE).unwrap();

                                if let Err(e) = writeln!(
                                    file,
                                    "{},{},{:.2},{:.2},{:.2}",
                                    now,
                                    message_count,
                                    avg_per_second,
                                    avg_per_minute,
                                    average_delay
                                ) {
                                    eprintln!("Couldn't write to file: {}", e);
                                }
                                last_log_time = now; // never used ???
                            }
                            print!("\rReceived messages: {} [~ {:.2} / sec. ≡ ~ {:.2} / min.] [~ delay: {:.2} sec.]", message_count, avg_per_second, avg_per_minute, average_delay);

                            std::io::stdout().flush().unwrap();
                            // <- Logging the statistics
                        }
                        Ok(WebSocketMessage::Binary(_)) => println!("Received binary data"),
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("Error reading message: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error connecting to WebSocket: {}", e);
            }
        }

        // Wait before attempting to reconnect
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
