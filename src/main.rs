use std::env;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    let days = if args.len() > 2 {
        args[2].parse::<u64>().unwrap_or(0)
    } else {
        0
    };

    let path = Path::new(path);
    if path.exists() && path.is_dir() {
        print_tree(path, 0, days);
    } else {
        eprintln!("Invalid directory path");
    }
}

fn print_tree(path: &Path, level: usize, days: u64) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_type = entry.file_type().unwrap();
                let metadata = entry.metadata().unwrap();
                let modified = metadata.modified().unwrap();
                let duration = modified.duration_since(UNIX_EPOCH).unwrap();
                let modified_days = duration.as_secs() / 86400;

                if days == 0 || modified_days <= days {
                    for _ in 0..level {
                        print!("|   ");
                    }

                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_type.is_dir() {
                        println!("|-- {}", file_name.blue());
                    } else {
                        println!("|-- {}", file_name.green());
                    }
                }

                if file_type.is_dir() {
                    print_tree(&entry.path(), level + 1, days);
                }
            }
        }
    }
}

