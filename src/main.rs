mod enums;
mod utils;

use crate::utils::to_occ_contract;
use chrono::NaiveDate;
use clap::{Arg, Command};
use futures_util::{SinkExt, StreamExt};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WebSocketMessage};
use url::Url;

const BUFFER_SIZE: usize = 100;


fn handle_message(
    message: &str,
    trade_buffer: &Arc<Mutex<VecDeque<enums::TradeData>>>,
    quote_buffer: &Arc<Mutex<VecDeque<enums::QuoteData>>>,
    ohlc_buffer: &Arc<Mutex<VecDeque<enums::OhlcData>>>,
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
                let cntrct = to_occ_contract(
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
                    root: contract.root,
                    dte: daytoexp,
                    expiration: contract.expiration,
                    strike: contract.strike,
                    right: contract.right,
                    symbol: cntrct,
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

                let mut quote_buffer = quote_buffer.lock().unwrap();
                quote_buffer.push_back(quote_data);

                if quote_buffer.len() > BUFFER_SIZE {
                    let quote_data = quote_buffer.pop_front().unwrap();
                    utils::write_to_csv("quote.csv", quote_data)?;
                }
            }
        }
        "TRADE" => {
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

                let trade_data = enums::TradeData {
                    timestamp: datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
                    // security_type: contract.security_type,
                    root: contract.root,
                    dte: daytoexp,
                    expiration: contract.expiration,
                    strike: contract.strike,
                    right: contract.right,
                    symbol: cntrct,
                    size: trade.size,
                    price: trade.price,
                    exchange: trade.exchange,
                    sequence: trade.sequence,
                    condition: trade.condition,
                    ms_of_day: trade.ms_of_day,
                    date: trade.date,
                };

                let mut trade_buffer = trade_buffer.lock().unwrap();
                trade_buffer.push_back(trade_data);

                if trade_buffer.len() > BUFFER_SIZE {
                    let trade_data = trade_buffer.pop_front().unwrap();
                    utils::write_to_csv("trade.csv", trade_data)?;
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
                    root: contract.root,
                    dte: daytoexp,
                    expiration: contract.expiration,
                    strike: contract.strike,
                    right: contract.right,
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

                let mut ohlc_buffer = ohlc_buffer.lock().unwrap();
                ohlc_buffer.push_back(ohlc_data);

                if ohlc_buffer.len() > BUFFER_SIZE {
                    let ohlc_data = ohlc_buffer.pop_front().unwrap();
                    utils::write_to_csv("ohlc.csv", ohlc_data)?;
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
    let trade_buffer: Arc<Mutex<VecDeque<enums::TradeData>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    let quote_buffer: Arc<Mutex<VecDeque<enums::QuoteData>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    let ohlc_buffer: Arc<Mutex<VecDeque<enums::OhlcData>>> = Arc::new(Mutex::new(VecDeque::new()));

    let matches = Command::new("tdfirehose")
        .version("0.1.0")
        .about("A cross platform options data client.")
        .arg(
            Arg::new("ws_url")
                .short('u')
                .long("url")
                .value_name("URL")
                .takes_value(true),
        )
        .get_matches();

    let ws_url = matches
        .value_of("ws_url")
        .unwrap_or("ws://127.0.0.1:25520/v1/events");
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

                while let Some(message) = read.next().await {
                    match message {
                        Ok(WebSocketMessage::Text(text)) => {
                            let text_clone = text.clone();
                            let trade_buffer = Arc::clone(&trade_buffer);
                            let quote_buffer = Arc::clone(&quote_buffer);
                            let ohlc_buffer = Arc::clone(&ohlc_buffer);
                            tokio::spawn(async move {
                                if let Err(e) = handle_message(
                                    &text_clone,
                                    &trade_buffer,
                                    &quote_buffer,
                                    &ohlc_buffer,
                                ) {
                                    eprintln!("Error handling message: {}", e);
                                }
                            });
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
