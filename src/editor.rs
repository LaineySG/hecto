mod terminal; // Rust looks for terminal/mod.rs or terminal.rs in the dir named after the current module, aka editor/terminal.rs
use terminal::Terminal;

use std::io::{stdout, Error, Write};
use crossterm::{cursor::{Hide, Show}, event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers}, style::Print, queue};

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
        queue!(stdout(), Hide)?;
        if self.should_quit {
            Terminal::clear_screen()?;
            queue!(stdout(), Print("Exiting.\r\n"))?;
            stdout().flush()?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(
                terminal::Coords {x:0,y:0}
            )?;
        }
        queue!(stdout(), Show)?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let height = Terminal::size()?.height;
        for current_row in 0..height {
            queue!(stdout(), Print("~"))?;
            stdout().flush()?;
            if current_row + 1 < height {
            queue!(stdout(), Print("\r\n"))?;
            stdout().flush()?;
            }
        }
        Ok(())
    }

    fn evaluate_event(&mut self, evt: &Event) {
        if let Key(KeyEvent { code, modifiers, ..}) = evt
        {
            match code { // Rust implicitly dereferences code here https://doc.rust-lang.org/reference/patterns.html#binding-modes
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