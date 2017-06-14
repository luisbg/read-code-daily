extern crate walkdir;
#[macro_use]
extern crate error_chain;
extern crate clap;
extern crate rand;

use walkdir::WalkDir;
use std::io;
use clap::{Arg, App};
use rand::Rng;

error_chain! {
    // Error types from other libraries that we want to just wrap
    // automatically.
    foreign_links {
        Io(io::Error);
        WalkDir(walkdir::Error);
    }
}

fn run(folder: &str) -> Result<()> {
    let mut files: Vec<String> = Vec::new();

    // List recusively all accessible files in the current directory
    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        // Get entry's filename
        if let Some(file_path) = entry.path().to_str() {
            // Add C files to collection
            if file_path.ends_with(".c") {
                files.push(String::from(file_path));
            }
        } 
    }

    if files.len() > 0 {
        // Print random file from the collection
        let mut rng = rand::thread_rng();
        match files.get(rng.gen_range(0, files.len())) {
            Some(x) => println!("{}", x),
            None => (),
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
                 .long("file")
                 .takes_value(true)
                 .help("A cool file"))
        .get_matches();

    // Get value for folder, or default to '.'
    let folder = matches.value_of("folder").unwrap_or(".");
    println!("The folder passed is: {}", folder);

    run(folder).unwrap();
}