use std::io::{stdout, Error};
use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};

pub struct TerminalSize {
    pub width: u16,
    pub height: u16,
}
pub struct Coords {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal;

impl Terminal {

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Coords {x:0,y:0})?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Self::clear_screen()
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All)) // FromCursorDown works, kind of
    }

    pub fn move_cursor_to(c: Coords) -> Result<(), Error>{
        queue!(stdout(), MoveTo(c.x,c.y))?;
        Ok(())
    }

    pub fn size() -> Result<TerminalSize, Error> {
        let (width, height) = size()?;
        let tsize = TerminalSize { width, height };
        Ok(tsize)
    }
    
}