mod terminal; // Rust looks for terminal/mod.rs or terminal.rs in the dir named after the current module, aka editor/terminal.rs
use terminal::{Position, Terminal, TerminalSize};

use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::Char,
    KeyEvent, KeyModifiers,
};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl(); // We unwrap this at the end so we can terminate the session properly
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let evt = read()?;
            self.evaluate_event(&evt);
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Exiting.\r\n")?;
        } else {
            Self::draw_rows()?;
            Self::draw_welcome_message()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let TerminalSize { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Self::draw_empty_row()?;
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::clear_line()?;
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let (width, height) = (
            Terminal::size()?.width as usize,
            Terminal::size()?.height as usize,
        );

        let mut welcome_message = format!("{}: v{}", NAME, VERSION);
        welcome_message.truncate(width);
        let len = welcome_message.len() as usize;

        let y_cursor_position = height * 2 / 3;
        let x_cursor_position = (width / 2) - (len / 2); // half the width, then go back half the message length
        Terminal::move_cursor_to(Position {
            x: x_cursor_position as u16,
            y: y_cursor_position as u16,
        })?;

        Terminal::print(&welcome_message)?;
        Ok(())
    }

    fn evaluate_event(&mut self, evt: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = evt
        {
            match code {
                // Rust implicitly dereferences code here https://doc.rust-lang.org/reference/patterns.html#binding-modes
                Char('q') if *modifiers == KeyModifiers::ALT => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
}

// cargo fmt
// cargo clippy -- -W clippy::all  -W clippy::pedantic
//cargo clippy -- -W clippy::all -W clippy::pedantic  -W clippy::nursery -W clippy::cargo -W clippy::restriction
