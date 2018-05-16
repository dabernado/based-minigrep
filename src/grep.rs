extern crate regex;

use std::fs;
use std::error::Error;
use self::regex::Regex;
use self::regex::escape;
use std::process;

pub fn grep_exec(query: &str, filename: &str) {
    let contents = read_file(&filename).unwrap_or_else(|err| {
        println!("Problem reading file: {}", err);
        process::exit(1);
    });

    println!("'{}' occures {} times in {}", query, search(&query, &contents), filename);
}

pub fn read_file(filename: &str) -> Result<String, Box<Error>> {
    let contents = fs::read_to_string(filename)?.to_lowercase();
    Ok(contents)
}

pub fn search(query: &str, contents: &str) -> usize {
    let re = Regex::new(format!(r#"{}\b"#, query).as_str()).unwrap();
    let text = escape(contents);
    let mut score: usize = 0;
    for _mat in re.find_iter(text.as_str()) {
        score = score + 1;
    }

    score
}

pub struct Config {
    pub query: String,
    pub filename: String,
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

                Ok(Config {query, filename})
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

        assert_eq!(
            2,
            ::grep::search(query, contents)
        );
    }
}
