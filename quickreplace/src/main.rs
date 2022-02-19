use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!("{} - Change occurrences of one string into another", "Quick Replace".green());
    eprintln!("Usage: Quick Replace <target> <replacement> <INPUT> <OUTPUT>";)
}

fn main() {
    println!("Hello, world!");
}
