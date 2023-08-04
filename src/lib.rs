use std::{env, error::Error, fs};

pub struct Reader {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Reader {
    pub fn new(mut args: env::Args) -> Result<Reader, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not receive query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not receive query string"),
        };

        let case_sensitive: bool = true;//env::var("case_sensitive").is_err();

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
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }

    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let query = query.to_lowercase();
        contents
            .lines()
            .filter(|line| line.contains(query.to_lowercase().as_str()))
            .collect()
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
