use super::{settings::Settings, Pos, View, ViewID};
use crate::{
    color::{Color, Colorful},
    terminal::{cursor::Cursor, term::Term},
    view::Position,
    FileMod,
};
use std::io::{self, Write};
use tged::view;

#[view]
#[start=(1, 1)]
#[end=(-1, 2)]
#[bcolor=(0x1c, 0x1c, 0x1c)]
#[fcolor=(0xa0, 0x40, 0x40)]
pub struct TopBar {
    content: String,
    prior: u8,
    settings: Settings,
}

impl View for TopBar {
    fn update(&mut self, _: &Term, file_mod: &mut FileMod) {
        let content: String = file_mod
            .names()
            .iter()
            .fold(String::new(), |init: String, name| {
                if init.is_empty() {
                    format!("{}", name)
                } else {
                    format!("{init} | {}", name)
                }
            });
        self.content = content;
    }
    fn matchar(&mut self, _: &Term, _: &mut FileMod, _: getch_rs::Key) {}
    fn set_cursor(&self, _: &Term) {}
    fn draw(&self, term: &Term) -> std::io::Result<()> {
        self.refresh(term);

        let (bclr, fclr) = (&self.bcolor, &self.fcolor);
        let width = self.end.0.unwrap(term.width) - self.start.0.unwrap(term.width);
        let (x, y) = self.get_start(term);
        Cursor::set_csr(x, y);
        //let output = format!("{:^width$}", self.content, width = width.into());
        let output = format!("{}", self.content);

        print!("{}", output.color(bclr, fclr));
        io::stdout().flush()?;
        Ok(())
    }
}

impl TopBar {
    pub fn push_str(&mut self, string: &str) {
        self.content.push_str(string);
    }
}
