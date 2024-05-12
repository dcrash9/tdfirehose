use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppMessage {
    pub header: Header,
    pub contract: Option<Contract>,
    pub quote: Option<Quote>,
    pub ohlc: Option<Ohlc>,
    pub trade: Option<Trade>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub r#type: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contract {
    pub security_type: String,
    pub root: String,
    pub expiration: i32,
    pub strike: i64,
    pub right: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    pub bid_condition: u8,
    pub bid_exchange: u8,
    pub bid_size: u32,
    pub bid: f64,
    pub ask: f64,
    pub ask_size: u32,
    pub ask_exchange: u8,
    pub ask_condition: u8,
    pub ms_of_day: u32,
    pub date: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ohlc {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u32,
    pub count: u32,
    pub ms_of_day: u32,
    pub date: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub sequence: i64,
    pub size: u32,
    pub price: f64,
    pub exchange: u8,
    pub condition: u8,
    pub ms_of_day: u32,
    pub date: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuoteData {
    pub timestamp: String,
    // pub security_type: String,
    pub root: String,
    pub dte: i64,
    pub expiration: i32,
    // pub strike: f64,
    pub strike: i64,
    pub right: String,
    pub symbol: String,
    pub bid_condition: u8,
    pub bid_exchange: u8,
    pub bid_size: u32,
    pub bid: f64,
    pub ask: f64,
    pub ask_size: u32,
    pub ask_exchange: u8,
    pub ask_condition: u8,
    pub ms_of_day: u32,
    pub date: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OhlcData {
    pub timestamp: String,
    // pub security_type: String,
    pub root: String,
    pub dte: i64,
    pub expiration: i32,
    pub strike: i64,
    pub right: String,
    pub symbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u32,
    pub count: u32,
    pub ms_of_day: u32,
    pub date: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeData {
    pub timestamp: String,
    // pub security_type: String,
    pub root: String,
    pub dte: i64,
    pub expiration: i32,
    pub strike: i64,
    pub right: String,
    pub symbol: String,
    pub size: u32,
    pub price: f64,
    pub exchange: u8,
    pub sequence: i64,
    pub condition: u8,
    pub ms_of_day: u32,
    pub date: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AggData {
    pub timestamp: String,
    // pub security_type: String,
    pub root: String,
    pub dte: i64,
    pub expiration: i32,
    pub strike: i64,
    pub right: String,
    pub symbol: String,
    pub size: u32,
    pub price: f64,
    pub exchange: u8,
    pub sequence: i64,
    pub condition: u8,
    // QUOTES DATA
    pub bid_condition: u8,
    pub bid_exchange: u8,
    pub bid_size: u32,
    pub bid: f64,
    pub ask: f64,
    pub ask_size: u32,
    pub ask_exchange: u8,
    pub ask_condition: u8,
    // OHLC DATA
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u32,
    pub count: u32,
    pub ms_of_day: u32,
    pub date: i32,
}