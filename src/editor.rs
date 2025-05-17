
use crate::terminal;

pub struct Editor;

impl Editor {
    pub fn default() -> Self {
        Editor{}
    }

    pub fn run(&mut self) {
        terminal::Terminal::default().run()
    }
}


// cargo fmt
// cargo clippy -- -W clippy::all  -W clippy::pedantic
//cargo clippy -- -W clippy::all -W clippy::pedantic  -W clippy::nursery -W clippy::cargo -W clippy::restriction