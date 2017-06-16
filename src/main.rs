extern crate walkdir;
#[macro_use]
extern crate error_chain;
extern crate clap;
extern crate rand;

use walkdir::WalkDir;
use std::io;
use clap::{Arg, App};
use rand::Rng;
use std::process;

error_chain! {
    // Error types from other libraries that we want to just wrap
    // automatically.
    foreign_links {
        Io(io::Error);
        WalkDir(walkdir::Error);
    }
}

fn run(folder: &str, extension: &str) -> Result<()> {
    let mut files: Vec<String> = Vec::new();

    // List recusively all accessible files in the current directory
    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        let dotted_extension = [".", extension].concat();

        // Get entry's filename
        if let Some(file_path) = entry.path().to_str() {
            // Add source files to collection
            if file_path.ends_with(&dotted_extension) {
                files.push(String::from(file_path));
            }
        }
    }

    if files.len() == 0 {
        println!("No files found. Try a different folder");
        process::exit(0);
    }

    // Print random file from the collection
    let mut rng = rand::thread_rng();
    match files.get(rng.gen_range(0, files.len())) {
        Some(x) => println!("{}", x),
        None => (),
    }

    Ok(())
}

fn main () {
    // Define command line arguments.
    let matches = App::new("linux-daily")
        .version("0.1.0")
        .author("Luis de Bethencourt <luis@debethencourt.com>")
        .about("Pick a random C file")
        .arg(Arg::with_name("folder")
             .short("f")
             .long("folder")
             .takes_value(true)
             .help("Folder to search in"))
        .arg(Arg::with_name("extension")
             .short("e")
             .long("extension")
             .takes_value(true)
             .help("Extension to look for"))
        .get_matches();

    // Get value for folder, or default to '.'
    let folder = matches.value_of("folder").unwrap_or(".");
    println!("The folder passed is: {}", folder);

    let extension = matches.value_of("extension").unwrap_or("c");
    run(folder, extension).unwrap();
}
