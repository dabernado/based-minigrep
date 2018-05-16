extern crate regex;

use std::fs;
use std::error::Error;
use self::regex::Regex;
use self::regex::escape;
use std::process;

pub fn grep_exec(query: &str, filename: &str, flag: &Option<String>) {
    let contents = read_file(&filename).unwrap_or_else(|err| {
        println!("Problem reading file: {}", err);
        process::exit(1);
    });

    println!("'{}' occures {} times in {}", query, search(&query, &contents, &flag), filename);
}

pub fn read_file(filename: &str) -> Result<String, Box<Error>> {
    let contents = fs::read_to_string(filename)?;

    Ok(contents)
}

pub fn search(query: &str, contents: &str, flag: &Option<String>) -> usize {
    let re = Regex::new(format!(r#"{}\b"#, query.to_lowercase().as_str()).as_str()).unwrap();
    let mut score: usize = 0;

    let case_sensitive = String::from("-c");

    match flag {
        Some(case_sensitive) => {
            let lowercased = contents.to_lowercase();
            let text = escape(lowercased.as_str());
            for _mat in re.find_iter(text.as_str()) {
                score = score + 1;
            }
        }
        None => {
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
    pub flag: Option<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let args_length = args.len();
        match args_length {
            1 => Err("Not enough arguments"),
            2 => Err("Not enough arguments"),
            3 => {
                let query = args[1].clone();
                let filename = args[2].clone();
                let flag = None;

                Ok(Config {query, filename, flag})
            },
            4 => {
                let query = args[1].clone();
                let filename = args[2].clone();
                let flag = Some(args[3].clone());

                Ok(Config {query, filename, flag})
            },
            _ => Err("Too many arguments"),
        }
    }
}

mod test {
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\nRust:\nsafe, fast, productive, duct tape.\nPick three.";
        let flag = None;

        assert_eq!(
            1,
            ::grep::search(&query, &contents, &flag)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\nRust:\nsafe, fast, productive, duct tape.\nPick three. RuSt";
        let flag = Some(String::from("-c"));

        assert_eq!(
            2,
            ::grep::search(&query, &contents, &flag),
        );
    }
}
