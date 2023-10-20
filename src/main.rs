use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {
    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: mkdirfi <path_to_create>");
        std::process::exit(1);
    }

    let path_to_create = &args[1];

    // Create the parent directory and the file
    if let Some(parent_dir) = Path::new(path_to_create).parent() {
        if let Err(err) = fs::create_dir_all(parent_dir) {
            eprintln!("Error creating directory: {}", err);
            std::process::exit(1);
        }
    }

    match File::create(path_to_create) {
        Ok(mut file) => {
            // You can optionally write data to the file here if needed.
            // For example, you can write an initial content to the file.
            if let Err(err) = writeln!(file) {
                eprintln!("Error writing to file: {}", err);
                std::process::exit(1);
            }
            println!("Created file: {:?}", path_to_create);
        }
        Err(err) => {
            eprintln!("Error creating file: {}", err);
            std::process::exit(1);
        }
    }
}

