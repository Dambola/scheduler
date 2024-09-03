use config::Configuration;
use database::get_postgres_pool;
use queue::{insert_queued_job, CreateQueuedJob};

mod config;
mod database;
mod queue;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() {
    // Load the configurations
    let config = Configuration::new().expect("Error loading the configurations.");
    
    let pool = get_postgres_pool(&config)
        .await
        .unwrap();

    let new_job = CreateQueuedJob{
        priority: 0,
        task: "task".to_string(),
        metadata: None,
    };

    let queued_job = insert_queued_job(&pool, new_job)
        .await
        .unwrap();
    println!("{:?}", queued_job);
}