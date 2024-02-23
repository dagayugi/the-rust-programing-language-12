use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get aquery string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        case_sensitive = match args.next() {
            Some(arg) => match arg.to_lowercase().as_str() {
                "true" => true,
                "false" => false,
                _ => case_sensitive,
            },
            None => case_sensitive,
        };

        Ok(Config { query, filename , case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insenstive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query)) 
        .collect()
}

pub fn search_case_insenstive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    const MINIGREP: &str = "minigrep";
    const QUERY: &str = "query";
    const FILENAME: &str = "poem.txt";

    #[test]
    fn case_new() {
        let args = vec![String::from(MINIGREP), String::from(QUERY), String::from(FILENAME)];
        let config = Config::new(args.into_iter()).unwrap();
        assert_eq!(config.query, QUERY);
        assert_eq!(config.filename, FILENAME);
        assert_eq!(config.case_sensitive, true);
    }

    #[test]
    fn case_new_sensitive_from_args() {
        let args = vec![String::from(MINIGREP), String::from(QUERY), String::from(FILENAME), String::from("true")];
        let config = Config::new(args.into_iter()).unwrap();
        assert_eq!(config.query, QUERY);
        assert_eq!(config.filename, FILENAME);
        assert_eq!(config.case_sensitive, true);
    }

    #[test]
    fn case_new_with_no_args() {
        let args = vec![String::from(MINIGREP)];
        let config = Config::new(args.into_iter());
        assert!(config.is_err());
    }

    #[test]
    fn case_new_with_case_no_env() {
        let args = vec![String::from(MINIGREP), String::from("query"), String::from(FILENAME)];
        let config = Config::new(args.into_iter()).unwrap();
        assert_eq!(config.case_sensitive, true);
    }

    #[test]
    fn case_new_with_case_env() {
        env::set_var("CASE_INSENSITIVE", "1");
        let args = vec![String::from(MINIGREP), String::from("query"), String::from(FILENAME)];
        let config = Config::new(args.into_iter()).unwrap();
        assert_eq!(config.case_sensitive, false);
        env::remove_var("CASE_INSENSITIVE");
    }

    #[test]
    fn case_run() {
        let args = vec![String::from(MINIGREP), String::from("query"), String::from(FILENAME)];
        let config = Config::new(args.into_iter()).unwrap();
        let result = run(config);

        assert!(result.is_ok());
    }

    #[test]
    fn case_run_with_none_file() {
        let args = vec![String::from(MINIGREP), String::from(QUERY), String::from("none")];
        let config = Config::new(args.into_iter()).unwrap();

        let result = run(config);

        assert!(result.is_err());

    }

    #[test]
    fn case_senstive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."],
        search(query, contents)
        );
    }

    #[test]
    fn case_insenstive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insenstive(query, contents)
        );
    }
}