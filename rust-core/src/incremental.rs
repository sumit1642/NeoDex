// rust-core/src/incremental.rs
use std::time::SystemTime;
use sqlx::SqlitePool;

/// Check if a file needs to be rescanned based on its modification time
pub async fn needs_rescan(
    pool: &SqlitePool,
    path: &str,
    current_modified: SystemTime,
) -> Result<bool, sqlx::Error> {
    let record: Option<(i64,)> = sqlx::query_as(
        "SELECT modified FROM files WHERE path = ?"
    )
    .bind(path)
    .fetch_optional(pool)
    .await?;

    match record {
        Some((db_modified,)) => {
            let db_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(db_modified as u64);
            Ok(current_modified > db_time)
        }
        None => Ok(true), // New file
    }
}