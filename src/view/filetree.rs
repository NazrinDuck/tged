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
#[start=(1, 2)]
#[end=(26, -2)]
#[bcolor=(0x28, 0x28, 0x28)]
#[fcolor=(0xd0, 0xc0, 0x80)]
pub struct FileTree {
    dir: String,
    dir_entrys: Vec<String>,
}

impl View for FileTree {
    fn init(&mut self, _: &Term, _: &mut FileMod, settings: &Settings) {
        let (bclr, fclr) = (
            &settings.theme.normal_bclr.darken(0x4),
            &settings.theme.stress_fclr,
        );
        self.bcolor = bclr.clone();
        self.fcolor = fclr.clone();
    }

    fn update(&mut self, _: &Term, file_mod: &mut FileMod) {
        let curr_dir = file_mod.curr_dir();
        self.dir = curr_dir.to_str().unwrap().to_string();
        let mut dir_entrys = Vec::new();
        for entry in std::fs::read_dir(".").unwrap() {
            let dir = entry.unwrap();
            let file_type = dir.file_type().unwrap();
            let icon = if file_type.is_dir() {
                "  "
            } else if file_type.is_file() {
                "  "
            } else {
                "  ?"
            };
            let dir_entry = format!("{icon} {}", dir.file_name().into_string().unwrap());
            dir_entrys.push(dir_entry);
        }
        self.dir_entrys = dir_entrys;
    }
    fn matchar(&mut self, _: &Term, _: &mut FileMod, settings: &Settings, _: getch_rs::Key) {}
    fn set_cursor(&self, _: &Term, settings: &Settings) {}
    fn draw(&self, term: &Term, settings: &Settings) -> std::io::Result<()> {
        self.refresh(term);
        let (bclr, fclr) = (&self.bcolor, &self.fcolor);
        let (x, y) = self.get_start(term);

        Cursor::set_csr(x, y);
        println!("{}", self.dir.color(bclr, fclr).bold());

        for entry in &self.dir_entrys {
            println!("{}", entry.color(bclr, fclr));
        }
        //print!("{:^width$}", width = term.width.into());
        io::stdout().flush()?;
        Ok(())
    }
}
