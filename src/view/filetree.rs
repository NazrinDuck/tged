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

macro_rules! impl_icon_color {
    (
        $(
            $extension_name: expr, $icon: expr, $color: expr,
        )*
    ) => {
        #[inline]
        fn match_icon(string: Option<&str>) -> String {
            match string {
                $(
                Some($extension_name) => $icon.fclr_head(&Color::from($color)),
                )*
                _ => "".to_string(),
            }
        }
    };
}

impl_icon_color! {
    // #dea584
    "rs"  , "", 0xdea584,
    // #51a0cf
    "lua" , "", 0x51a0cf,
    // #a074c4
    "hs"  , "", 0xa074c4,
    // #599eff
    "c"   , "", 0x599eff,
    // #f34b7d
    "cpp" , "", 0xf34b7d,
    // #ffbc03
    "py"  , "", 0xffbc03,
    // #9f0500
    "out" , "", 0x9f0500,
    // #0091bd
    "S"   , "", 0x0091bd,
    "asm" , "", 0x0091bd,
    // #e44d26
    "html", "", 0xe44d26,
    // #a074c4
    "php" , "", 0xa074c4,
    // #42a5f5
    "css" , "", 0x42a5f5,
    // #cc3e44
    "java", "", 0xcc3e44,
    // #cbcb41
    "js"  , "", 0xcbcb41,
    // #cbcb41
    "json", "", 0xcbcb41,
    // #9c4221
    "toml", "", 0x9c4221,
    // #bbbbbb
    "lock", "", 0xbbbbbb,
    // #89e051
    "txt" , "󰈙", 0x89e051,
}

#[view("FileTree")]
#[start=(1, 2)]
#[end=(26, -2)]
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
        let (bclr, fclr) = (&self.bcolor, &self.fcolor);
        let curr_dir = file_mod.curr_dir();
        self.dir = curr_dir
            .to_str()
            .unwrap()
            .to_string()
            .color(bclr, fclr)
            .bold();
        let mut dir_entrys = Vec::new();
        for entry in std::fs::read_dir(".").unwrap() {
            let dir = entry.unwrap();
            let file_type = dir.file_type().unwrap();
            let icon = if file_type.is_dir() {
                "".to_string().fclr_head(fclr)
            } else if file_type.is_file() {
                if let Some(os_str) = dir.path().extension() {
                    match_icon(os_str.to_str())
                } else {
                    "".to_string()
                }
            } else {
                "?".to_string()
            };
            let dir_entry = format!(
                "{}  {icon} {}",
                bclr.bclr_head(),
                dir.file_name().into_string().unwrap().fcolor(fclr)
            );
            dir_entrys.push(dir_entry);
        }
        self.dir_entrys = dir_entrys;
    }
    fn matchar(&mut self, _: &Term, _: &mut FileMod, settings: &Settings, _: getch_rs::Key) {}
    fn set_cursor(&self, _: &Term, settings: &Settings) {}
    fn draw(&self, term: &Term, settings: &Settings) -> std::io::Result<()> {
        self.refresh(term);
        let (x, y) = self.get_start(term);

        Cursor::set_csr(x, y);
        println!("{}", self.dir);

        for entry in &self.dir_entrys {
            println!("{}", entry);
        }
        //print!("{:^width$}", width = term.width.into());
        io::stdout().flush()?;
        Ok(())
    }
}
