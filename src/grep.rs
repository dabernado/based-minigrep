extern crate regex;

use std::fs;
use std::env;
use std::error::Error;
use self::regex::Regex;
use self::regex::escape;
use std::process;

pub fn grep_exec(query: &str, filename: &str, flag: &str) {
    let contents = read_file(&filename).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        process::exit(1);
    });

    println!("'{}' occurs {} times in {}", query, search(&query, &contents, &flag), filename);
}

pub fn read_file(filename: &str) -> Result<String, Box<Error>> {
    let contents = fs::read_to_string(filename)?;

    Ok(contents)
}

pub fn search(query: &str, contents: &str, flag: &str) -> usize {
    let re = Regex::new(format!(r#"{}\b"#, query.to_lowercase().as_str()).as_str()).unwrap();
    let mut score = 0;

    let case_sensitive = String::from("-c");
    let none = String::from("NO_FLAG");

    match flag {
        case_sensitive => {
            let lowercased = contents.to_lowercase();
            let text = escape(lowercased.as_str());

            for _mat in re.find_iter(text.as_str()) {
                score = score + 1;
            }
        }
        none => {
            let text = escape(contents);
            for _mat in re.find_iter(text.as_str()) {
                score = score + 1;
            }
        }
    }

    score
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub flag: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let args_length = args.len();
        match args_length {
            1 => Err("Not enough arguments"),
            2 => {
                let query = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a query string"),
                };

                let filename = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a filename"),
                };

                let flag = String::from("NO_FLAG");

                Ok(Config {query, filename, flag})
            },
            3 => {
                let query = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a query string"),
                };

                let filename = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a filename"),
                };
                
                let flag = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a flag"),
                };

                Ok(Config {query, filename, flag})
            },
            _ => Err("Too many arguments"),
        }
    }
}
