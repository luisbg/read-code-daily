extern crate walkdir;
#[macro_use]
extern crate error_chain;

use walkdir::WalkDir;
use std::io;

error_chain! {
    // Error types from other libraries that we want to just wrap
    // automatically.
    foreign_links {
        Io(io::Error);
        WalkDir(walkdir::Error);
    }
}

fn run() -> Result<()> {
    // List recusively all accessible files in the current directory
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        // Get entry's filename
        let f_name = entry.file_name().to_string_lossy();

        // Print C files
        if f_name.ends_with(".c") {
            println!("{}", f_name);
        }
    }

    Ok(())
}

fn main () {
    run().unwrap();
}