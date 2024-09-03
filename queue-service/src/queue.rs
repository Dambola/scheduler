use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Error, FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateQueuedJob {
    priority: i64,
    task: String,
    metadata: Option<Value>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct QueuedJob {
    id: Uuid,
    priority: i64,
    task: String,
    metadata: Option<Value>,
    created_at: DateTime<Utc>
}

pub async fn push_to_queue(pool: &PgPool, queued_job: &CreateQueuedJob) -> Result<QueuedJob, Error> {
    let query_result = sqlx::query_as!(
        QueuedJob,
        "INSERT INTO queue (priority, task, metadata) VALUES ($1, $2, $3) RETURNING id",
        queued_job.priority,
        queued_job.task.to_string(),
        queued_job.metadata
    )
    .fetch_one(&pool)
    .await;

    match query_result {
        Ok(created_queued_job) => {
            return Ok(created_queued_job);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

impl QueueService {
    pub async fn pop(client: &tokio_postgres::Client) -> Result<Option<QueuedJob>, Error> {
        let row = client
            .query_opt(
                "DELETE FROM queue 
                 WHERE id = (
                     SELECT id 
                     FROM queue 
                     ORDER BY priority ASC, created_at ASC 
                     LIMIT 1
                 ) 
                 RETURNING id, priority, task, metadata, created_at",
                &[],
            )
            .await?;
        
        if let Some(row) = row {
            Ok(Some(QueuedJob::from(row)))
        } else {
            Ok(None)
        }
    }

    pub async fn find(client: &tokio_postgres::Client, task: String) -> Result<Vec<QueuedJob>, Error> {
        let rows = client
            .query(
                "SELECT id, task, priority, metadata, created_at FROM queue WHERE task = $1", 
                &[&task])
            .await?;
        
        let result: Vec<QueuedJob> = rows.into_iter().map(QueuedJob::from).collect();
        Ok(result)
    }

    pub async fn remove(client: &tokio_postgres::Client, task: String) -> Result<(), Error> {
        client
            .execute("DELETE FROM queue WHERE task = $1", &[&task])
            .await?;
        Ok(())
    }
}
