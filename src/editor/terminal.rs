use std::io::{stdout, Error};
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};

pub struct Terminal {
    should_quit: bool
}

impl Terminal {

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0,0)?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Self::clear_screen()
    }

    pub fn clear_screen() -> Result<(), Error> {
        execute!(stdout(), Clear(ClearType::All))
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), Error>{
        execute!(stdout(), MoveTo(x,y))?;
        Ok(())
    }

    pub fn size() -> Result<(u16,u16), Error> {
        size()
    }
    
}