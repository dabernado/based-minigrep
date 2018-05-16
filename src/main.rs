extern crate pandion;
use std::env;
use std::process;
use pandion::grep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = grep::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    grep::grep_exec(&config.query, &config.filename)
}
