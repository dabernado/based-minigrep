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

    pub fn grep_exec(filename: &str, flag: &str, words: &Vec<String>) {
        let contents = read_file(&filename).unwrap_or_else(|err| {
            eprintln!("Problem reading file: {}", err);
            process::exit(1);
        });

        let words_iter = words.par_iter();

        words_iter.for_each(|word| {
            println!("'{}' occurs {} times in {}", word, search(&word, &contents, &flag), filename);
        });
    }

    pub fn read_file(filename: &str) -> Result<String, Box<Error>> {
        let contents = fs::read_to_string(filename)?;

        Ok(contents)
    }

    pub fn search(query: &str, contents: &str, flag: &str) -> usize {
        let re = Regex::new(format!(r#"{}\b"#, query.to_lowercase().as_str()).as_str()).unwrap();

        let case_sensitive = String::from("-c");
        let none = String::from("NO_FLAG");

        match flag {
            case_sensitive => {
                let lowercased = contents.to_lowercase();
                let text = escape(lowercased.as_str());
                let score = re.find_iter(text.as_str()).count();
                
                score
            }
            none => {
                let text = escape(contents);
                let score = re.find_iter(text.as_str()).count();

                score
            }
        }
    }

    pub struct Config {
        pub filename: String,
        pub flag: String,
        pub words: Vec<String>,
    }

    impl Config {
        pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
            args.next();

            let args_length = args.len();
            match args_length {
                1 => Err("Not enough arguments"),
                _ => {
                    let filename = match args.next() {
                        Some(arg) => arg,
                        None => return Err("Didn't get a filename"),
                    };

                    let flag = match args.next() {
                        Some(arg) => arg,
                        None => String::from("NO_FLAG"),
                    };

                    let words = args.collect();

                    Ok(Config {filename, flag, words})
                },
            }
        }
    }
}
