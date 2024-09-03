use sqlx::{Error, PgPool};
use crate::config::Configuration;

pub async fn get_postgres_pool(config: &Configuration) -> Result<PgPool, Error> {
    let url = get_database_url(config);
    PgPool::connect(&url).await
}

fn get_database_url(config: &Configuration) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database.username,
        config.database.password,
        config.database.host,
        config.database.port,
        config.database.database
    )
}
