// rust-core/src/main.rs
mod file_indexer;
mod skipcompare;

use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::{error::Error, fs, path::Path, str::FromStr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_path = "index.db";

    if !Path::new(db_path).exists() {
        fs::File::create(db_path)?;
        println!("🆕 Created new DB file: {}", db_path);
    }

    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", db_path))?
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    file_indexer::init_db(&pool).await?;

    let scan_path = file_indexer::get_scan_path(&pool).await?;
    file_indexer::scan_and_store(&pool, scan_path).await?;

    println!("✅ File scan complete. Data saved to '{}'.", db_path);
    Ok(())
}
