use std::env; // bring the parent to scope so that it can be reused
use std::fs::File; // to handle files
use std::io::prelude::*; // contains useful trains for doing I/O

// args return iterator of command line
// env to read command line arguments
// collect method to transform input into a collection
// collect is one function we need to annotate what type we are trying to derive from
// in our case its a vector string



fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);

    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");


    println!("With text:\n{}", contents);

}
