// rust-core/src/file_indexer.rs
use indicatif::{ProgressBar, ProgressStyle};
use sqlx::SqlitePool;
use std::{
    collections::HashSet,
    fs::Metadata,
    io::{self, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::fs;
use walkdir::WalkDir;

use crate::skipcompare::{is_blacklisted, load_blacklisted_folders};

/// Format file permissions into rwxr-xr-- style (Unix-only)
fn format_permissions(metadata: &Metadata) -> String {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        let mut result = String::new();
        for i in 0..9 {
            let bit = 1 << (8 - i);
            let ch = match i % 3 {
                0 => 'r',
                1 => 'w',
                _ => 'x',
            };
            result.push(if mode & bit != 0 { ch } else { '-' });
        }
        result
    }
    #[cfg(not(unix))]
    {
        "n/a".to_string()
    }
}

/// Convert SystemTime to Unix timestamp (seconds since epoch)
fn to_unix_timestamp(time: Result<SystemTime, std::io::Error>) -> i64 {
    time.ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Initialize DB schema for files and settings tables
pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL,
            filename TEXT NOT NULL,
            filetype TEXT NOT NULL,
            size INTEGER,
            permissions TEXT,
            created INTEGER,
            modified INTEGER,
            content TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Ask user to reuse previous scan path or enter new one
pub async fn get_scan_path(pool: &SqlitePool) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let prev: Option<(String,)> =
        sqlx::query_as("SELECT value FROM settings WHERE key = 'scan_path'")
            .fetch_optional(pool)
            .await?;

    if let Some((path,)) = prev {
        println!(
            "📁 Last scanned path: {}\nPress [c] to change or Enter to continue:",
            path
        );
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().eq_ignore_ascii_case("c") {
            return prompt_and_save_path(pool).await;
        }
        Ok(PathBuf::from(path))
    } else {
        prompt_and_save_path(pool).await
    }
}

/// Prompt user for scan directory and save to DB
async fn prompt_and_save_path(pool: &SqlitePool) -> Result<PathBuf, Box<dyn std::error::Error>> {
    print!("🛠️  Enter full folder path to scan: ");
    io::stdout().flush()?;
    let mut path = String::new();
    io::stdin().read_line(&mut path)?;
    let trimmed = path.trim();

    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES ('scan_path', ?)")
        .bind(trimmed)
        .execute(pool)
        .await?;

    Ok(PathBuf::from(trimmed))
}

/// Walk directory, collect metadata, and insert into database
pub async fn scan_and_store<P: AsRef<Path>>(
    pool: &SqlitePool,
    dir: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let dir = dir.as_ref();
    println!("📂 Scanning directory: {}", dir.display());

    let blacklist: HashSet<String> = load_blacklisted_folders();

    let entries: Vec<_> = WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !is_blacklisted(e.path(), &blacklist) && e.file_type().is_file())
        .collect();

    let bar = ProgressBar::new(entries.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("⏳ [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    for entry in entries {
        let path = entry.path();
        let fullpath = path.to_string_lossy().to_string();
        let filename = entry.file_name().to_string_lossy().to_string();
        let filetype = "file".to_string();

        match fs::metadata(path).await {
            Ok(metadata) => {
                let size = metadata.len() as i64;
                let permissions = format_permissions(&metadata);
                let created = to_unix_timestamp(metadata.created());
                let modified = to_unix_timestamp(metadata.modified());

                sqlx::query(
                    r#"
                    INSERT INTO files (
                        path, filename, filetype, size, permissions, created, modified, content
                    )
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL)
                    "#,
                )
                .bind(&fullpath)
                .bind(&filename)
                .bind(&filetype)
                .bind(size)
                .bind(&permissions)
                .bind(created)
                .bind(modified)
                .execute(pool)
                .await?;
            }
            Err(e) => {
                eprintln!("⚠️  Skipped {:?} (metadata error): {}", path, e);
                continue;
            }
        }

        bar.inc(1);
    }

    bar.finish_with_message("✅ Scan complete");
    Ok(())
}
