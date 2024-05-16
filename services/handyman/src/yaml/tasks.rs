use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BitgetOpt {
    pub cron: String,
    pub symbol: String,
    pub granularity: String,
    pub limit: u16,
}

impl BitgetOpt {
    pub fn task(self) -> bitget::executor::Task {
        bitget::executor::Task {
            cron: self.cron,
            symbol: self.symbol,
            granularity: self.granularity,
            limit: self.limit,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JupiterOpt {
    pub cron: String,
    pub symbol: String,
    pub timeframe: String,
}

impl JupiterOpt {
    pub fn task(self) -> jupiter::Task {
        jupiter::Task {
            cron: self.cron,
            symbol: self.symbol,
            timeframe: self.timeframe,
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Tasks {
    pub(crate) bitget: Vec<BitgetOpt>,
    pub(crate) jupiter: Vec<JupiterOpt>,
}

impl Tasks {
    pub(crate) fn new(file_path: &Path) -> anyhow::Result<Tasks> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let tasks: Tasks = serde_yaml::from_reader(reader)?;

        Ok(tasks)
    }
}
