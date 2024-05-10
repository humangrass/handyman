use chrono::NaiveDateTime;
use sqlx::types::uuid;
use uuid::Uuid;

#[derive(Debug)]
pub struct Candle {
    pub uuid: Uuid,
    pub exchange: String,
    pub symbol: String,
    pub timeframe: String,
    pub open_time: NaiveDateTime,
    pub open: f64,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub base_asset_volume: Option<f64>,
    pub quote_asset_volume: Option<f64>,
    pub usdt_volume: Option<f64>,
}
