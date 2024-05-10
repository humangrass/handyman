mod yaml;
mod handyman;
mod bitget;
mod jupiter;

use std::path::Path;
use std::process;
use database::postgres::new_postgres_pool;
use logger::tracer_logger::new_tracer_logger;
use crate::handyman::Handyman;
use crate::yaml::{config};
use crate::yaml::tasks::{Tasks};


#[tokio::main]
async fn main() {
    println!("Handyman service started!");
    if let Err(err) = run().await {
        eprintln!("Fatal error occurred: {}", err);
        process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    // TODO: нужен cli
    new_tracer_logger(logger::tracer_logger::LogLevel::Info);

    let tasks = Tasks::new(Path::new("tasks.yaml")).expect("Failed to load tasks");
    let config = config::HandymanConfig::new(Path::new("config.yaml")).expect("Failed to load config");
    let pool = new_postgres_pool(config.database)
        .await
        .expect("🪂 Failed to create Postgres pool");

    let handyman = Handyman::new(pool, tasks);
    handyman.run().await;

    Ok(())
}
