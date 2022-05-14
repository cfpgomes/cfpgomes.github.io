#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use std::error::Error;

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 115), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Heisenberg", focus: true)]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "Say my name")]
    #[nwg_layout_item(layout: grid, col: 0, row: 1, row_span: 2)]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    hello_button: nwg::Button,
}

impl BasicApp {
    fn say_hello(&self) {
        nwg::modal_info_message(
            &self.window,
            "Hello",
            &format!("Hello {}", self.name_edit.text()),
        );
    }
    fn say_goodbye(&self) {
        nwg::modal_info_message(
            &self.window,
            "Goodbye",
            &format!("Goodbye {}", self.name_edit.text()),
        );
        nwg::stop_thread_dispatch();
    }
}

/// Enum with all possible queries for the engine.
/// `Publication` is a query to create a new publication.
/// `Modification` is a query to modify an existing publication.
/// `Compilation` is a query to compile the website.
pub enum Query {
    Publication,
    Modification,
    Compilation,
}

pub struct Config {
    pub query: Query,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("Incorrect number of arguments!");
        }

        let query = match args[1].as_str() {
            "p" | "pub" | "publication" => Query::Publication,
            "m" | "mod" | "modification" => Query::Modification,
            "c" | "com" | "compilation" => Query::Compilation,
            _ => return Err("Unrecognized query!"),
        };

        Ok(Config { query })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.query {
        Query::Publication => publish(),
        Query::Modification => modify(),
        Query::Compilation => compile(),
    }

    Ok(())
}

pub fn publish() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}

pub fn modify() {
    unimplemented!();
}

pub fn compile() {
    unimplemented!();
}
