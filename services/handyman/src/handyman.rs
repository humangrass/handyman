use std::time::Duration;
use job_scheduler::{Job, JobScheduler};
use log::error;
use repository::repository::Repository;
use sqlx::PgPool;
use tokio::time::sleep;
use bitget::executor::BitgetExecutor;
use jupiter::JupiterExecutor;
use crate::yaml::tasks::{Tasks};


// It would be better to use traits instead :D
macro_rules! add_jobs {
    ($self:ident, $scheduler:ident, $tasks:expr, $executor:ty) => {
        for task in $tasks.iter() {
            let repo = $self.repo.clone();
            let executor = <$executor>::new(repo, task.clone().task());
            let job = Job::new(task.cron.to_string().parse().unwrap(), move || {
                let executor = executor.clone();
                tokio::spawn(async move {
                    if let Err(e) = executor.execute().await {
                        error!("Error executing task: {:?}\n{:?}", executor.task, e);
                    }
                });
            });
            $scheduler.add(job);
        }
    };
}

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
        let mut scheduler = JobScheduler::new();

        add_jobs!(self, scheduler, self.tasks.bitget, BitgetExecutor);
        add_jobs!(self, scheduler, self.tasks.jupiter, JupiterExecutor);

        loop {
            scheduler.tick();
            sleep(Duration::from_millis(500)).await;
        }
    }
}
