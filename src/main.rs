#![warn(clippy::all, clippy::pedantic)]
// use std::io::{self, Read};
// use std::fs;
mod document;
mod row;
mod editor;
mod terminal;
use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;
pub use document::Document;
pub use row::Row;
fn main() {
    // let editor= Editor::default();
    Editor::default().run();
}
