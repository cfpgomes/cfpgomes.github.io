use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write as OtherWrite;
use std::path::PathBuf;
use std::sync::mpsc;

use image_compressor::Factor;
use image_compressor::FolderCompressor;

use image::io::Reader as ImageReader;
use image::{GenericImage, GenericImageView, Pixel, Rgba, RgbaImage};
use std::io::Cursor;

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

fn apply_white_overlay_to_images(from_dest_dir: &str, to_dest_dir: &str) {
    let dir_entries = fs::read_dir(from_dest_dir).unwrap();
    fs::create_dir_all(to_dest_dir).unwrap();

    for dir_entry in dir_entries {
        let mut img = ImageReader::open(dir_entry.as_ref().unwrap().path())
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();

        for px in img.pixels_mut() {
            px.blend(Rgba::from_slice(&[255, 255, 255, 229]));
        }

        println!(
            "{}/{}",
            to_dest_dir,
            dir_entry
                .as_ref()
                .unwrap()
                .file_name()
                .into_string()
                .unwrap(),
        );

        img.save(format!(
            "{}/{}",
            to_dest_dir,
            dir_entry
                .as_ref()
                .unwrap()
                .file_name()
                .into_string()
                .unwrap(),
        ))
        .unwrap();
    }
}

fn compress_images(from_dest_dir: &str, to_dest_dir: &str) {
    // Compress images in `img` folder for web
    println!("Compression of images started!");
    let origin = PathBuf::from(from_dest_dir); // original directory path
    let dest = PathBuf::from(to_dest_dir); // destination directory path
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
}

enum CSS {
    Homemade,
    Science,
}

struct Page {
    title: String,
    css: CSS,
    buf: Buffer,
}

