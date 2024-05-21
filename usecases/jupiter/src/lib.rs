use log::info;
use repository::repository::Repository;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub cron: String,
    pub symbol: String,
    pub timeframe: String,
}

#[derive(Clone)]
pub struct JupiterExecutor {
    pub repo: Repository,
    pub task: Task,
}

impl JupiterExecutor {
    pub fn new(repo: Repository, task: Task) -> Self {
        Self { repo, task }
    }

    pub async fn execute(&self) -> anyhow::Result<()> {
        info!("Running jupiter task: {:?}", self.task);
        Ok(())
    }
}
