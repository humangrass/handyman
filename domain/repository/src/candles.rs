use models::candle::Candle;
use crate::repository::Repository;
use sqlx::{Error};
use sqlx::postgres::PgQueryResult;

impl Repository {
    pub async fn insert_candle(&self, candle: Candle) -> Result<PgQueryResult, Error> {
        sqlx::query(
            "INSERT INTO candles (uuid, exchange, symbol, timeframe, open_time, open, high, low, close, asset_volume, quote_asset_volume, usdt_volume) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"
        )
            .bind(&candle.uuid)
            .bind(candle.exchange)
            .bind(candle.symbol)
            .bind(candle.timeframe)
            .bind(candle.open_time)
            .bind(candle.open)
            .bind(candle.high)
            .bind(candle.low)
            .bind(candle.close)
            .bind(candle.base_asset_volume)
            .bind(candle.quote_asset_volume)
            .bind(candle.usdt_volume)
            .execute(self.pool())
            .await
    }
}
