use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::Read;
use std::io::Write as OtherWrite;
use std::path::PathBuf;
use std::sync::mpsc;

use image_compressor::Factor;
use image_compressor::FolderCompressor;

use chrono::NaiveDate;

use comrak::{markdown_to_html, ComrakOptions};

use html_builder::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

const FOLDER_PUBLICATIONS: &str = "publications";

const NUMBERS: [&str; 2] = [
    // "zero",
    "one",
    "two",
    // "three",
    // "four",
    // "five",
    // "six",
    // "seven",
    // "eight",
    // "nine",
    // "ten",
    // "eleven",
    // "twelve",
    // "thirteen",
    // "fourteen",
    // "fifteen",
    // "sixteen",
    // "seventeen",
    // "eighteen",
    // "nineteen",
];

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
        println!("{:?}", publication.tags);
        match self.condition {
            Condition::In => publication.tags.iter().any(|t| t == &self.tag),
            Condition::NotIn => publication.tags.iter().all(|t| t != &self.tag),
        }
    }
}

enum CSS {
    Homemade,
    Science,
}

struct Page {
    title: String,
    rules: Vec<Rule>,
    publications: Vec<Publication>,
    css: CSS,
}

impl Page {
    fn new(title: String, css: CSS) -> Self {
        Self {
            title: title,
            rules: vec![],
            publications: vec![],
            css: css,
        }
    }

    fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    fn add_publication(&mut self, publication: Publication) -> bool {
        match self.rules.iter().all(|r| r.check(&publication)) {
            true => self.publications.push(publication),
            false => return false,
        };

        true
    }

