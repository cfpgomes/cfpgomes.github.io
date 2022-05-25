use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::Read;
use std::io::Write as OtherWrite;

use chrono::NaiveDate;

use comrak::{markdown_to_html, ComrakOptions};

use html_builder::*;

enum Condition {
    In,
    NotIn,
}

struct Rule {
    condition: Condition,
    tag: String,
}

impl Rule {
    fn new(condition: Condition, tag: String) -> Self {
        Self { condition, tag }
    }

    fn check(&self, publication: &Publication) -> bool {
        match self.condition {
            Condition::In => publication.tags.iter().any(|t| t == &self.tag),
            Condition::NotIn => publication.tags.iter().all(|t| t != &self.tag),
        }
    }
}

struct Page {
    title: String,
    rules: Vec<Rule>,
    publications: Vec<Publication>,
}

impl Page {
    fn new(title: String) -> Self {
        Self {
            title: title,
            rules: vec![],
            publications: vec![],
        }
    }

    fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    fn add_publication(&mut self, publication: Publication) {
        self.publications.push(publication);
    }

    fn to_html(&self) -> String {
        let mut buf = Buffer::new();
        writeln!(buf, "<!-- My website -->").unwrap();

        // The Html5 trait provides various helper methods.  For instance, doctype()
        // simply writes the <!DOCTYPE> header
        buf.doctype();

        // Most helper methods create child nodes.  You can set a node's attributes
        // like so
        let mut html = buf.html().attr("lang='en'");

        let mut head = html.head();

        // Meta is a "void element", meaning it doesn't need a closing tag.  This is
        // handled correctly.
        head.meta().attr("charset='utf-8'");

        // For site responsiveness
        head.meta()
            .attr("name='utf-8' content='width=device-width,initial-scale=1.0'");

        // Just like Buffer, nodes are also writable.  Set their contents by
        // writing into them.
        // Title
        writeln!(head.title(), "Cláudio Gomes | {}", self.title).unwrap();

        // Description is the same as title.
        head.meta()
            .attr(format!("name='description' content='{}'", self.title).as_str());
        // Keywords are tags used in rules.
        head.meta().attr(
            format!(
                "name='keywords', content='{}'",
                self.rules
                    .iter()
                    .map(|r| match r.condition {
                        Condition::In => "".to_string() + &r.tag + ",",
                        Condition::NotIn => "not ".to_string() + &r.tag + ",",
                    })
                    .collect::<String>()
                    .trim_end_matches(",")
            )
            .as_str(),
        );

        // Necessary stylesheets.
        head.link()
            .attr("rel='stylesheet' href='https://unpkg.com/spectre.css/dist/spectre.min.css'");
        head.link()
            .attr("rel='stylesheet' href='https://unpkg.com/spectre.css/dist/spectre.min.css'");
        head.link()
            .attr("rel='stylesheet' href='https://unpkg.com/spectre.css/dist/spectre.min.css'");

        // Body
        let mut body = html.body();

        // Navbar TODO: link between pages
        let mut header = body.header().attr("class='navbar'");

        let mut div = header.div().attr("class='navbar-primary'");
        writeln!(
            div.a().attr("href='#' class='navbar-brand mr-10'"),
            "Cláudio Gomes | {}",
            self.title
        )
        .unwrap();
        writeln!(
            div.a().attr("href='#' class='btn btn-link selected'"),
            "Home"
        )
        .unwrap();
        writeln!(div.a().attr("href='#' class='btn btn-link'"), "About").unwrap();
        writeln!(div.a().attr("href='#' class='btn btn-link'"), "Contact").unwrap();

        let mut div = body.div().attr("class='container text-center'");
        writeln!(div.h1(), "Spectre.css starter template").unwrap();
        writeln!(div.h2(), "Tiny, responsive, fast.").unwrap();

        // Finish
        buf.finish()
    }
}

/// Struct that represents a publication, which contains info such
/// as title, date, markdown, and tags.
#[derive(Debug)]
struct Publication {
    title: String,
    date: NaiveDate,
    markdown: String,
    tags: Vec<String>,
}

impl Publication {
    fn new(title: String, date: NaiveDate, markdown: String, tags: Vec<String>) -> Self {
        Self {
            title,
            date,
            markdown,
            tags,
        }
    }

    fn from_gobbet(gobbet_path: &str) -> Result<Self, &'static str> {
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
                    return Ok(Self {
                        title: title.to_string(),
                        date: match NaiveDate::parse_from_str(&date, "%Y/%m/%d") {
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

    fn to_html(&self) -> String {
        markdown_to_html(&self.markdown, &ComrakOptions::default()).to_string()
    }
}

/// Enum to enumerate the three types of possible queries:
/// `Publish` represents a query to publish a new `Publication`;
/// `Modify` represents a query to edit an existing `Publication`;
/// `Build` represents a query to build the website.
enum Query {
    Publish,
    Modify,
    Build,
}

/// Simple struct that stores query.
pub struct Config {
    query: Query,
}

impl Config {
    /// Creates a `Config` according to the argument passed via terminal.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 3 {
            return Err("Invalid number of arguments");
        }

        let query = match args[1].as_str() {
            "p" | "pub" | "publ" | "publish" => Query::Publish,
            "m" | "mod" | "modi" | "modify" => Query::Modify,
            "b" | "bui" | "buil" | "build" => Query::Build,
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
    let publication = Publication::from_gobbet("posts\\test.gobbet").unwrap();
    let mut page = Page::new("Página de Teste".to_string());

    page.add_publication(publication);

    page.add_rule(Rule {
        condition: Condition::In,
        tag: "nature".to_string(),
    });

    let mut index_file = File::create("index.html")?;

    write!(index_file, "{}", page.to_html())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publication_from_gobbet() {
        assert!(Publication::from_gobbet("posts\\test.gobbet").is_ok());
    }
}
