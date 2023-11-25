use std::fs::File;
use std::io::{self, Write};
use std::path::Path;


fn 
main() {
    // Retrieve directory and file path from the command line argument
    let args: Vec<String> = std::env::args().collect();

    // Check if a single command line argument is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <directory/file>", args[0]);
        std::process::exit(1);
    }

    let target_path = &args[1];

    // Create the directory or file based on the specified path
    if let Err(e) = create_dir_or_file(target_path) {
        eprintln!("Error creating {}: {}", target_path, e);
        std::process::exit(1);
    }

    println!("Directory or file created successfully.");
}

// Function to create a directory or file at the specified path
fn
create_dir_or_file(target_path: &str) -> io::Result<()> {
    let path = Path::new(target_path);

    if path.is_dir() {
        // Create the directory and its parents
        if let Err(e) = std::fs::create_dir_all(path) {
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    } else {
        // Create the file and its parent directories
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let mut file = File::create(&path)?;
        // Write initial content to the file if needed
        writeln!(file, "// Your initial content goes here")?;
    }

    Ok(())
}

