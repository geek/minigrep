use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("missing arguments: `minigrep search file.txt`")
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(&config.filename)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    for line in search(&config.query, &file_contents) {
      println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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

  #[test]
  fn search() {
    let query = "test";
    let contents = "\
here is some
lines with some
test items
on the lines
    ";

    assert_eq!(vec!["test items"], super::search(query, contents));
  }
}
