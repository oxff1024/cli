use std::{env, error::Error, fs};

pub struct Reader {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Reader {
    pub fn new(args: &[String]) -> Result<Reader, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive: bool = env::var("case_sensitive").is_err();

        Ok(Reader {
            query,
            filename,
            case_sensitive,
        })
    }

    pub fn run(reader: Reader) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(reader.filename)?;

        let results = if reader.case_sensitive {
            Self::search(&reader.query, &contents)
        } else {
            Self::search_case_insensitive(&reader.query, &contents)
        };

        for line in results {
            println!("{}", line);
        }
        Ok(())
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::<&str>::new();

        for line in contents.lines() {
            if line.contains(query) {
                results.push(line)
            }
        }
        results
    }

    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let query = query.to_lowercase();
        let mut results = Vec::<&str>::new();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "rep";
        let contents = "mini-grep:\nsimple cli implementation mimicing\nGRep\n functionality.";

        assert_eq!(vec!["mini-grep:"], Reader::search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rep";
        let contents = "mini-grep:\nsimple cli implementation mimicing\nGrep\n functionality.";

        assert_eq!(
            vec!["mini-grep:", "Grep"],
            Reader::search_case_insensitive(query, contents)
        );
    }
}
