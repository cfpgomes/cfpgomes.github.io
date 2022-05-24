use std::error::Error;
use std::fs::File;
use std::io::Read;

use chrono::NaiveDate;

/// Struct that represents a publication, which contains info such
/// as title, date, markdown, and tags.
#[derive(Debug)]
pub struct Publication {
    pub title: String,
    pub date: NaiveDate,
    pub markdown: String,
    pub tags: Vec<String>,
}

impl Publication {
    pub fn new(title: String, date: NaiveDate, markdown: String, tags: Vec<String>) -> Publication {
        Publication {
            title,
            date,
            markdown,
            tags,
        }
    }

    pub fn from_gobbet(gobbet_path: &str) -> Result<Publication, &'static str> {
        // Read contents of gobbet.
        let mut gobbet_file = match File::open(gobbet_path) {
            Ok(f) => f,
            _ => return Err("Couldn't open gobbet file."),
        };
        let mut gobbet_contents = String::new();
        match gobbet_file.read_to_string(&mut gobbet_contents) {
            Ok(_) => (),
            _ => return Err("Couldn't read gobbet contents."),
        };

        // Fill Publication fields.

        // Title
        if let Some((title, gobbet_contents)) = gobbet_contents.split_once("🍖DATE🍖") {
            let title = &title.replace("🍖TITLE🍖", "");
            let title = title.trim();
            // Date
            if let Some((date, gobbet_contents)) = gobbet_contents.split_once("🍖MARKDOWN🍖") {
                let date = &date.replace("🍖DATE🍖", "");
                let date = date.trim();
                // Markdown
                if let Some((markdown, tags)) = gobbet_contents.split_once("🍖TAGS🍖") {
                    let markdown = &markdown.replace("🍖MARKDOWN🍖", "");
                    let markdown = markdown.trim();
                    return Ok(Publication {
                        title: title.to_string(),
                        date: match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                            Ok(d) => d,
                            _ => return Err("Couldn't parse date from str."),
                        },
                        markdown: markdown.to_string(),
                        tags: tags.split(",").map(|s| s.to_string()).collect(),
                    });
                } else {
                    return Err("Couldn't split at markdown.");
                };
            } else {
                return Err("Couldn't split at date.");
            };
        } else {
            return Err("Couldn't split at title.");
        };
    }
}

/// Enum to enumerate the three types of possible queries:
/// `Publish` represents a query to publish a new `Publication`;
/// `Modify` represents a query to edit an existing `Publication`;
/// `Build` represents a query to build the website.
pub enum Query {
    Publish,
    Modify,
    Build,
}

/// Simple struct that stores query.
pub struct Config {
    pub query: Query,
}

impl Config {
    /// Creates a `Config` according to the argument passed via terminal.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 2 {
            return Err("Invalid number of arguments");
        }

        let query = match args[1].as_str() {
            "p" | "pub" | "publ" | "publish" => Query::Publish,
            "m" | "mod" | "modi" | "modify" => Query::Publish,
            "b" | "bui" | "buil" | "build" => Query::Publish,
            _ => return Err("Invalid query"),
        };

        Ok(Config { query })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.query {
        Query::Publish => publish(),
        Query::Modify => modify(),
        Query::Build => build(),
    }
}

fn publish() -> Result<(), Box<dyn Error>> {
    unimplemented!()
}

fn modify() -> Result<(), Box<dyn Error>> {
    unimplemented!()
}

fn build() -> Result<(), Box<dyn Error>> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publication_from_gobbet() {
        assert!(Publication::from_gobbet("posts\\test.gobbet").is_ok());
    }
}
