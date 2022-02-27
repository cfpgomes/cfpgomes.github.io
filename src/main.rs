use html_builder::*; // Contents added to buf by each statement
use std::fmt::Write;
use std::fs::File;
use std::io::Write as WriteIo;

fn add_carousel(node: &mut Node, default_image_id: usize, fullscreen: bool) {
    let mut carousel = node.div().attr(&format!(
        "class='carousel' {}",
        if fullscreen {
            "style='position:absolute; top:0px; left:0px; height:100%; z-index:-10'"
        } else {
            ""
        }
    ));

    for i in 1..=8 {
        carousel
            .input()
            .attr(&format!(
                "class='carousel-locator' id='slide-{}' type='radio' name='carousel-radio' hidden='' {}",
                i,
                if i==default_image_id {"checked=''"} else {""}
            ));
    }

    let mut carousel_container = carousel.div().attr("class='carousel-container'");

    for i in 1..=8 {
        carousel_container
            .figure()
            .attr("class='carousel-item'")
            .img()
            .attr(&format!(
                "class='img-responsive rounded' src='img/carousel{}.jpg'",
                i
            ));
    }
}

fn create_front_page(buf: &mut Buffer) {
    let mut html = buf.html().attr("lang='en'");

    // Modify head
    let mut head = html.head();
    head.meta().attr("charset='utf-8'");
    writeln!(head.title(), "Title!").unwrap();

    head.link()
        .attr("rel='stylesheet' href='css/spectre.min.css'");
    head.link()
        .attr("rel='stylesheet' href='css/spectre-exp.min.css'");
    head.link()
        .attr("rel='stylesheet' href='css/spectre-icons.min.css'");

    // Modify body
    let mut body = html.body();

    // Add carousel as fullscreen background
    add_carousel(&mut body, 1, true);

    // Add sidebar
    let mut side_bar_div = body
        .div()
        .attr("class='off-canvas off-canvas-sidebar-show'");

    // off-screen toggle button
    let mut a = side_bar_div
        .a()
        .attr("class='off-canvas-toggle btn btn-primary btn-action' href='#sidebar-id'");
    a.i().attr("class='icon icon-menu'");

    // off-screen sidebar
    let mut off_screen_sidebar = side_bar_div
        .div()
        .attr("id='sidebar-id' class='off-canvas-sidebar'");

    writeln!(off_screen_sidebar.h1(), "Sidebar!").unwrap();

    side_bar_div
        .a()
        .attr("class='off-canvas-overlay' href='#close'");

    // off-screen content
    let mut off_screen_content = side_bar_div.div().attr("class='off-canvas-content'");
    
    writeln!(off_screen_content.span().attr("class='label label-warning'"), "Warning!").unwrap();
}

fn step_print_and_execute(buf: &mut Buffer, item_name: &str, func: fn(&mut Buffer)) {
    println!("Creating {}...", item_name);
    func(buf);
    println!("{} created!", item_name);
}

fn main() {
    println!("Tool to create personal website started!");
    let mut buf = Buffer::new();

    step_print_and_execute(&mut buf, "front page", create_front_page);

    let mut file = File::create("index.html").unwrap();
    file.write_all(buf.finish().as_bytes()).unwrap();
}
