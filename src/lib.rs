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
use image::{Pixel, Rgba};

use chrono::NaiveDate;

use comrak::{markdown_to_html, ComrakOptions};

use html_builder::*;

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

fn titlefy_to_page_title(title: &str) -> String {
    return title
        .to_lowercase()
        .replace(|c: char| !c.is_ascii(), "")
        .replace(|c: char| c.is_ascii_whitespace(), "-")
        .replace(|c: char| !(c.is_alphanumeric() || c == '-'), "");
}

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
        head.link().attr(
            "rel='stylesheet' href='https://unpkg.com/spectre.css/dist/spectre-icons.min.css'",
        );
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

    fn new_without_background(title: &str, css: CSS) -> Self {
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
        head.link().attr(
            "rel='stylesheet' href='https://unpkg.com/spectre.css/dist/spectre-icons.min.css'",
        );
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


        // Body
        let mut body = html
            .body();
        // Container to apply shadow
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
            .attr("class='columns col-gapless full-height ultra-wide-treatment'");
        let mut column_a = columns.div().attr("class='column col-2-and-half'");
        let mut button_a = column_a.button().attr("class='btn btn-top-bar'");
        write!(button_a, "{}", page_a);
        let mut column_b = columns.div().attr("class='column col-2-and-half'");
        let mut button_b = column_b.button().attr("class='btn btn-top-bar'");
        write!(button_b, "{}", page_b);

        let mut column_pic = columns.div().attr("class='column col-2'");
        let mut parallax_pic = column_pic
            .div()
            .attr("class='parallax square-pic-parallax'");
        let mut parallax_top_left = parallax_pic
            .div()
            .attr("class='parallax-top-left' tabindex='1'");
        let mut parallax_top_right = parallax_pic
            .div()
            .attr("class='parallax-top-right' tabindex='2'");
        let mut parallax_bottom_left = parallax_pic
            .div()
            .attr("class='parallax-bottom-left' tabindex='3'");
        let mut parallax_bottom_right = parallax_pic
            .div()
            .attr("class='parallax-bottom-right' tabindex='4'");
        let mut parallax_content = parallax_pic.div().attr("class='parallax-content'");
        let mut parallax_front = parallax_content.div().attr("class='parallax-front'");
        let mut parallax_back = parallax_content.div().attr("class='parallax-back'");
        parallax_back.div().attr(
            format!(
                "style='background-image:url(\"{}\")' class='square-pic-img'",
                path_img
            )
            .as_ref(),
        );

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
        let mut button_a = column_a
            .button()
            .attr(format!("class='btn btn-top-bar-mobile fa-solid {}'", icon_a).as_ref());

        let mut column_b = columns.div().attr("class='column col-2-and-quarter'");
        let mut button_b = column_b
            .button()
            .attr(format!("class='btn btn-top-bar-mobile fa-solid {}'", icon_b).as_ref());

        let mut column_home = columns.div().attr("class='column col-3'");
        column_home.button().attr(
            format!(
                "style='background-image:url(\"{}\")' class='btn btn-home-top-bar-mobile'",
                path_img
            )
            .as_ref(),
        );

        let mut column_c = columns.div().attr("class='column col-2-and-quarter'");
        let mut button_c = column_c
            .button()
            .attr(format!("class='btn btn-top-bar-mobile fa-solid {}'", icon_c).as_ref());

        let mut column_d = columns.div().attr("class='column col-2-and-quarter'");
        let mut button_d = column_d
            .button()
            .attr(format!("class='btn btn-top-bar-mobile fa-solid {}'", icon_d).as_ref());

        self.buf.div().attr("class='top-bar-invisible hide-xl'");
        self.buf
            .div()
            .attr("class='top-bar-mobile-invisible show-xl'");
    }
    
    fn add_simple_top_bar(
        &mut self,
        page_a: &str,
        page_b: &str,
        page_c: &str,
        page_d: &str,
        icon_a: &str,
        icon_b: &str,
        icon_c: &str,
        icon_d: &str,
        active_page: Option<&str>,
        link_a: Option<&str>,
        link_b: Option<&str>,
        link_c: Option<&str>,
        link_d: Option<&str>,
    ) {
        // Desktop top bar
        let mut container = self.buf.div().attr("class='top-bar hide-xl'");
        let mut columns = container
            .div()
            .attr("class='columns col-gapless full-height ultra-wide-treatment'");
        let mut column_a = columns.div().attr(format!("class='column col-3' onclick='window.location.href=\"{}\";'", match link_a {
            Some(link) => link,
            _ => ""
        }).as_ref());
        let mut button_a = column_a.button().attr("class='btn btn-top-bar'");
        write!(button_a, "{}", page_a);
        
        let mut column_b = columns.div().attr(format!("class='column col-3' onclick='window.location.href=\"{}\";'", match link_b {
            Some(link) => link,
            _ => ""
        }).as_ref());
        let mut button_b = column_b.button().attr("class='btn btn-top-bar'");
        write!(button_b, "{}", page_b);

        let mut column_c = columns.div().attr(format!("class='column col-3' onclick='window.location.href=\"{}\";'", match link_c {
            Some(link) => link,
            _ => ""
        }).as_ref());
        let mut button_c = column_c.button().attr("class='btn btn-top-bar'");
        write!(button_c, "{}", page_c);

        let mut column_d = columns.div().attr(format!("class='column col-3' onclick='window.location.href=\"{}\";'", match link_d {
            Some(link) => link,
            _ => ""
        }).as_ref());
        let mut button_d = column_d.button().attr("class='btn btn-top-bar'");
        write!(button_d, "{}", page_d);

        // Mobile top bar
        let mut container = self.buf.div().attr("class='top-bar-mobile show-xl'");
        let mut columns = container
            .div()
            .attr("class='columns col-gapless full-height'");
        let mut column_a = columns.div().attr(format!("class='column col-3' onclick='window.location.href=\"{}\";'", match link_a {
            Some(link) => link,
            _ => ""
        }).as_ref());
        let mut button_a = column_a
            .button()
            .attr(format!("class='btn btn-top-bar-mobile fa-solid {}'", icon_a).as_ref());

        let mut column_b = columns.div().attr(format!("class='column col-3' onclick='window.location.href=\"{}\";'", match link_b {
            Some(link) => link,
            _ => ""
        }).as_ref());
        let mut button_b = column_b
            .button()
            .attr(format!("class='btn btn-top-bar-mobile fa-solid {}'", icon_b).as_ref());

        let mut column_c = columns.div().attr(format!("class='column col-3' onclick='window.location.href=\"{}\";'", match link_c {
            Some(link) => link,
            _ => ""
        }).as_ref());
        let mut button_c = column_c
            .button()
            .attr(format!("class='btn btn-top-bar-mobile fa-solid {}'", icon_c).as_ref());

        let mut column_d = columns.div().attr(format!("class='column col-3' onclick='window.location.href=\"{}\";'", match link_d {
            Some(link) => link,
            _ => ""
        }).as_ref());
        let mut button_d = column_d
            .button()
            .attr(format!("class='btn btn-top-bar-mobile fa-solid {}'", icon_d).as_ref());

        self.buf.div().attr("class='top-bar-invisible hide-xl'");
        self.buf
            .div()
            .attr("class='top-bar-mobile-invisible show-xl'");
    }

    fn add_columns(&mut self, classes: &str) -> Node {
        let columns = self.buf.div().attr(
            format!(
                "class='columns page-height page-ultra-wide-treatment {}'",
                classes
            )
            .as_ref(),
        );
        columns
    }

    fn add_figure(&mut self, figure_path: &str) {
        let mut figure_file = File::open(figure_path).unwrap();
        let mut figure_str = String::new();
        figure_file.read_to_string(&mut figure_str).unwrap();
        write!(self.buf, "{}", figure_str).unwrap();
    }

    fn add_footer(&mut self) {
        self.buf.footer();
    }

    fn publish(self, path: &str) {
        let mut index_file = File::create(path).unwrap();
        write!(index_file, "{}", self.buf.finish()).unwrap();
    }
}