    fn add_to_node(&self, mut root: html_builder::Node) {
        let mut rng = thread_rng();
        let mut container = root.div().attr("class='container grid-lg'");
        for publication in &self.publications {
            let mut column = container.div().attr("class='columns col-gapless'");
            let mut card_col = column
                .div()
                .attr("class='column col-xl-12 col-10' style='padding: 32px 0 0 0'");
            let mut card = card_col.div().attr(
                format!(
                    "class='card s-rounded' id='{}'",
                    NUMBERS.choose(&mut rng).unwrap()
                )
                .as_str(),
            );
            let mut card_header = card.div().attr("class='card-header'");
            writeln!(
                card_header.div().attr("class='card-title h2 strong'"),
                "{}",
                publication.title
            );

            let mut sub_header = card_header.div().attr("class='card-subtitle h4 text-gray'");
            writeln!(sub_header, "{}", publication.date);

            writeln!(
                card.div().attr("class='card-body'"),
                "{}",
                publication.to_html()
            );

            let mut card_footer = card.div().attr("class='card-footer'");

            for tag in publication.tags.iter() {
                writeln!(card_footer.span().attr("class='chip'"), "#{}", tag);
            }

            column.div().attr("class='column hide-xl col-1'");

            let mut share_col = column
                .div()
                .attr("class='column col-xl-12 col-1' style='padding: 16px 0 16px 0'");
            let mut share_cols = share_col.div().attr("class='columns col-gapless'");

            let mut twitter_col = share_cols
                .div()
                .attr("class='column col-xl-2 col-12' style='padding: 16px 0'");
            let mut twitter_button = twitter_col.button().attr(format!("class='btn btn-action tooltip tooltip-right' data-tooltip='Share on Twitter!' data-sharer='twitter' data-title='I read \"{}\" and you should too!' data-hashtags='{}' data-url='https://cfpgomes.github.io/'",publication.title, publication.tags.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")).as_str());

            twitter_button.i().attr("class='fa-brands fa-twitter'");

            let mut reddit_col = share_cols
                .div()
                .attr("class='column col-xl-2 col-12' style='padding: 16px 0'");

            let mut reddit_button = reddit_col.button().attr(format!("class='btn btn-action tooltip tooltip-right' data-tooltip='Share on Reddit!'  data-sharer='reddit' data-title='I read \"{}\" and you should too!' data-url='https://cfpgomes.github.io/'",publication.title).as_str());

            reddit_button.i().attr("class='fa-brands fa-reddit-alien'");
        }
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
        println!("Reading gobbet file {:?}", gobbet_path);
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
                        tags: tags
                            .split(",")
                            .map(|s| s.to_string().trim().to_string())
                            .collect(),
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
    // Compress images in `img` folder for web
    println!("Compression of images started!");
    let origin = PathBuf::from("img"); // original directory path
    let dest = PathBuf::from("compressed-img"); // destination directory path
    let thread_count = 4; // number of threads
    let (tx, _tr) = mpsc::channel(); // Sender and Receiver. for more info, check mpsc and message passing.

    let mut comp = FolderCompressor::new(origin, dest);
    comp.set_cal_func(|_width, _height, _file_size| return Factor::new(69., 1.0));
    comp.set_thread_count(thread_count);
    comp.set_sender(tx);

    match comp.compress() {
        Ok(_) => {}
        Err(e) => println!("Cannot compress the folder!: {}", e),
    }

    println!("Compression of images finished!");

    // Get all publications
    let mut publications: Vec<Publication> = vec![];

    for entry in std::fs::read_dir(FOLDER_PUBLICATIONS)? {
        publications.push(
            Publication::from_gobbet(
                format!("{:?}", entry.unwrap().path())
                    .as_str()
                    .strip_prefix("\"")
                    .unwrap()
                    .strip_suffix("\"")
                    .unwrap(),
            )
            .unwrap(),
        )
    }

    let mut page = Page::new("Página de Teste".to_string(), CSS::Science);

    page.add_rule(Rule {
        condition: Condition::In,
        tag: "paper".to_string(),
    });

    publications.sort_by_key(|p| p.date);

    for publication in publications {
        let title = publication.title.clone();
        if page.add_publication(publication) {
            println!("Publication {} added to Page {}.", title, page.title);
        }
    }

    let mut index_file = File::create("index.html")?;

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
        .attr("name='viewport' content='width=device-width,initial-scale=1.0'");

    // Just like Buffer, nodes are also writable.  Set their contents by
    // writing into them.
    // Title
    writeln!(head.title(), "Cláudio Gomes | {}", "TODO").unwrap();

    // Description is the same as title.
    head.meta()
        .attr(format!("name='description' content='{}'", "TODO").as_str());
    // Keywords are tags used in rules.
    // TODO
    // head.meta().attr(
    //     format!(
    //         "name='keywords', content='{}'",
    //         self.rules
    //             .iter()
    //             .map(|r| match r.condition {
    //                 Condition::In => "".to_string() + &r.tag + ",",
    //                 Condition::NotIn => "not ".to_string() + &r.tag + ",",
    //             })
    //             .collect::<String>()
    //             .trim_end_matches(",")
    //     )
    //     .as_str(),
    // );

    // Necessary stylesheets.
    head.link()
        .attr("rel='stylesheet' href='https://unpkg.com/spectre.css/dist/spectre.min.css'");
    head.link()
        .attr("rel='stylesheet' href='https://unpkg.com/spectre.css/dist/spectre.min.css'");
    head.link()
        .attr("rel='stylesheet' href='https://unpkg.com/spectre.css/dist/spectre.min.css'");
    head.link()
        .attr("rel='preconnect' href='https://fonts.googleapis.com'");
    head.link()
        .attr("rel='preconnect' href='https://fonts.gstatic.com' crossorigin");
    head.link()
        .attr("rel='stylesheet' href='https://fonts.googleapis.com/css2?family=Atkinson+Hyperlegible&family=Fredericka+the+Great&family=Kdam+Thmor+Pro&family=Klee+One&display=swap'");

    head.link().attr(match page.css {
        CSS::Homemade => "rel='stylesheet' href='css\\homemade.css'",
        CSS::Science => "rel='stylesheet' href='css\\science.css'",
    });

    html.script()
        .attr("src='https://cdn.jsdelivr.net/npm/sharer.js@latest/sharer.min.js'");

    html.script()
        .attr("src='https://kit.fontawesome.com/6a394e2d40.js' crossorigin='anonymous'");

    let mut script = html.script();
    write!(script, "\
    function onLoad()
    {{
        preload_image_object = new Image();
        var imagesArray = ['carousel1.jpg', 'carousel2.jpg', 'carousel3.jpg', 'carousel4.jpg', 'carousel5.jpg', 'carousel6.jpg', 'carousel7.jpg', 'carousel8.jpg', 'carousel9.jpg', 'carousel10.jpg'];
    
        //Preload images for faster page response
        for (var i=0; i < imagesArray.length; i++) {{
            preload_image_object.src = imagesArray[i];
            preload_image_object.onload = console.log(i);
        }};
        
        document.getElementById('background-image-id').style.backgroundImage = 'url(\"./compressed-img/' + imagesArray[Math.floor(Math.random() * 10)] + '\")';
    }}
    ").unwrap();

    // Body
    let mut body = html
        .body()
        .attr("class='gallery-background' id='background-image-id' onload='onLoad()'");
    // Container to apply shadow
    let shadow_gradient = body.div().attr("class='special-shadow-gradient'");

    // Navbar TODO: link between pages
    let mut header = body.header().attr("class='navbar'");

    let mut div = header.div().attr("class='navbar-primary'");
    writeln!(
        div.a().attr("href='#' class='navbar-brand mr-10'"),
        "Cláudio Gomes | {}",
        "TODO"
    )
    .unwrap();
    writeln!(
        div.a().attr("href='#' class='btn btn-link selected'"),
        "Home"
    )
    .unwrap();
    writeln!(div.a().attr("href='#' class='btn btn-link'"), "Research").unwrap();
    writeln!(div.a().attr("href='#' class='btn btn-link'"), "Contact").unwrap();

    let div = body.div().attr("class='container'");

    page.add_to_node(div);

    write!(index_file, "{}", buf.finish())?;

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
