use super::{Pos, View, ViewID};
use crate::{
    color::{Color, Colorful},
    settings::Settings,
    terminal::{cursor::Cursor, term::Term},
    view::Position,
    FileMod,
};
use std::io::{self, Write};
use tged::view;

#[view]
#[start=(1, -2)]
#[end=(-1, -1)]
#[bcolor=(0x10, 0x10, 0x10)]
#[fcolor=(0x10, 0x10, 0x10)]
pub struct BottomBar {
    content: String,
    prior: u8,
}

impl View for BottomBar {
    fn init(&mut self, term: &Term, file_mod: &mut FileMod, settings: &Settings) {}
    fn update(&mut self, _: &Term, _: &mut FileMod) {}
    fn matchar(&mut self, _: &Term, _: &mut FileMod, settings: &Settings, _: getch_rs::Key) {}
    fn set_cursor(&self, _: &Term, settings: &Settings) {}
    fn draw(&self, term: &Term, settings: &Settings) -> std::io::Result<()> {
        let (x, y) = self.get_start(term);
        Cursor::set_csr(x, y);
        print!("{:^width$}", self.content, width = term.width.into());
        io::stdout().flush()?;
        Ok(())
    }
}

impl BottomBar {
    pub fn push_str(&mut self, string: &str) {
        self.content.push_str(string);
    }
}
