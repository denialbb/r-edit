#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
mod editor;
use editor::Editor;
use editor::logger::{Config, CustomLogger};
use log::debug;
use log::info;
use log::{self, LevelFilter};

fn main() {
    let logger = CustomLogger::new(
        Config {
            level_filter: LevelFilter::Debug,
            truncate: true,
        },
        "r-edit.log",
    )
    .unwrap();
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(LevelFilter::Debug);

    // if we don't get any argument we default to the test file
    let mut filename: String = String::from("test/test.txt");
    // collect depletes the iterator
    let args: Vec<String> = std::env::args().collect();
    if let Some(first_arg) = args.get(1) {
        info!("Argument provided: {first_arg} (filename)");
        filename = first_arg.clone();
    } else {
        info!("No argument provided");
    }

    let mut editor = Editor::default(filename);
    editor.run();
}
