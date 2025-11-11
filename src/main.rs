#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
mod editor;
use editor::Editor;
use editor::logger::{Config, CustomLogger};
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

    let mut editor = Editor::default();
    editor.run();
}
