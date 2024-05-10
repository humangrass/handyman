use std::time::Duration;
use job_scheduler::{Job, JobScheduler};
use repository::repository::Repository;
use sqlx::PgPool;
use tokio::time::sleep;
use crate::bitget::executor::Executor;
use crate::jupiter::JupiterExecutor;
use crate::yaml::tasks::{Tasks};


pub struct Handyman {
    pub repo: Repository,
    pub tasks: Tasks,
}

impl Handyman {
    pub fn new(pool: PgPool, tasks: Tasks) -> Self {
        let repo = Repository::new(pool);
        Self { repo, tasks }
    }

    pub async fn run(&self) {
        // TODO: надо возвращать ошибку, если задач нет или не осталось
        let mut scheduler = JobScheduler::new();

        for task in self.tasks.bitget.iter() {
            let repo = self.repo.clone();
            let executor = Executor::new(repo, task.clone());
            let job = Job::new(task.cron.to_string().parse().unwrap(), move || {
                let executor = executor.clone();
                tokio::spawn(async move {
                    // TODO: надо чекать на ошибку и удалять все задачи этого типа из планировщика, если она есть
                    executor.execute().await
                });
            });
            scheduler.add(job);
        }

        for task in self.tasks.jupiter.iter() {
            let repo = self.repo.clone();
            let executor = JupiterExecutor::new(repo, task.clone());
            let job = Job::new(task.cron.to_string().parse().unwrap(), move || {
                let executor = executor.clone();
                tokio::spawn(async move {
                    // TODO: надо чекать на ошибку и удалять все задачи этого типа из планировщика, если она есть
                    executor.execute().await
                });
            });
            scheduler.add(job);
        }

        loop {
            scheduler.tick();
            sleep(Duration::from_millis(500)).await;
        }
    }
}
