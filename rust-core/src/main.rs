// rust-core/src/main.rs
mod file_indexer;
mod skipcompare;
mod incremental;

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

    let scan_path = match file_indexer::get_scan_path(&pool).await {
        Ok(path) => path,
        Err(e) => {
            eprintln!("❌ Error getting scan path: {}", e);
            return Err(e);
        }
    };
    
    // Ask user if they want incremental scan
    println!("🔍 Do you want an incremental scan? (y/n)");
    print!("> ");
    std::io::Write::flush(&mut std::io::stdout())?;
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let incremental = input.trim().eq_ignore_ascii_case("y");
    
    if incremental {
        println!("🔄 Running incremental scan...");
    } else {
        println!("🔄 Running full scan...");
    }
    
    match file_indexer::scan_and_store(&pool, scan_path, incremental).await {
        Ok(()) => {
            println!("✅ File scan complete. Data saved to '{}'.", db_path);
        }
        Err(e) => {
            eprintln!("❌ Error during scan: {}", e);
            return Err(e);
        }
    }

    Ok(())
}