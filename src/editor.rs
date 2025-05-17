mod terminal; // Rust looks for terminal/mod.rs or terminal.rs in the dir named after the current module, aka editor/terminal.rs
use terminal::{Position, Terminal, TerminalSize};

use crossterm::event::{
    read, Event::{self, Key}, KeyCode, KeyEvent, KeyEventKind, KeyModifiers
};
use std::{cmp::min, io::Error};

#[derive (Copy, Clone, Default)]
struct CaretLocation {
    x: usize,
    y: usize,
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive (Copy, Clone, Default)]
pub struct Editor {
    should_quit: bool,
    caret_location: CaretLocation,
}

impl Editor {

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl(); // We unwrap this at the end so we can terminate the session properly
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            if self.should_quit {
                break;
            }
            let evt = read()?;
            self.evaluate_event(&evt)?;
            self.refresh_screen()?;
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Exiting.\r\n")?;
        } else {
            Self::draw_rows()?;
            Self::draw_welcome_message()?;
            Terminal::move_caret_to(Position {
                col: self.caret_location.x as isize,
                row: self.caret_location.y as isize,
            })?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let TerminalSize { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Self::draw_empty_row()?;
            if current_row.saturating_add(1) < height {
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

        let y_caret_position = height * 2 / 3;
        let x_caret_position = (width / 2).saturating_sub(len / 2); // half the width, then go back half the message length
        Terminal::move_caret_to(Position {
            col: x_caret_position as isize,
            row: y_caret_position as isize,
        })?;

        Terminal::print(&welcome_message)?;
        Terminal::move_caret_to(Position::default())?;
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let CaretLocation { mut x, mut y } = self.caret_location;
        let TerminalSize { height, width } = Terminal::size()?;
        let (max_y, max_x) = (height as isize, width as isize);
        match key_code {
            KeyCode::Up => {
                //y = y.saturating_sub(1);
                let new_y = y as isize - 1;
                y = if new_y < 0 {(max_y - 1) as usize} else {new_y as usize};
            }
            KeyCode::Down => {
                //y = min((height as usize).saturating_sub(1), y.saturating_add(1));
                let new_y = y as isize + 1;
                y = if new_y >= max_y {0} else {new_y as usize};
            }
            KeyCode::Left => {
                //x = x.saturating_sub(1);
                let new_x = x as isize - 1;
                x = if new_x < 0 {(max_x - 1) as usize} else {new_x as usize};
            }
            KeyCode::Right => {
                //x = min((width as usize).saturating_sub(1), x.saturating_add(1));
                let new_x = x as isize + 1;
                x = if new_x >= max_x {0} else {new_x as usize};
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = (height as usize).saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = (width as usize).saturating_sub(1);
            }
            _ => (),
        }
        self.caret_location = CaretLocation { x, y };
        Ok(())
    }

    fn evaluate_event(&mut self, evt: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code, modifiers, kind: KeyEventKind::Press, ..
        }) = evt
        {
            match code {
                // Rust implicitly dereferences code here https://doc.rust-lang.org/reference/patterns.html#binding-modes
                KeyCode::Char('q') if *modifiers == KeyModifiers::ALT => {
                    self.should_quit = true;
                },
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }
}

// cargo fmt
// cargo clippy -- -W clippy::all  -W clippy::pedantic
//cargo clippy -- -W clippy::all -W clippy::pedantic  -W clippy::nursery -W clippy::cargo -W clippy::restriction