// Add responsive column to columns div
fn add_column_to_dual_columns<'a>(columns: &'a mut Node) -> Node<'a> {
    let mut col = columns.div().attr("class='column col-6 col-xl-12'");

    col
}

fn add_figure_to_node(node: &mut Node, figure_path: &str){
    let mut figure_file = File::open(figure_path).unwrap();
    let mut figure_str = String::new();
    figure_file.read_to_string(&mut figure_str).unwrap();
    write!(node, "{}", figure_str).unwrap();
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

    fn from_gobbets_in_folder(folder_path: &str) -> Result<Vec<Self>, &'static str> {
        let paths = match fs::read_dir(folder_path) {
            Ok(p) => p,
            _ => return Err("Couldn't open folder."),
        };

        let mut posts = Vec::<Self>::new();

        for path in paths {
            let path_str = format!("{}", path.unwrap().path().display());
            if path_str.ends_with(".gobbet") {
                let post = Self::from_gobbet(&path_str).unwrap();
                posts.push(post);
            }
        }

        posts.sort_by(|a, b| b.date.cmp(&a.date));

        Ok(posts)
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
    let posts = Publication::from_gobbets_in_folder("publications").unwrap();

    let papers = posts
        .into_iter()
        .filter(|p| p.tags.contains(&String::from("paper")))
        .collect::<Vec<_>>();

    for paper in &papers {
        let mut page_paper = Page::new(&paper.title, CSS::Science);
        page_paper.add_top_bar(
            "profile_pic.png",
            "Who am I?",
            "Publications",
            "Miscellaneous",
            "CV",
            "fa-person-rays",
            "fa-atom",
            "fa-cow",
            "fa-address-book",
            Some("Publications"),
        );

        let mut columns = page_paper.add_columns("");
        let mut col = add_column_to_dual_columns(&mut columns);
        let mut cont = col.div().attr("class='blank-container'");
        write!(cont.h1(), "{}", paper.title)?;
        write!(cont.h2(), "{}", paper.date)?;
        write!(cont, "{}", paper.to_html())?;

        page_paper.add_footer();

        page_paper
            .publish(format!("publications-{}.html", titlefy_to_page_title(&paper.title)).as_str());
    }

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

    // Create "SMS" Pages
    let mut page_sms_article_quality = Page::new_without_background("Systematic Mapping Study", CSS::Science);
    let mut page_sms_article_individual = Page::new_without_background("Systematic Mapping Study", CSS::Science);
    let mut page_sms_article_pairwise = Page::new_without_background("Systematic Mapping Study", CSS::Science);

    // Add top bar to every page
    page_homepage.add_top_bar(
        "profile_pic.png",
        "Who am I?",
        "Publications",
        "Miscellaneous",
        "CV",
        "fa-person-rays",
        "fa-atom",
        "fa-cow",
        "fa-address-book",
        None,
    );

    
    page_sms_article_quality.add_simple_top_bar(
        "Quality<br><br>Assessment",
        "Individual<br><br>Analysis",
        "Pairwise<br><br>Analysis",
        "Article<br><br>Page",
        "fa-check",
        "fa-folder",
        "fa-folder-tree",
        "fa-address-book",
        Some("Quality<br><br>Assessment"),
        Some("smsquality.html"),
        Some("sms.html"),
        Some("smspairwise.html"),
        Some("https://scholar.google.com"),
    );

    page_sms_article_individual.add_simple_top_bar(
        "Quality<br><br>Assessment",
        "Individual<br><br>Analysis",
        "Pairwise<br><br>Analysis",
        "Article<br><br>Page",
        "fa-check",
        "fa-folder",
        "fa-folder-tree",
        "fa-address-book",
        Some("Individual<br><br>Analysis"),
        Some("smsquality.html"),
        Some("sms.html"),
        Some("smspairwise.html"),
        Some("https://scholar.google.com"),
    );

    page_sms_article_pairwise.add_simple_top_bar(
        "Quality<br><br>Assessment",
        "Individual<br><br>Analysis",
        "Pairwise<br><br>Analysis",
        "Article<br><br>Page",
        "fa-check",
        "fa-folder",
        "fa-folder-tree",
        "fa-address-book",
        Some("Pairwise<br><br>Analysis"),
        Some("smsquality.html"),
        Some("sms.html"),
        Some("smspairwise.html"),
        Some("https://scholar.google.com"),
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

    let mut columns_who_am_i_section = page_homepage.add_columns("");
    let mut col_intro = add_column_to_dual_columns(&mut columns_who_am_i_section);
    let mut container_intro = col_intro.div().attr("class='blank-container'");
    write!(
        container_intro.h1().b(),
        "Hey there! 👋<br>Cláudio Gomes here."
    )?;
    write!(container_intro.h3(), "I'm a dual-degree PhD student with one foot at the Carnegie Mellon University and the other foot at the University of Porto. Pretty exciting, I know! 😄")?;
    write!(container_intro.h3(), "My research focuses on sustainable forms of computing, such as <b>quantum computing</b>. Massive amounts of information are handled and processed by small, big and super computers all over the world. Any alternative form of computing that offers the same processing power for a lot less energy would have a tremendous benefit for society! <b>My goal is to augment classical computing with sustainable alternatives.</b> 🌳")?;
    write!(
        container_intro.h3(),
        "Welcome to my webpage and feel free to contact me! 🤠"
    )?;

    let mut col_contacts = add_column_to_dual_columns(&mut columns_who_am_i_section);
    let mut container_contacts = col_contacts.div().attr("class='blank-container'");
    write!(container_contacts.h1().b(), "Find me in those places! 👇")?;

    let mut col_buttons = container_contacts.div().attr("class='columns'");
    let mut button_email = col_buttons.a().attr("class='col-4 tooltip tooltip-left' data-tooltip='Send an email!'  href='mailto:claudiogomes@cmu.edu'");
    let mut icon_email = button_email.div().attr("class='grid_button'");
    icon_email
        .i()
        .attr("class='grid_icon fa-solid fa-envelope'");
    let mut button_github = col_buttons
        .a()
        .attr("class='col-4 tooltip' data-tooltip='GitHub' href='https://github.com/cfpgomes'");
    let mut icon_github = button_github.div().attr("class='grid_button'");
    icon_github
        .i()
        .attr("class='grid_icon fa-brands fa-github'");
    let mut button_twitter = col_buttons.a().attr("class='col-4 tooltip tooltip-right' data-tooltip='Twitter' href='https://twitter.com/cfpgomes'");
    let mut icon_twitter = button_twitter.div().attr("class='grid_button'");
    icon_twitter
        .i()
        .attr("class='grid_icon fa-brands fa-twitter'");

    let mut button_scholar = col_buttons.a().attr("class='col-4 tooltip tooltip-left' data-tooltip='Google Scholar' href='https://scholar.google.com/citations?user=xlm7eBYAAAAJ'");
    let mut icon_scholar = button_scholar.div().attr("class='grid_button'");
    icon_scholar
        .img()
        .attr("src='svg/Google_Scholar_logo.svg' class='svg_icon'");
    let mut button_research_gate = col_buttons.a().attr("class='col-4 tooltip' data-tooltip='ResearchGate' href='https://www.researchgate.net/profile/Claudio-Gomes-12'");
    let mut icon_research_gate = button_research_gate.div().attr("class='grid_button'");
    icon_research_gate
        .i()
        .attr("class='grid_icon fa-brands fa-researchgate'");
    let mut button_orcid = col_buttons.a().attr("class='col-4 tooltip tooltip-right' data-tooltip='ORCID' href='https://orcid.org/0000-0001-6292-0222'");
    let mut icon_orcid = button_orcid.div().attr("class='grid_button'");
    icon_orcid.i().attr("class='grid_icon fa-brands fa-orcid'");

    let mut button_linkedin = col_buttons.a().attr("class='col-4 tooltip tooltip-left' data-tooltip='LinkedIn' href='https://www.linkedin.com/in/cfpgomes'");
    let mut icon_linkedin = button_linkedin.div().attr("class='grid_button'");
    icon_linkedin
        .i()
        .attr("class='grid_icon fa-brands fa-linkedin-in'");
    let mut button_unsplash = col_buttons.a().attr("class='col-4 tooltip tooltip-bottom' data-tooltip='Unsplash' href='https://unsplash.com/@cfpgomes'");
    let mut icon_unsplash = button_unsplash.div().attr("class='grid_button'");
    icon_unsplash
        .i()
        .attr("class='grid_icon fa-brands fa-unsplash'");
    let mut button_cmu_portugal = col_buttons.a().attr("class='col-4 tooltip tooltip-right' data-tooltip='CMU Portugal' href='https://www.cmuportugal.org/students/claudio-filipe-prata-gomes'");
    let mut icon_cmu_portugal = button_cmu_portugal.div().attr("class='grid_button'");
    icon_cmu_portugal
        .img()
        .attr("src='svg/cmu_portugal.svg' class='svg_icon'");

    // Add "Publications" and "Miscellaneous" section to "Homepage" page.
    // (publications and miscellaneous, once clicked, should open dedicated
    // page to that publication or miscellaneous)
    let mut columns_pub_misc_section = page_homepage.add_columns("blank-background");

    let mut col_pubs = add_column_to_dual_columns(&mut columns_pub_misc_section);
    let mut container_pubs = col_pubs.div().attr("class='blank-container'");
    write!(container_pubs.h1().b(), "Check out my research! 💡")?;

    // Get latest post
    let latest_post = &papers[0];
    let mut latest_post_container = container_pubs.a().attr(
        format!(
            "class='article_container' href='publications-{}.html'",
            titlefy_to_page_title(&latest_post.title)
        )
        .as_str(),
    );
    write!(
        latest_post_container.h4().attr("class='no_margin'").b(),
        "{}",
        latest_post.title
    )?;
    write!(
        latest_post_container
            .h5()
            .attr("class='date_color no_margin'"),
        "{}",
        latest_post.date
    )?;
    write!(
        latest_post_container
            .p()
            .attr("class='auto_crop no_margin'"),
        "{}",
        latest_post.markdown
    )?;

    // Get second latest post
    let second_latest_post = &papers[1];
    let mut second_latest_post_container = container_pubs.a().attr(
        format!(
            "class='article_container' href='publications-{}.html'",
            titlefy_to_page_title(&second_latest_post.title)
        )
        .as_str(),
    );
    write!(
        second_latest_post_container
            .h4()
            .attr("class='no_margin'")
            .b(),
        "{}",
        second_latest_post.title
    )?;
    write!(
        second_latest_post_container
            .h5()
            .attr("class='date_color no_margin'"),
        "{}",
        second_latest_post.date
    )?;
    write!(
        second_latest_post_container
            .p()
            .attr("class='auto_crop no_margin'"),
        "{}",
        second_latest_post.markdown
    )?;

    let mut col_pubs = add_column_to_dual_columns(&mut columns_pub_misc_section);
    let mut container_pubs = col_pubs.div().attr("class='blank-container'");
    write!(container_pubs.h1().b(), "And also some random stuff...")?;

    // Add "CV" section to "Homepage" page
    let mut columns_cv_section = page_homepage.add_columns("");
    let mut col_cv_1 = add_column_to_dual_columns(&mut columns_cv_section);
    let mut container_cv_1 = col_cv_1.div().attr("class='blank-container'");
    write!(container_cv_1.h1(), "Hey! I'm Cláudio Gomes.")?;
    write!(container_cv_1.h3(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Id eu nisl nunc mi ipsum faucibus vitae aliquet nec. Congue eu consequat ac felis. Quisque sagittis purus sit amet volutpat consequat. Suspendisse in est ante in nibh mauris. Enim blandit volutpat maecenas volutpat blandit aliquam etiam erat velit. Mattis vulputate enim nulla aliquet. Aliquam ultrices sagittis orci a scelerisque purus semper eget. Viverra mauris in aliquam sem fringilla ut morbi. Egestas fringilla phasellus faucibus scelerisque eleifend. Volutpat sed cras ornare arcu. Enim lobortis scelerisque fermentum dui. Magna etiam tempor orci eu lobortis elementum nibh. Quis blandit turpis cursus in hac habitasse platea dictumst. Sed blandit libero volutpat sed cras ornare arcu dui.")?;

    let mut col_cv_2 = add_column_to_dual_columns(&mut columns_cv_section);
    let mut container_cv_2 = col_cv_2.div().attr("class='blank-container'");
    write!(container_cv_2.h1(), "Here are my contacts:")?;
    write!(container_cv_2.h3(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Id eu nisl nunc mi ipsum faucibus vitae aliquet nec. Congue eu consequat ac felis. Quisque sagittis purus sit amet volutpat consequat. Suspendisse in est ante in nibh mauris. Enim blandit volutpat maecenas volutpat blandit aliquam etiam erat velit. Mattis vulputate enim nulla aliquet. Aliquam ultrices sagittis orci a scelerisque purus semper eget. Viverra mauris in aliquam sem fringilla ut morbi. Egestas fringilla phasellus faucibus scelerisque eleifend. Volutpat sed cras ornare arcu. Enim lobortis scelerisque fermentum dui. Magna etiam tempor orci eu lobortis elementum nibh. Quis blandit turpis cursus in hac habitasse platea dictumst. Sed blandit libero volutpat sed cras ornare arcu dui.")?;

    // Add footer to "Homepage" page
    page_homepage.add_footer();

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

    
    //// "sms_article" Pages Building process
    //// Quality Assessment Page
    
    let mut sms_quality_title = page_sms_article_quality.buf.div().attr("class='blank-container-square-title page-ultra-wide-treatment'");
    write!(
        sms_quality_title.h1(),
        "Quality Assessment"
    )?;

    let mut sms_quality_sstotal_vs_srtotal = page_sms_article_quality.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_quality_sstotal_vs_srtotal.h4(),
        "Scatter plot of the S<sub>S</sub> and S<sub>R</sub> values of each of the selected studies. Blue dots represent the selected studies, and the red line is a visual aid to mark the diagonal of the chart. All the dots in a perfect positive correlation between both scores would overlap the line."
    )?;
    add_figure_to_node(&mut sms_quality_sstotal_vs_srtotal, "sms_figures/quality_sstotal_vs_srtotal.html");

    let mut sms_quality_sstotal_vs_sjr = page_sms_article_quality.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_quality_sstotal_vs_sjr.h4(),
        "Scatter plot of the S<sub>S</sub> value and SJR Indicator of each of the selected studies. Blue dots represent the selected studies."
    )?;
    add_figure_to_node(&mut sms_quality_sstotal_vs_sjr, "sms_figures/quality_sstotal_vs_sjr.html");

    let mut sms_quality_srtotal_vs_sjr = page_sms_article_quality.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_quality_srtotal_vs_sjr.h4(),
        "Scatter plot of the S<sub>R</sub> value and SJR Indicator of each of the selected studies. Blue dots represent the selected studies."
    )?;
    add_figure_to_node(&mut sms_quality_srtotal_vs_sjr, "sms_figures/quality_srtotal_vs_sjr.html");

    let mut sms_quality_sstotal_vs_sjrquartile = page_sms_article_quality.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_quality_sstotal_vs_sjrquartile.h4(),
        "Box plots of the S<sub>S</sub> value of the selected studies, grouped by their SJR Best Quartile. Blue dots represent the selected studies."
    )?;
    add_figure_to_node(&mut sms_quality_sstotal_vs_sjrquartile, "sms_figures/quality_sstotal_vs_sjrquartile.html");

    let mut sms_quality_srtotal_vs_sjrquartile = page_sms_article_quality.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_quality_srtotal_vs_sjrquartile.h4(),
        "Box plots of the S<sub>R</sub> value of the selected studies, grouped by their SJR Best Quartile. Blue dots represent the selected studies."
    )?;
    add_figure_to_node(&mut sms_quality_srtotal_vs_sjrquartile, "sms_figures/quality_srtotal_vs_sjrquartile.html");

    let mut sms_quality_sstotal_vs_core = page_sms_article_quality.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_quality_sstotal_vs_core.h4(),
        "Box plots of the S<sub>S</sub> value of the selected studies, grouped by their CORE Ranking. Blue dots represent the selected studies."
    )?;
    add_figure_to_node(&mut sms_quality_sstotal_vs_core, "sms_figures/quality_sstotal_vs_core.html");

    let mut sms_quality_srtotal_vs_core = page_sms_article_quality.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_quality_srtotal_vs_core.h4(),
        "Box plots of the S<sub>R</sub> value of the selected studies, grouped by their CORE Ranking. Blue dots represent the selected studies."
    )?;
    add_figure_to_node(&mut sms_quality_srtotal_vs_core, "sms_figures/quality_srtotal_vs_core.html");

    //// Individual Analysis Page
    let mut sms_individual_title = page_sms_article_individual.buf.div().attr("class='blank-container-square-title page-ultra-wide-treatment'");
    write!(
        sms_individual_title.h1(),
        "Analysis of Individual Features"
    )?;

    let mut sms_individual_ss00_sr00 = page_sms_article_individual.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_individual_ss00_sr00.h4(),
        "Violin plots of the S<sub>S</sub> and S<sub>R</sub> values of the selected studies. Black dots represent the outliers. The black dashed line represents the median."
    )?;
    add_figure_to_node(&mut sms_individual_ss00_sr00, "sms_figures/individual_ss00_sr00.html");
    
    let mut sms_individual_ss_counts = page_sms_article_individual.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_individual_ss_counts.h4(),
        "Stacked bar charts with the proportions of the different responses (SS01 to SS10) collected for the checklist to assess the quality of the study."
    )?;
    add_figure_to_node(&mut sms_individual_ss_counts, "sms_figures/individual_ss_counts.html");
    
    let mut sms_individual_sr_counts = page_sms_article_individual.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_individual_sr_counts.h4(),
        "Stacked bar charts with the proportions of the different responses (SR01 to SR16) collected for the checklist to assess the quality of the reporting."
    )?;
    add_figure_to_node(&mut sms_individual_sr_counts, "sms_figures/individual_sr_counts.html");

    let mut sms_individual_D010 = page_sms_article_individual.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_individual_D010.h4(),
        "Stacked bar chart with the proportions of the different responses collected for feature D010."
    )?;
    add_figure_to_node(&mut sms_individual_D010, "sms_figures/individual_D010.html");
    
    let mut sms_individual_D030 = page_sms_article_individual.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_individual_D030.h4(),
        "Stacked bar chart with the proportions of the different responses collected for feature D030."
    )?;
    add_figure_to_node(&mut sms_individual_D030, "sms_figures/individual_D030.html");
    
    let mut sms_individual_D040 = page_sms_article_individual.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_individual_D040.h4(),
        "Stacked bar chart with the proportions of the different responses collected for feature D040."
    )?;
    add_figure_to_node(&mut sms_individual_D040, "sms_figures/individual_D040.html");

    let mut sms_individual_D070 = page_sms_article_individual.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_individual_D070.h4(),
        "Stacked bar chart with the proportions of the different responses collected for feature D070."
    )?;
    add_figure_to_node(&mut sms_individual_D070, "sms_figures/individual_D070.html");

    let mut sms_individual_D020 = page_sms_article_individual.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_individual_D020.h4(),
        "Bar chart with the distribution of the different responses collected for feature D020."
    )?;
    add_figure_to_node(&mut sms_individual_D020, "sms_figures/individual_D020.html");

    let mut sms_individual_D060 = page_sms_article_individual.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_individual_D060.h4(),
        "Bar chart with the distribution of the different responses collected for feature D060. <b>Note that the bar for year 2022 only contains publications made in the first four months.</b>"
    )?;
    add_figure_to_node(&mut sms_individual_D060, "sms_figures/individual_D060.html");

    
    let mut sms_individual_D050_to_D053_treemap = page_sms_article_individual.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_individual_D050_to_D053_treemap.h4(),
        "Treemap chart of the selected studies according to their section, division, group, and class from the International Standard Industrial Classification of All Economic Activities (ISIC)."
    )?;
    add_figure_to_node(&mut sms_individual_D050_to_D053_treemap, "sms_figures/individual_D050_to_D053_treemap.html");
    
    let mut sms_individual_D050_to_D053_icicle = page_sms_article_individual.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_individual_D050_to_D053_icicle.h4(),
        "Icicle chart of the selected studies according to their section, division, group, and class from the International Standard Industrial Classification of All Economic Activities (ISIC)."
    )?;
    add_figure_to_node(&mut sms_individual_D050_to_D053_icicle, "sms_figures/individual_D050_to_D053_icicle.html");
    
    //// Pairwise Analysis Page
    let mut sms_pairwise_title = page_sms_article_pairwise.buf.div().attr("class='blank-container-square-title page-ultra-wide-treatment'");
    write!(
        sms_pairwise_title.h1(),
        "Analysis of Relationships between Pairs Features"
    )?;

    let mut sms_pair_ss00_d010 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_pair_ss00_d010.h4(),
        "<b>Pair 1:</b> Violin plots of the distributions of the quality of the study scores (S<sub>S</sub>) of the selected studies, grouped by their type of approach (D010)."
    )?;
    add_figure_to_node(&mut sms_pair_ss00_d010, "sms_figures/pair_ss00_d010.html");

    let mut sms_pair_ss00_d020 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_pair_ss00_d020.h4(),
        "<b>Pair 2:</b> Violin plots of the distributions of the quality of the study scores (S<sub>S</sub>) of the selected studies, grouped by the four most frequent algorithm names (D020)."
    )?;
    add_figure_to_node(&mut sms_pair_ss00_d020, "sms_figures/pair_ss00_d020.html");

    let mut sms_pair_ss00_d030 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_pair_ss00_d030.h4(),
        "<b>Pair 3:</b> Violin plots of the distributions of the quality of the study scores (S<sub>S</sub>) of the selected studies, grouped by whether they use a simulator or a real quantum computer (D030)."
    )?;
    add_figure_to_node(&mut sms_pair_ss00_d030, "sms_figures/pair_ss00_d030.html");

    let mut sms_pair_ss00_d040 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_pair_ss00_d040.h4(),
        "<b>Pair 4:</b> Violin plots of the distributions of the quality of the study scores (S<sub>S</sub>) of the selected studies, grouped by the type of quantum computer (D040)."
    )?;
    add_figure_to_node(&mut sms_pair_ss00_d040, "sms_figures/pair_ss00_d040.html");

    let mut sms_pair_ss00_d050 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square-double-size page-ultra-wide-treatment'");
    write!(
        sms_pair_ss00_d050.h4(),
        "<b>Pair 5:</b> Violin plots of the distributions of the quality of the study scores (S<sub>S</sub>) of the selected studies, grouped by the seven most frequent ISIC sections (D050)."
    )?;
    add_figure_to_node(&mut sms_pair_ss00_d050, "sms_figures/pair_ss00_d050.html");

    let mut sms_pair_ss00_d060 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square-double-size blank-background page-ultra-wide-treatment'");
    write!(
        sms_pair_ss00_d060.h4(),
        "<b>Pair 6:</b> Violin plots of the distributions of the quality of the study scores (S<sub>S</sub>) of the selected studies, grouped by the publication year (D060)."
    )?;
    add_figure_to_node(&mut sms_pair_ss00_d060, "sms_figures/pair_ss00_d060.html");

    let mut sms_pair_ss00_d070 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_pair_ss00_d070.h4(),
        "<b>Pair 7:</b> Violin plots of the distributions of the quality of the study scores (S<sub>S</sub>) of the selected studies, grouped by their publication source (D070)."
    )?;
    add_figure_to_node(&mut sms_pair_ss00_d070, "sms_figures/pair_ss00_d070.html");

    let mut sms_pair_sr00_d010 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_pair_sr00_d010.h4(),
        "<b>Pair 8:</b> Violin plots of the distributions of the quality of the reporting scores (S<sub>R</sub>) of the selected studies, grouped by their type of approach (D010)."
    )?;
    add_figure_to_node(&mut sms_pair_sr00_d010, "sms_figures/pair_sr00_d010.html");

    let mut sms_pair_sr00_d020 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_pair_sr00_d020.h4(),
        "<b>Pair 9:</b> Violin plots of the distributions of the quality of the reporting scores (S<sub>R</sub>) of the selected studies, grouped by the four most frequent algorithm names (D020)."
    )?;
    add_figure_to_node(&mut sms_pair_sr00_d020, "sms_figures/pair_sr00_d020.html");

    let mut sms_pair_sr00_d030 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_pair_sr00_d030.h4(),
        "<b>Pair 10:</b> Violin plots of the distributions of the quality of the reporting scores (S<sub>R</sub>) of the selected studies, grouped by whether they use a simulator or a real quantum computer (D030)."
    )?;
    add_figure_to_node(&mut sms_pair_sr00_d030, "sms_figures/pair_sr00_d030.html");

    let mut sms_pair_sr00_d040 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_pair_sr00_d040.h4(),
        "<b>Pair 11:</b> Violin plots of the distributions of the quality of the reporting scores (S<sub>R</sub>) of the selected studies, grouped by the type of quantum computer (D040)."
    )?;
    add_figure_to_node(&mut sms_pair_sr00_d040, "sms_figures/pair_sr00_d040.html");

    let mut sms_pair_sr00_d050 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square-double-size blank-background page-ultra-wide-treatment'");
    write!(
        sms_pair_sr00_d050.h4(),
        "<b>Pair 12:</b> Violin plots of the distributions of the quality of the reporting scores (S<sub>R</sub>) of the selected studies, grouped by the seven most frequent ISIC sections (D050)."
    )?;
    add_figure_to_node(&mut sms_pair_sr00_d050, "sms_figures/pair_sr00_d050.html");

    let mut sms_pair_sr00_d060 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square-double-size page-ultra-wide-treatment'");
    write!(
        sms_pair_sr00_d060.h4(),
        "<b>Pair 13:</b> Violin plots of the distributions of the quality of the reporting scores (S<sub>R</sub>) of the selected studies, grouped by the publication year (D060)."
    )?;
    add_figure_to_node(&mut sms_pair_sr00_d060, "sms_figures/pair_sr00_d060.html");

    let mut sms_pair_sr00_d070 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_pair_sr00_d070.h4(),
        "<b>Pair 14:</b> Violin plots of the distributions of the quality of the reporting scores (S<sub>R</sub>) of the selected studies, grouped by their publication source (D070)."
    )?;
    add_figure_to_node(&mut sms_pair_sr00_d070, "sms_figures/pair_sr00_d070.html");

    
    let mut sms_pair_d010_d050 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_pair_d010_d050.h4(),
        "<b>Pair 15:</b> Bar plot of the proportions of the selected studies according to their type of approach (D010), grouped by their ISIC section (D050)."
    )?;
    add_figure_to_node(&mut sms_pair_d010_d050, "sms_figures/pair_d010_d050.html");
    
    let mut sms_pair_d010_d060 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_pair_d010_d060.h4(),
        "<b>Pair 16:</b> Bar plot of the proportions of the selected studies according to their type of approach (D010), grouped by their publication year (D060)."
    )?;
    add_figure_to_node(&mut sms_pair_d010_d060, "sms_figures/pair_d010_d060.html");

    let mut sms_pair_d010_d070 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_pair_d010_d070.h4(),
        "<b>Pair 17:</b> Bar plot of the proportions of the selected studies according to their type of approach (D010), grouped by their publication source (D070)."
    )?;
    add_figure_to_node(&mut sms_pair_d010_d070, "sms_figures/pair_d010_d070.html");

    let mut sms_pair_d020_d050 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_pair_d020_d050.h4(),
        "<b>Pair 18:</b> Bar plot of the counts of the selected studies according to their algorithm name (D020), grouped by their ISIC section (D050). Only studies belonging to the four most frequent algorithm names and to the seven most frequent ISIC sections are considered."
    )?;
    add_figure_to_node(&mut sms_pair_d020_d050, "sms_figures/pair_d020_d050.html");
    
    let mut sms_pair_d020_d060 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_pair_d020_d060.h4(),
        "<b>Pair 19:</b> Bar plot of the counts of the selected studies according to their algorithm name (D020), grouped by their publication year (D060). Only studies belonging to the four most frequent algorithm names are considered."
    )?;
    add_figure_to_node(&mut sms_pair_d020_d060, "sms_figures/pair_d020_d060.html");
    
    let mut sms_pair_d020_d070 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_pair_d020_d070.h4(),
        "<b>Pair 20:</b> Bar plot of the counts of the selected studies according to their algorithm name (D020), grouped by their publication source (D070). Only studies belonging to the four most frequent algorithm names are considered."
    )?;
    add_figure_to_node(&mut sms_pair_d020_d070, "sms_figures/pair_d020_d070.html");
    
    let mut sms_pair_d050_d060 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_pair_d050_d060.h4(),
        "<b>Pair 28:</b> Bar plot of the counts of the selected studies according to their ISIC section (D050), grouped by their publication year (D060). Only studies belonging to the seven most frequent ISIC sections are considered."
    )?;
    add_figure_to_node(&mut sms_pair_d050_d060, "sms_figures/pair_d050_d060.html");

    let mut sms_pair_d050_d070 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_pair_d050_d070.h4(),
        "<b>Pair 29:</b> Bar plot of the counts of the selected studies according to their ISIC section (D050), grouped by their publication source (D070). Only studies belonging to the seven most frequent ISIC sections are considered."
    )?;
    add_figure_to_node(&mut sms_pair_d050_d070, "sms_figures/pair_d050_d070.html");

    let mut sms_pair_d060_d070 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_pair_d060_d070.h4(),
        "<b>Pair 30:</b> Bar plot of the counts of the selected studies according to their publication source (D070), grouped by their publication year (D060)."
    )?;
    add_figure_to_node(&mut sms_pair_d060_d070, "sms_figures/pair_d070_d060.html");


    let mut sms_pair_ss03_d040 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_pair_ss03_d040.h4(),
        "<b>Pair 31:</b> Bar plot of the counts of the selected studies according to their response to SS03, grouped by their type of quantum computer (D040)."
    )?;
    add_figure_to_node(&mut sms_pair_ss03_d040, "sms_figures/pair_ss03_d040.html");


    let mut sms_pair_ss03_d050 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_pair_ss03_d050.h4(),
        "<b>Pair 32:</b> Bar plot of the counts of the selected studies according to their response to SS03, grouped by their ISIC section (D050)."
    )?;
    add_figure_to_node(&mut sms_pair_ss03_d050, "sms_figures/pair_ss03_d050.html");


    let mut sms_pair_ss03_d060 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square blank-background page-ultra-wide-treatment'");
    write!(
        sms_pair_ss03_d060.h4(),
        "<b>Pair 33:</b> Bar plot of the counts of the selected studies according to their response to SS03, grouped by their publication year (D060)."
    )?;
    add_figure_to_node(&mut sms_pair_ss03_d060, "sms_figures/pair_ss03_d060.html");

    let mut sms_pair_ss03_d070 = page_sms_article_pairwise.buf.div().attr("class='blank-container-square page-ultra-wide-treatment'");
    write!(
        sms_pair_ss03_d070.h4(),
        "<b>Pair 34:</b> Bar plot of the counts of the selected studies according to their response to SS03, grouped by their publication source (D070)."
    )?;
    add_figure_to_node(&mut sms_pair_ss03_d070, "sms_figures/pair_ss03_d070.html");

    //// Publish SMS pages
    page_sms_article_quality.publish("smsquality.html");
    page_sms_article_individual.publish("sms.html");
    page_sms_article_pairwise.publish("smspairwise.html");

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
