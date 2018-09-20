use std::env; // bring the parent to scope so that it can be reused
use std::fs::File; // to handle files
use std::io::prelude::*; // contains useful trains for doing I/O

use std::process;
use std::error::Error;

// args return iterator of command line
// env to read command line arguments
// collect method to transform input into a collection
// collect is one function we need to annotate what type we are trying to derive from
// in our case its a vector string



fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}


struct Config {
    query: String ,
    filename: String ,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &'static str>  {
      if args.len() < 3 {
          return Err(" not enough arguments");
      }
      let query = args[1].clone();
      let filename = args[2].clone();

      Ok(Config { query, filename })
  }
}




