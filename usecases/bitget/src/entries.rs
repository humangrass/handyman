use serde::{Deserialize};
use sqlx::types::chrono::{NaiveDateTime, TimeZone, Utc};
use models::candle;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ResponseCandleData {
    pub open_time: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub base_asset_volume: String,
    pub quote_asset_volume: String,
    pub usdt_volume: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub code: String,
    pub msg: String,
    #[serde(rename = "requestTime")]
    pub request_time: i64,
    pub data: Vec<Vec<String>>,
}

impl ApiResponse {
    pub fn new(json_string: String) -> anyhow::Result<ApiResponse> {
        parse_response(&json_string)
    }

    pub fn parse_candle_data(&self) -> Vec<ResponseCandleData> {
        self.data.iter().map(|item| {
            ResponseCandleData {
                open_time: item[0].clone(),
                open: item[1].clone(),
                high: item[2].clone(),
                low: item[3].clone(),
                close: item[4].clone(),
                base_asset_volume: item[5].clone(),
                quote_asset_volume: item[6].clone(),
                usdt_volume: item[7].clone(),
            }
        }).collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Candle {
    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub base_asset_volume: f64,
    pub quote_asset_volume: f64,
    pub usdt_volume: f64,
}

impl Candle {
    pub fn new(item: ResponseCandleData) -> Candle {
        Self {
            open_time: item.open_time.parse().unwrap(),
            open: item.open.parse().unwrap_or(0.0),
            high: item.high.parse().unwrap_or(0.0),
            low: item.low.parse().unwrap_or(0.0),
            close: item.close.parse().unwrap_or(0.0),
            base_asset_volume: item.base_asset_volume.parse().unwrap_or(0.0),
            quote_asset_volume: item.quote_asset_volume.parse().unwrap_or(0.0),
            usdt_volume: item.usdt_volume.parse().unwrap_or(0.0),
        }
    }
    pub fn model(&self, exchange: String, symbol: String, timeframe: String) -> candle::Candle {
        candle::Candle {
            uuid: Uuid::new_v4(),
            exchange,
            symbol,
            timeframe,
            open_time: millis_to_datetime(self.open_time),
            open: self.open,
            high: Option::from(self.high),
            low: Option::from(self.low),
            close: Option::from(self.close),
            base_asset_volume: Option::from(self.base_asset_volume),
            quote_asset_volume: Option::from(self.quote_asset_volume),
            usdt_volume: Option::from(self.usdt_volume),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Candles {
    pub candles: Vec<Candle>,
}


impl Candles {
    pub fn new(response: ApiResponse) -> Self {
        let mut candles: Vec<Candle> = Vec::new();
        for candle_data in response.parse_candle_data() {
            candles.push(Candle::new(candle_data));
        }
        Self { candles }
    }

    pub fn models(&self, exchange: String, symbol: String, timeframe: String) -> Vec<candle::Candle> {
        let mut candles: Vec<candle::Candle> = Vec::new();
        for candle in &self.candles {
            candles.push(candle.model(exchange.clone(), symbol.clone(), timeframe.clone()));
        }
        candles
    }
}

fn parse_response(json_string: &str) -> anyhow::Result<ApiResponse> {
    let response: ApiResponse = serde_json::from_str(json_string)?;
    Ok(response)
}

fn millis_to_datetime(millis: i64) -> NaiveDateTime {
    let seconds = millis / 1000;
    let nanos = ((millis % 1000) * 1_000_000) as u32;
    Utc.timestamp_opt(seconds, nanos).unwrap().naive_utc()
}