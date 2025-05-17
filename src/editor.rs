mod terminal; // Rust looks for terminal/mod.rs or terminal.rs in the dir named after the current module, aka editor/terminal.rs
use terminal::Terminal;

use std::io::Error;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};

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
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Exiting.\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let height = Terminal::size()?.1;
        for current_row in 0..height {
            print!("~");
            if current_row + 1 < height {
                print!("\r\n");
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