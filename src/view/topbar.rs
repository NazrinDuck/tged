use super::{settings::Settings, Pos, View};
use crate::terminal::{cursor::Cursor, term::Term};
use std::io::{self, Write};
use tged::view;

#[view]
#[start=(1, 1)]
#[end=(-1, 1)]
pub struct TopBar {
    content: String,
    prior: u8,
    settings: Settings,
}

impl View for TopBar {
    fn matchar(&mut self, _: &Term, _: getch_rs::Key) {}
    fn set_cursor(&self, _: &Term) {}
    fn draw(&self, term: &Term) -> std::io::Result<()> {
        let (x, y) = self.get_start(term);
        Cursor::set_csr(x, y);
        print!("{:^width$}", self.content, width = term.width.into());
        io::stdout().flush()?;
        Ok(())
    }
}

impl TopBar {
    pub fn push_str(&mut self, string: &str) {
        self.content.push_str(string);
    }
}
