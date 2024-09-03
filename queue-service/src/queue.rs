use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Error, FromRow, PgPool, Row};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateQueuedJob {
    pub priority: i64,
    pub task: String,
    pub metadata: Option<Value>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct QueuedJob {
    id: Uuid,
    priority: i64,
    task: String,
    metadata: Option<Value>,
    created_at: DateTime<Utc>
}

pub async fn insert_queued_job(pool: &PgPool, queued_job: CreateQueuedJob) -> Result<QueuedJob, Error> {
    let query_result = sqlx::query(
            "INSERT INTO queue (priority, task, metadata) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(queued_job.priority)
        .bind(queued_job.task)
        .bind(queued_job.metadata)
        .fetch_one(pool)
        .await;

    match query_result {
        Ok(row) => {
            let id = row.try_get("id").expect("No 'id' found on PgRow");
            get_queued_job_by_id(&pool, id).await
        }
        Err(e) => Err(e)
    }
}

pub async fn get_queued_job_by_id(pool: &PgPool, id: Uuid) -> Result<QueuedJob, Error> {
    sqlx::query_as(
            "SELECT * FROM queue WHERE id = $1",
        )
        .bind(id)
        .fetch_one(pool)
        .await
}