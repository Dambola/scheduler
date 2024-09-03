use config::Configuration;
use database::get_postgres_pool;
use queue::{create_queued_job, CreateQueuedJob};
use uuid::Uuid;

mod config;
mod database;
mod queue;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() {
    // Load the configurations
    let config = Configuration::new()
        .expect("Error loading the configurations");
    
    // Get postgres pool
    let pool = get_postgres_pool(&config)
        .await
        .unwrap();

    // Create queued job
    let new_job = CreateQueuedJob{
        priority: 0,
        parent: Uuid::new_v4(),
        metadata: None,
    };
    let queued_job = create_queued_job(&pool, new_job)
        .await
        .unwrap();
    println!("{:?}", queued_job);
}