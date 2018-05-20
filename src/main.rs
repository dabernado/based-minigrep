extern crate based_minigrep;
use std::env;
use std::process;
use based_minigrep::grep;

fn main() {
    let config = grep::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    grep::grep_exec(&config.query, &config.filename, &config.flag)
}
