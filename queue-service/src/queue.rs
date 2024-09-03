use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Error, FromRow, PgPool, Row};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateQueuedJob {
    pub priority: i64,
    pub parent: Uuid,
    pub metadata: Option<Value>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct QueuedJob {
    id: Uuid,
    priority: i64,
    parent: Uuid,
    metadata: Option<Value>,
    created_at: DateTime<Utc>
}

pub async fn create_queued_job(pool: &PgPool, queued_job: CreateQueuedJob) -> Result<QueuedJob, Error> {
    let query_result = sqlx::query(
            "INSERT INTO queue (priority, task, metadata) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(queued_job.priority)
        .bind(queued_job.parent)
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

pub async fn pop_queued_job(pool: &PgPool, batch_size: i64) -> Result<Vec<QueuedJob>, Error> {
    let query_result = sqlx::query(
            "DELETE FROM queue WHERE id IN (SELECT id FROM queue ORDER BY priority, created_at LIMIT $1) RETURNING *",
        )
        .bind(batch_size)
        .fetch_all(pool)
        .await;

    match query_result {
        Ok(rows) => {
            let queued_jobs = rows.into_iter()
                .map(|r| QueuedJob::from_row(&r).unwrap())
                .collect();
            Ok(queued_jobs)
        }
        Err(e) => Err(e)
    }
}

pub async fn delete_queued_jobs_by_parent(pool: &PgPool, parent: Uuid) -> Result<Vec<QueuedJob>, Error> {
    let query_result = sqlx::query(
            "DELETE FROM queue WHERE parent = $1 RETURNING *",
        )
        .bind(parent)
        .fetch_all(pool)
        .await;

    match query_result {
        Ok(rows) => {
            let queued_jobs = rows.into_iter()
                .map(|r| QueuedJob::from_row(&r).unwrap())
                .collect();
            Ok(queued_jobs)
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
