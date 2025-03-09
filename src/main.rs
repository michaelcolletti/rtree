use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    let path = Path::new(path);
    if path.exists() && path.is_dir() {
        print_tree(path, 0);
    } else {
        eprintln!("Invalid directory path");
    }
}

fn print_tree(path: &Path, level: usize) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_type = entry.file_type().unwrap();
                for _ in 0..level {
                    print!("|   ");
                }
                println!("|-- {}", entry.file_name().to_string_lossy());
                if file_type.is_dir() {
                    print_tree(&entry.path(), level + 1);
                }
            }
        }
    }
}