impl Page {
    fn new(title: &str, css: CSS) -> Self {
        let mut page = Self {
            title: title.to_string(),
            css: css,
            buf: Buffer::new(),
        };

        // Header content
        writeln!(page.buf, "<!-- My website -->").unwrap();

        // The Html5 trait provides various helper methods.  For instance, doctype()
        // simply writes the <!DOCTYPE> header
        page.buf.doctype();

        // Most helper methods create child nodes.  You can set a node's attributes
        // like so
        let mut html = page.buf.html().attr("lang='en'");

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
        writeln!(head.title(), "Cláudio Gomes | {}", title).unwrap();

        // Description is the same as title.
        head.meta()
            .attr(format!("name='description' content='{}'", title).as_str());

        // Necessary stylesheets.
        head.link()
            .attr("rel='stylesheet' href='https://unpkg.com/spectre.css/dist/spectre.min.css'");
        head.link()
            .attr("rel='stylesheet' href='https://unpkg.com/spectre.css/dist/spectre-exp.min.css'");
        head.link()
            .attr("rel='stylesheet' href='https://unpkg.com/spectre.css/dist/spectre-icons.min.css'");
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
            var imagesArray = ['carousel1.jpg', 'carousel2.jpg', 'carousel3.jpg', 'carousel4.jpg', 'carousel5.jpg', 'carousel6.jpg', 'carousel7.jpg', 'carousel8.jpg', 'carousel9.jpg', 'carousel10.jpg', 'carousel11.jpg', 'carousel12.jpg', 'carousel13.jpg', 'carousel14.jpg', 'carousel15.jpg', 'carousel16.jpg'];
        
            //Preload images for faster page response
            for (var i=0; i < imagesArray.length; i++) {{
                preload_image_object.src = imagesArray[i];
                preload_image_object.onload = console.log(i);
            }};
            
            document.getElementById('background-image-id').style.backgroundImage = 'url(\"./compressed-img/' + imagesArray[Math.floor(Math.random() * 16)] + '\")';
        }}
        ").unwrap();

        // Body
        let mut body = html
            .body()
            .attr("class='gallery-background' id='background-image-id' onload='onLoad()'");
        // Container to apply shadow
        let shadow_gradient = body.div().attr("class='special-shadow-gradient'");

        page
    }

    fn add_top_bar(
        &mut self,
        path_img: &str,
        page_a: &str,
        page_b: &str,
        page_c: &str,
        page_d: &str,
        icon_a: &str,
        icon_b: &str,
        icon_c: &str,
        icon_d: &str,
        active_page: Option<&str>,
    ) {
        // Desktop top bar
        let mut container = self.buf.div().attr("class='top-bar hide-xl'");
        let mut columns = container
            .div()
            .attr("class='columns col-gapless full-height'");
        let mut column_a = columns.div().attr("class='column col-2-and-half'");
        let mut button_a = column_a.button().attr("class='btn btn-top-bar'");
        write!(button_a, "{}", page_a);
        let mut column_b = columns.div().attr("class='column col-2-and-half'");
        let mut button_b = column_b.button().attr("class='btn btn-top-bar'");
        write!(button_b, "{}", page_b);

        let mut column_pic = columns.div().attr("class='column col-2'");
        let mut parallax_pic = column_pic.div().attr("class='parallax square-pic-parallax'");
        let mut parallax_top_left = parallax_pic.div().attr("class='parallax-top-left' tabindex='1'");
        let mut parallax_top_right = parallax_pic.div().attr("class='parallax-top-right' tabindex='2'");
        let mut parallax_bottom_left = parallax_pic.div().attr("class='parallax-bottom-left' tabindex='3'");
        let mut parallax_bottom_right = parallax_pic.div().attr("class='parallax-bottom-right' tabindex='4'");
        let mut parallax_content = parallax_pic.div().attr("class='parallax-content'");
        let mut parallax_front = parallax_content.div().attr("class='parallax-front'");
        let mut parallax_back = parallax_content.div().attr("class='parallax-back'");
        parallax_back
            .div()
            .attr(format!("style='background-image:url(\"{}\")' class='square-pic-img'", path_img).as_ref());

        let mut column_c = columns.div().attr("class='column col-2-and-half'");
        let mut button_c = column_c.button().attr("class='btn btn-top-bar'");
        write!(button_c, "{}", page_c);

        let mut column_d = columns.div().attr("class='column col-2-and-half'");
        let mut button_d = column_d.button().attr("class='btn btn-top-bar'");
        write!(button_d, "{}", page_d);

        // Mobile top bar
        let mut container = self.buf.div().attr("class='top-bar-mobile show-xl'");
        let mut columns = container
            .div()
            .attr("class='columns col-gapless full-height'");
        let mut column_a = columns.div().attr("class='column col-2-and-quarter'");
        let mut button_a = column_a.button().attr(format!("class='btn btn-top-bar-mobile fa-solid {}'", icon_a).as_ref());

        let mut column_b = columns.div().attr("class='column col-2-and-quarter'");
        let mut button_b = column_b.button().attr(format!("class='btn btn-top-bar-mobile fa-solid {}'", icon_b).as_ref());

        let mut column_home = columns.div().attr("class='column col-3'");
        column_home.button().attr(format!("style='background-image:url(\"{}\")' class='btn btn-home-top-bar-mobile'", path_img).as_ref());

        let mut column_c = columns.div().attr("class='column col-2-and-quarter'");
        let mut button_c = column_c.button().attr(format!("class='btn btn-top-bar-mobile fa-solid {}'", icon_c).as_ref());

        let mut column_d = columns.div().attr("class='column col-2-and-quarter'");
        let mut button_d = column_d.button().attr(format!("class='btn btn-top-bar-mobile fa-solid {}'", icon_d).as_ref());
    }

    fn publish(self, path: &str) {
        let mut index_file = File::create(path).unwrap();
        write!(index_file, "{}", self.buf.finish());
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
    fn new(title: &str, date: NaiveDate, markdown: &str, tags: Vec<String>) -> Self {
        Self {
            title: title.to_string(),
            date: date,
            markdown: markdown.to_string(),
            tags: tags,
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
    // Apply white color with opacity of 0.9 to background images
    // apply_white_overlay_to_images("img", "white-img");
    compress_images("white-img", "compressed-img");

    // Create "Homepage" Page
    let mut page_homepage = Page::new("Homepage", CSS::Science);

    // Create "Who am I?" Page
    let mut page_who_am_i = Page::new("Who am I?", CSS::Science);

    // Create "Publications" Page
    let mut page_publications = Page::new("Publications", CSS::Science);

    // Create "Miscellaneous" Page
    let mut page_miscellaneous = Page::new("Miscellaneous", CSS::Science);

    // Create "CV" Page
    let mut page_cv = Page::new("CV", CSS::Science);

    // Add top bar to every page
    page_homepage.add_top_bar(
        "profile_pic.png",
        "Who am I?",
        "Publications",
        "Miscellaneous",
        "CV",
        "fa-user",
        "fa-atom",
        "fa-cow",
        "fa-envelope-open-text",
        None,
    );
    /*
    page_who_am_i.add_top_bar(
        "profile_pic.jpg",
        "Who am I?",
        "Publications",
        "Miscellaneous",
        "CV",
        "Who am I?",
    );
    page_publications.add_top_bar(
        "profile_pic.jpg",
        "Who am I?",
        "Publications",
        "Miscellaneous",
        "CV",
        "Publications",
    );
    page_miscellaneous.add_top_bar(
        "profile_pic.jpg",
        "Who am I?",
        "Publications",
        "Miscellaneous",
        "CV",
        "Miscellaneous",
    );
    page_cv.add_top_bar(
        "profile_pic.jpg",
        "Who am I?",
        "Publications",
        "Miscellaneous",
        "CV",
        "CV",
    );

    // Add social media buttons
    let social_media = vec![
        (SocialMedia::Twitter, "cfpgomes"),
        (SocialMedia::GoogleScholar, "cfpgomes"),
        (SocialMedia::GitHub, "cfpgomes"),
        (SocialMedia::LinkedIn, "cfpgomes"),
        (SocialMedia::ORCID, "cfpgomes"),
    ];

    page_homepage.add_social_media_buttons(social_media);
    page_who_am_i.add_social_media_buttons(social_media);
    page_publications.add_social_media_buttons(social_media);
    page_miscellaneous.add_social_media_buttons(social_media);
    page_cv.add_social_media_buttons(social_media);

    */
    //// "Homepage" Page Building process

    // Add "Who am I?" section to "Homepage" page
    // Add "Publications" and "Miscellaneous" section to "Homepage" page.
    // (publications and miscellaneous, once clicked, should open dedicated
    // page to that publication or miscellaneous)
    // Add "CV" section to "Homepage" page
    // Save page as index.html

    page_homepage.publish("index.html");

    //// "Who am I?" Page Building process
    // TODO: Descobrir depois o que meter, contar narrativa gira

    //// "Publications" Page Building process
    // Add every publication to this page, in the same way as in the homepage.
    // Every publication should expand to read in its entirety.

    //// "Miscellaneous" Page Building process
    // Add every miscellaneous to this page, in the same way as in the homepage.
    // Every miscellaneous should have a thumbnail as header.
    // Every miscellaneous should expand to read in its entirety.

    //// "CV" Page Building process
    // TODO: Descobrir depois o que meter, estrutura gira de CV, printable to pdf idealmente

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
