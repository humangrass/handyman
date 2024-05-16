use std::string::ToString;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use repository::repository::Repository;
use crate::entries::{ApiResponse, Candles};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub cron: String,
    pub symbol: String,
    pub granularity: String,
    pub limit: u16,
}

#[derive(Clone)]
pub struct BitgetExecutor {
    pub repo: Repository,
    pub task: Task,
}

const BITGET: &str = "BITGET";

impl BitgetExecutor {
    pub fn new(repo: Repository, task: Task) -> Self {
        Self { repo, task }
    }

    pub async fn execute(&self) {
        // TODO: надо возвращать anyhow::Result
        info!("Running bitget task: {:?}", self.task);
        let candles = self.request_candles().await.expect("An error occurred while requesting candles");
        // TODO: можно сделать чек c ошибкой при len > 1
        let candle = candles.candles.get(0).unwrap();
        let candle_record = candle.model(BITGET.to_string(), self.task.symbol.clone(), self.task.granularity.clone());
        self.repo.insert_candle(candle_record).await.expect("An error occurred while sending data to the database");
        debug!("{:?}", candle);
    }

    async fn request_candles(&self) -> anyhow::Result<Candles> {
        let (start_time, end_time) = self.compute_time_range();

        let url = format!("https://api.bitget.com/api/v2/spot/market/candles?symbol={}&granularity={}&startTime={}&endTime={}&limit={}",
                          self.task.symbol, self.task.granularity, start_time, end_time, self.task.limit);
        debug!("{:?}", url);

        let response = reqwest::get(&url).await?;
        let body = response.text().await?;

        let api_response = ApiResponse::new(body).expect("Error parsing response");
        Ok(Candles::new(api_response))
    }

    fn compute_time_range(&self) -> (i64, i64) {
        // TODO: сейчас только для 1min
        let now = Utc::now();
        let start_time = now.timestamp_millis() - (now.timestamp_millis() % (60 * 1000)) - (60 * 1000);
        let end_time = now.timestamp_millis() - (now.timestamp_millis() % (60 * 1000));

        (start_time, end_time)
    }
}
