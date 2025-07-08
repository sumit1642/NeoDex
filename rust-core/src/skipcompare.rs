// rust-core/src/skipcompare.rs
use std::{
    collections::HashSet,
    fs,
    path::{Path, Component},
};

pub fn load_blacklisted_folders() -> HashSet<String> {
    // Always look for this file in the working directory
    let path = "blacklisted_folders.txt";

    let contents = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("❌ Failed to read blacklisted file: {}", path));

    let folders: HashSet<String> = contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|s| s.to_string())
        .collect();

    display_folders_grid(&folders);
    folders
}

fn display_folders_grid(folders: &HashSet<String>) {
    let mut sorted: Vec<_> = folders.iter().collect();
    sorted.sort();

    let columns = 3;
    let rows = (sorted.len() as f32 / columns as f32).ceil() as usize;

    println!("\n🚫 Skipping these folders:\n");

    for row in 0..rows {
        for col in 0..columns {
            let idx = row + col * rows;
            if let Some(folder) = sorted.get(idx) {
                print!("{:<25}", folder);
            }
        }
        println!();
    }

    println!();
}

pub fn is_blacklisted(path: &Path, blacklist: &HashSet<String>) -> bool {
    path.components().any(|comp| {
        if let Component::Normal(name) = comp {
            blacklist.contains(&name.to_string_lossy().to_string())
        } else {
            false
        }
    })
}
