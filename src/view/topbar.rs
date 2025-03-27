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
#[start=(1, 1)]
#[end=(-1, 2)]
#[bcolor=(0x14, 0x14, 0x14)]
#[fcolor=(0xa0, 0x40, 0x40)]
pub struct TopBar {
    // stress color (used for chosen tab)
    scolor: Color,
    // dark color  (used for background)
    dcolor: Color,
    content: String,
    prior: u8,
}

impl View for TopBar {
    fn init(&mut self, _: &Term, _: &mut FileMod, settings: &Settings) {
        let (bclr, fclr) = (&settings.theme.black, &settings.theme.weak_fclr);
        let (sclr, dclr) = (&settings.theme.yellow, &settings.theme.normal_bclr);
        self.bcolor = bclr.clone();
        self.fcolor = fclr.clone();
        self.scolor = sclr.clone();
        self.dcolor = dclr.clone();
    }

    fn update(&mut self, _: &Term, file_mod: &mut FileMod) {
        let (bclr, fclr) = (&self.bcolor, &self.fcolor);
        let dclr = &self.dcolor;
        let sclr = &self.scolor;
        let curr_id = file_mod.curr_id();
        let mut content = file_mod.to_vec();
        content.sort_unstable_by_key(|x| x.0);
        let content: String =
            content
                .into_iter()
                .fold(String::new(), |init: String, (id, file_buf)| {
                    let name = file_buf.name();
                    let banner = if *id == curr_id {
                        format!(
                            "{}{}{}",
                            "".color(bclr, dclr),
                            format!(" {id}.{:^12} ", name).color(dclr, sclr),
                            "".color(bclr, dclr)
                        )
                    } else {
                        let clr = &dclr.darken(0x4);
                        format!(
                            "{}{}{}",
                            "".color(bclr, clr),
                            format!(" {id}.{:^12} ", name).color(clr, fclr),
                            "".color(bclr, clr)
                        )
                    };
                    if init.is_empty() {
                        format!("{}  {banner}", bclr.bclr_head())
                    } else {
                        format!("{init}{}  {banner}", bclr.bclr_head())
                    }
                });
        self.content = content;
    }
    fn matchar(&mut self, _: &Term, _: &mut FileMod, _: &Settings, _: getch_rs::Key) {}
    fn set_cursor(&self, _: &Term, _: &Settings) {}
    fn draw(&self, term: &Term, _: &Settings) -> std::io::Result<()> {
        self.refresh(term);

        /*
        let (bclr, fclr) = (&self.bcolor, &self.fcolor);
        let width = self.end.0.unwrap(term.width) - self.start.0.unwrap(term.width);
        */
        let (x, y) = self.get_start(term);
        Cursor::set_csr(x, y);
        //let output = format!("{:^width$}", self.content, width = width.into());
        //let output = format!("{}", self.content);

        print!("{}", self.content);
        io::stdout().flush()?;
        Ok(())
    }
}

impl TopBar {
    pub fn push_str(&mut self, string: &str) {
        self.content.push_str(string);
    }
}
