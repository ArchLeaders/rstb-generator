mod generator;

use clap::Parser;
use generator::{Generator, GeneratorOptions};

fn main() {
    let options: GeneratorOptions = GeneratorOptions::parse();
    match Generator::from_options(options).build() {
        Err(e) => println!("{}", e),
        _ => ()
    }
}
