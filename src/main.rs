//use std::env; // bring the parent to scope so that it can be reused
//use std::fs::File; // to handle files
//use std::io::prelude::*; // contains useful trains for doing I/O

extern crate minigrep_emmanuel;

use std::env;
use std::process;

use minigrep_emmanuel::Config;

//use std::error::Error;

// args return iterator of command line
// env to read command line arguments
// collect method to transform input into a collection
// collect is one function we need to annotate what type we are trying to derive from
// in our case its a vector string



fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_emmanuel::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}








