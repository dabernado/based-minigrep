extern crate regex;
extern crate rayon;

pub mod grep {
    use std::fs;
    use std::env;
    use std::error::Error;
    use ::regex::Regex;
    use ::regex::escape;
    use std::process;
    use rayon::prelude::*;

    pub fn grep_exec(filename: &str, flag: Flag, words: &Vec<String>) {
        let contents = read_file(&filename).unwrap_or_else(|err| {
            eprintln!("Problem reading file: {}", err);
            process::exit(1);
        });

        let words_iter = words.par_iter();

        words_iter.for_each(|word| {
            println!("'{}' occurs {} times in {}", word, search(word, &contents, &flag), filename);
        });
    }

    pub fn read_file(filename: &str) -> Result<String, Box<Error>> {
        let contents = fs::read_to_string(filename)?;

        Ok(contents)
    }

    pub fn search(query: &str, contents: &str, flag: &Flag) -> usize {
        let re = Regex::new(format!(r#"{}\b"#, query.to_lowercase().as_str()).as_str()).unwrap();

        match flag {
            Flag::Lowercase => {
                let lowercased = contents.to_lowercase();
                let text = escape(lowercased.as_str());
                let score = re.find_iter(text.as_str()).count();
                
                score
            },
            Flag::NoFlag => {
                let text = escape(contents);
                let score = re.find_iter(text.as_str()).count();
                
                score
            },
        }
    }

    pub enum Flag {
        Lowercase,
        NoFlag,
    }

    pub struct Config {
        pub filename: String,
        pub flag: Flag,
        pub words: Vec<String>,
    }

    impl Config {
        pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
            args.next();

            let filename = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a filename"),
            };

            let flag = match args.next().unwrap_or_else(|| {
                eprintln!("ERROR: You must provide a flag");
                process::exit(1);
            }).as_str() {
                "-c" => Flag::Lowercase,
                "-n" => Flag::NoFlag,
                _ => {
                    eprintln!("ERROR: Flag provided does not exist. See documentation");
                    process::exit(1);
                }
            };

            let words = args.collect();

            Ok(Config {filename, flag, words})
        }
    }
}
