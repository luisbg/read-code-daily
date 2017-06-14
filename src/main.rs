extern crate walkdir;
#[macro_use]
extern crate error_chain;
extern crate clap;

use walkdir::WalkDir;
use std::io;
use clap::{Arg, App};

error_chain! {
    // Error types from other libraries that we want to just wrap
    // automatically.
    foreign_links {
        Io(io::Error);
        WalkDir(walkdir::Error);
    }
}

fn run(folder: &str) -> Result<()> {
    // List recusively all accessible files in the current directory
    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
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