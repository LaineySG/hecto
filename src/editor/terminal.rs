use std::io::{stdout, Error, Write};
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::{queue, Command};
use core::fmt::Display;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};

#[derive(Copy, Clone)]
pub struct TerminalSize {
    pub width: u16,
    pub height: u16,
}
#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal;

impl Terminal {

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position {x:0,y:0})?;
        Self::execute()
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Self::clear_screen()
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))
    }
    
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn move_cursor_to(pos: Position) -> Result<(), Error>{
        Self::queue_command(MoveTo(pos.x,pos.y))
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    pub fn show_cursor() -> Result<(), Error>{
        Self::queue_command(Show)
    }

    pub fn print<T: Display>(string: T) -> Result<(), Error>{ // identical to string: impl Display
        Self::queue_command(Print(string))
    }

    pub fn size() -> Result<TerminalSize, Error> {
        let (width, height) = size()?;
        Ok(TerminalSize { width, height })
    }

    /// Ensure all pending writes are gone.
    pub fn execute() -> Result<(),Error> {
        stdout().flush()
    }

    pub fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)
    }
    
}