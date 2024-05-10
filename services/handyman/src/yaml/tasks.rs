use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct BitgetTask {
    pub(crate) cron: String,
    pub(crate) symbol: String,
    pub(crate) granularity: String,
    pub(crate) limit: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct JupiterTask {
    pub(crate) cron: String,
    pub(crate) symbol: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Tasks {
    pub(crate) bitget: Vec<BitgetTask>,
    pub(crate) jupiter: Vec<JupiterTask>,
}

impl Tasks {
    pub(crate) fn new(file_path: &Path) -> anyhow::Result<Tasks> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let tasks: Tasks = serde_yaml::from_reader(reader)?;

        Ok(tasks)
    }
}
