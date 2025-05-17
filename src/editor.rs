use std::io::{stdout, Error};

use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor{ should_quit: false }
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl(); // We unwrap this at the end so we can terminate the session properly
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Self::clear_screen()
    }

    fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            let evt = read()?;
            self.evaluate_event(&evt);
            self.refresh_screen()?;
            if self.should_quit {
                break;
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

    fn refresh_screen(&self) -> Result<(), Error> {
            if self.should_quit {
                print!("Exiting program. Goodbye!\r\n");
                Self::clear_screen()?;
            }
            Ok(())
    }
}


// cargo fmt
// cargo clippy -- -W clippy::all  -W clippy::pedantic
//cargo clippy -- -W clippy::all -W clippy::pedantic  -W clippy::nursery -W clippy::cargo -W clippy::restriction