use std::string::ToString;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use repository::repository::Repository;
use crate::entries::{ApiResponse, Candles};
use crate::granularity::Granularity;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub cron: String,
    pub symbol: String,
    pub granularity: Granularity,
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

    pub async fn execute(&self) -> anyhow::Result<()> {
        info!("Running bitget task: {:?}", self.task);
        let candles = self.request_candles().await.expect("An error occurred while requesting candles");

        if candles.candles.len() > 1 {
            return Err(anyhow::anyhow!("Expected only one candle, but got multiple"));
        }

        let candle = candles.candles.get(0).ok_or_else(|| anyhow::anyhow!("Candles not found"))?;
        let granularity = self.task.granularity.clone().as_str().to_string();
        let candle_record = candle.model(BITGET.to_string(), self.task.symbol.clone(), granularity);
        self.repo.insert_candle(candle_record).await.expect("An error occurred while sending data to the database");
        debug!("{:?}", candle);
        
        Ok(())
    }

    async fn request_candles(&self) -> anyhow::Result<Candles> {
        let (start_time, end_time) = self.compute_time_range();

        let url = format!(
            "https://api.bitget.com/api/v2/spot/market/candles?symbol={}&granularity={}&startTime={}&endTime={}&limit={}",
            self.task.symbol,
            self.task.granularity.as_str(),
            start_time,
            end_time,
            self.task.limit,
        );
        debug!("{:?}", url);

        let response = reqwest::get(&url).await?;
        let body = response.text().await?;
        let api_response = ApiResponse::new(body).expect("Error parsing response");

        Ok(Candles::new(api_response))
    }

    fn compute_time_range(&self) -> (i64, i64) {
        let now = Utc::now();
        let granularity_millis = self.task.granularity.as_millis();

        let start_time = now.timestamp_millis() - (now.timestamp_millis() % granularity_millis) - granularity_millis;
        let end_time = now.timestamp_millis() - (now.timestamp_millis() % granularity_millis);

        (start_time, end_time)
    }
}
