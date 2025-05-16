use std::io::Error;

use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self { // The () indicates no parameters and that it doesn't require self, meaning it doesn't need to be called on a specific instance of the struct. It's an associated function.
        Editor{ should_quit: false }
    }

    pub fn run(&mut self) { // pass a mutable reference to an Editor.
        if let Err(e) = self.repl() {
            panic!("{e:#?}");
        }
        println!("Exiting.");
    }

    fn repl(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        loop {
            if let Key(KeyEvent { code, modifiers, kind, state}) = read()?
            {
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");
                match code {
                    Char('q') if modifiers == KeyModifiers::ALT => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}


// cargo fmt
// cargo clippy -- -W clippy::all  -W clippy::pedantic

//cargo clippy -- -W clippy::all -W clippy::pedantic  -W clippy::nursery -W clippy::cargo -W clippy::restriction