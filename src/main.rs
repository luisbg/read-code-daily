extern crate walkdir;
#[macro_use]
extern crate error_chain;
#[macro_use]
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

fn run(folder: &str, extension: &str, num_files: usize) -> Result<()> {
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

    if files.len() <= num_files {
        // Print all files from the collection
        for f in files.into_iter() {
            println!("{}", f);
        }
    } else {
        // Print random num_file's from the collection
        let mut rng = rand::thread_rng();
        for _ in 0..num_files {
            let index = rng.gen_range(0, files.len());
            match files.get(index) {
                Some(f) => println!("{}", f),
                None => (),
            }
            files.remove(index);
        }
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
        .arg(Arg::with_name("num")
             .short("n")
             .long("number")
             .takes_value(true)
             .help("Number of files to return"))
        .get_matches();

    // Get value for folder, or default to '.'
    let folder = matches.value_of("folder").unwrap_or(".");
    println!("The folder passed is: {}", folder);

    let extension = matches.value_of("extension").unwrap_or("c");
    let num_files = value_t!(matches, "num", usize).unwrap_or(1);

    run(folder, extension, num_files).unwrap();
}
