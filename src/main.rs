use std::env; // bring the parent to scope so that it can be reused


// args return iterator of command line
// env to read command line arguments
// collect method to transform input into a collection

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
