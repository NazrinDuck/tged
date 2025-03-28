use super::{Pos, SplitNAt, View, ViewID};
use getch_rs::Key;
use std::cell::RefCell;
use std::fmt::format;
use std::io::{self, Write};
use std::rc::Rc;
use tged::view;

use crate::color::{Color, Colorful};
use crate::file::Content;
use crate::settings::Settings;
use crate::{
    terminal::{cursor::Cursor, term::Term},
    view::Position,
    FileMod,
};
#[view("MainView")]
#[start=(26, 2)]
#[end=(-1, -2)]
//#[prior = 4]
pub struct MainView {
    //line number's color
    lnum_clr: Color,
    //line number's stressed color
    lnum_sclr: Color,
    curr_line: usize,
    curr_idx: usize,
    content: Content,
    scroll: usize,
    //settings: Settings,
}

//#[bcolor=(0x20, 0x20, 0x20)]
impl View for MainView {
    fn update(&mut self, _: &Term, _: &mut FileMod) {}
    fn matchar(&mut self, term: &Term, file_mod: &mut FileMod, settings: &Settings, key: Key) {
        match key {
            Key::Char('\r') => {
                self.push_line(term, settings);
            }
            Key::Char('\t') => {
                self.push_str("    ");
            }
            Key::Char(char) => {
                self.push(char);
            }
            Key::Ctrl('s') => {
                file_mod.save().unwrap();
            }
            Key::Delete => {
                self.delete(term, settings);
            }
            Key::Up => {
                self.up(term, settings);
            }

            Key::Down => {
                self.down(term, settings);
            }

            Key::Left => {
                self.left();
            }

            Key::Right => {
                self.right();
            }

            Key::F(6) => {
                let curr_pos = (self.curr_idx, self.curr_line);
                let scroll = self.scroll;
                let new_status = file_mod.shift(curr_pos, scroll);
                self.sync(file_mod, new_status).unwrap();
            }

            other => {
                dbg!(other);
            }
        }
    }

    fn set_cursor(&self, term: &Term, settings: &Settings) {
        let (curr_line, idx) = (self.curr_line, self.curr_idx);
        let content = self.content.borrow();

        let max = self.get_vpos_max(term, settings);
        let (mut csr_x, mut csr_y): (u16, u16) = self.get_text_pos(term, settings);

        csr_x += (idx % max) as u16;

        for line in content.iter().take(curr_line).skip(self.scroll) {
            if line.is_empty() {
                csr_y += 1;
                continue;
            }

            let len = line.len();
            csr_y += (len / max) as u16 + if (len % max) == 0 { 0u16 } else { 1u16 };
        }

        if idx >= max {
            csr_y += (idx / max) as u16
        }

        Cursor::set_csr(csr_x, csr_y);
    }

    fn draw(&self, term: &Term, settings: &Settings) -> io::Result<()> {
        self.refresh(term);
        let content = self.content.borrow();

        let (x_pos, y_pos) = self.get_pos(term);

        let height = self.end.1.unwrap(term.height);
        let width = self.end.0.unwrap(term.width);

        let is_show_num = settings.is_show_num;
        let line_num_offset = settings.num_offset;

        let mut line_num = self.scroll + 1;

        let max_line = width - x_pos - line_num_offset;
        let max_height = (height - y_pos) as usize;

        let (bclr, fclr) = (&self.bcolor, &self.fcolor);
        let (lnum_clr, lnum_sclr) = (&self.lnum_clr, &self.lnum_sclr);

        Cursor::set_csr(x_pos, y_pos);

        let mut height_cnt = 1;
        'out: for line in content.iter().skip(self.scroll) {
            let mut lines: Vec<String> = Vec::new();
            for (subline, cnt) in line.splitn_at(max_line as usize) {
                if is_show_num {
                    let mut number = if cnt == 0 {
                        format!(
                            "{:>width$}│",
                            line_num,
                            width = (line_num_offset - 1) as usize
                        )
                    } else {
                        " ".repeat((line_num_offset - 1) as usize) + "│"
                    };

                    let subline = if line_num == self.curr_line + 1 {
                        number = number.fcolor(lnum_sclr).bold();
                        format!("{:<width$}", subline, width = (max_line) as usize)
                            .color(&bclr.lighten(0x6), fclr)
                    } else {
                        number = number.fcolor(lnum_clr);
                        subline.color(bclr, fclr)
                    };

                    lines.push(format!("{number}{subline}"));
                    /*
                    lines.push(format!(
                        "{:>width$}│{subline}",
                        line_num,
                        width = (line_num_offset - 1) as usize
                    ));
                    */
                } else {
                    lines.push(subline);
                }
            }

            for subline in lines {
                Cursor::csr_setcol(x_pos);
                let subline = if line_num == self.curr_line + 1 {
                    format!(
                        "{:<width$}",
                        subline,
                        width = (max_line + line_num_offset) as usize
                    )
                    .color(&bclr.lighten(0x6), fclr)
                } else {
                    subline.color(bclr, fclr)
                };
                print!("{}", subline);
                height_cnt += 1;
                if height_cnt > max_height {
                    break 'out;
                }
                Cursor::csr_nextline();
            }
            line_num += 1;
        }
        io::stdout().flush()?;
        Ok(())
    }

    fn init(&mut self, _: &Term, file_mod: &mut FileMod, settings: &Settings) {
        /*
        let content = String::from_utf8_lossy(file_mod.get_content());
        for line in content.lines() {
            self.content.push(line.to_string());
        }
        */
        self.content = Rc::clone(file_mod.get_content());

        let (bclr, fclr) = (&settings.theme.normal_bclr, &settings.theme.normal_fclr);
        let (lnum_clr, lnum_sclr) = (&settings.theme.weak_fclr, &settings.theme.yellow);
        self.bcolor = bclr.clone();
        self.fcolor = fclr.clone();
        self.lnum_clr = lnum_clr.clone();
        self.lnum_sclr = lnum_sclr.clone();
    }
}

