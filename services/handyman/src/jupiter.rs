use log::info;
use repository::repository::Repository;
use crate::yaml::tasks::JupiterTask;

// TODO: надо вынести это в отдельный пакет, а может даже крейт

#[derive(Clone)]
pub struct JupiterExecutor {
    pub repo: Repository,
    pub task: JupiterTask,
}

impl JupiterExecutor {
    pub fn new(repo: Repository, task: JupiterTask) -> Self {
        Self { repo, task }
    }

    pub async fn execute(&self) {
        info!("Running jupiter task: {:?}", self.task);
    }
}
