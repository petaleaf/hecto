#![warn(clippy::all, clippy::pedantic)]
// use std::io::{self, Read};
// use std::fs;
mod editor;
use editor::Editor;

fn main() {
    // let editor= Editor::default();
    Editor::default().run();
}