impl MainView {
    pub fn sync(
        &mut self,
        file_mod: &mut FileMod,
        new_status: (usize, usize, usize),
    ) -> io::Result<()> {
        self.content = Rc::clone(file_mod.get_content());
        self.curr_idx = new_status.0;
        self.curr_line = new_status.1;
        self.scroll = new_status.2;
        Ok(())
    }

    #[inline]
    pub fn get_pos(&self, term: &Term) -> (u16, u16) {
        let (height, width) = (term.height, term.width);
        usize::default();
        (self.start.0.unwrap(width), self.start.1.unwrap(height))
    }

    #[inline]
    pub fn get_text_pos(&self, term: &Term, settings: &Settings) -> (u16, u16) {
        let (height, width) = (term.height, term.width);
        (
            self.start.0.unwrap(width) + settings.num_offset,
            self.start.1.unwrap(height),
        )
    }

    #[inline]
    fn get_vpos_max(&self, term: &Term, settings: &Settings) -> usize {
        (self.end.0.unwrap(term.width) - self.get_text_pos(term, settings).0) as usize
    }

    #[inline]
    fn pre_all_lines(&self, term: &Term, settings: &Settings) -> usize {
        let (curr_line, idx) = (self.curr_line, self.curr_idx);
        let content = self.content.borrow();

        let mut line_cnt = 0;
        let max = self.get_vpos_max(term, settings);
        for line in content.iter().take(curr_line).skip(self.scroll) {
            if line.is_empty() {
                line_cnt += 1;
                continue;
            }

            let len = line.len();
            line_cnt += (len / max) + if (len % max) == 0 { 0 } else { 1 };
        }

        if idx >= max {
            line_cnt += idx / max
        }

        line_cnt
    }

    #[inline]
    fn line_inc(&mut self, term: &Term, settings: &Settings) {
        let pre_all_lines = self.pre_all_lines(term, settings);

        let height = (self.end.1.unwrap(term.height) - self.start.1.unwrap(term.height)) as usize;

        if pre_all_lines + 4 > height {
            self.scroll += 1;
        }
        self.curr_line += 1;
    }

    #[inline]
    fn line_dec(&mut self, term: &Term, settings: &Settings) {
        if self.scroll != 0 {
            let pre_all_lines = self.pre_all_lines(term, settings);

            if pre_all_lines < 4 {
                self.scroll -= 1;
            }
        }
        self.curr_line -= 1;
    }

    #[inline]
    pub fn push(&mut self, ch: char) {
        let mut content = self.content.borrow_mut();
        if content.is_empty() {
            content.push(String::new());
        }
        content[self.curr_line].insert(self.curr_idx, ch);
        self.curr_idx += 1;
    }

    #[inline]
    pub fn push_str(&mut self, string: &str) {
        let mut content = self.content.borrow_mut();
        content[self.curr_line].insert_str(self.curr_idx, string);
        self.curr_idx += string.len();
    }

    #[inline]
    pub fn push_line(&mut self, term: &Term, settings: &Settings) {
        let (line, idx) = (self.curr_line, self.curr_idx);
        let mut content = self.content.borrow_mut();

        let curr_line = content[line].clone();
        let (first, last) = curr_line.split_at(idx);
        content[line] = String::from(first);
        content.insert(line + 1, String::from(last));
        drop(content);

        //self.curr_line += 1;
        self.line_inc(term, settings);
        self.curr_idx = 0;
    }

    #[inline]
    pub fn delete(&mut self, term: &Term, settings: &Settings) {
        let (line, idx) = (self.curr_line, self.curr_idx);

        if idx == 0 {
            if line == 0 {
                return;
            }

            let content = self.content.borrow().clone();
            let mut content_mut = self.content.borrow_mut();

            self.curr_idx = content[line - 1].len();
            content_mut[line - 1].push_str(&content[line]);

            content_mut.remove(line);
            drop(content_mut);

            self.line_dec(term, settings);
        } else {
            self.content.borrow_mut()[line].remove(idx - 1);
            self.curr_idx -= 1;
        }
    }

    #[inline]
    pub fn up(&mut self, term: &Term, settings: &Settings) {
        let (line, idx) = (self.curr_line, self.curr_idx);

        if line > 0 {
            let content = self.content.borrow();
            if idx > content[line - 1].len() {
                self.curr_idx = content[line - 1].len()
            };
            drop(content);
            self.line_dec(term, settings);
        }
    }

    #[inline]
    pub fn down(&mut self, term: &Term, settings: &Settings) {
        let (line, idx) = (self.curr_line, self.curr_idx);

        let content = self.content.borrow();
        if line < content.len() - 1 {
            if idx > content[line + 1].len() {
                self.curr_idx = content[line + 1].len()
            };
            drop(content);
            self.line_inc(term, settings);
        }
    }

    #[inline]
    pub fn left(&mut self) {
        if self.curr_idx > 0 {
            let mut idx = self.curr_idx - 1;
            /*
            while idx > 0 && !self.content.borrow()[self.curr_line].is_char_boundary(idx) {
                idx -= 1;
            }
            */
            self.curr_idx = idx;
        }
    }

    #[inline]
    pub fn right(&mut self) {
        let max = self.content.borrow()[self.curr_line].len();
        if self.curr_idx < max {
            let mut idx = self.curr_idx + 1;
            /*
            while idx < max && !self.content.borrow()[self.curr_line].is_char_boundary(idx) {
                idx += 1;
            }
            */
            self.curr_idx = idx;
        }
    }
}
