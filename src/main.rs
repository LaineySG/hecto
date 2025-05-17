#![warn(clippy::all, clippy::pedantic)]
mod editor; // Brings editor into scope. Looks for editor.rs or editor/mod.rs
mod terminal;

use editor::Editor;

fn main() {
    Editor::default().run();
}