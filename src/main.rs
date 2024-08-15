#![warn(clippy::all, clippy::pedantic)]
// use std::io::{self, Read};
// use std::fs;
mod editor;
mod terminal;
use editor::Editor;
pub use terminal::Terminal;
fn main() {
    // let editor= Editor::default();
    Editor::default().run();
}
