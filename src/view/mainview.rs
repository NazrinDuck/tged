use super::{settings::Settings, Pos, SplitNAt, View, ViewID};
use getch_rs::Key;
use std::io::{self, Write};
use tged::view;

use crate::color::{Color, Colorful};
use crate::rprintln;
use crate::{
    terminal::{cursor::Cursor, term::Term},
    view::Position,
    FileMod,
};

#[view]
#[start=(5, 2)]
#[end=(-1, -2)]
#[bcolor=(0x20, 0x20, 0x20)]
#[fcolor=(0xa0, 0xa0, 0xa0)]
pub struct MainView {
    curr_line: usize,
    curr_idx: usize,
    content: Vec<String>,
    scroll: usize,
    prior: u8,
    settings: Settings,
}

impl View for MainView {
    fn update(&mut self, _: &Term, _: &mut FileMod) {}
    fn matchar(&mut self, term: &Term, file_mod: &mut FileMod, key: Key) {
        match key {
            Key::Char('\r') => {
                self.push_line(term);
            }
            Key::Char('\t') => {
                self.push_str("    ");
            }
            Key::Char(char) => {
                self.push(char);
            }
            Key::Ctrl('s') => {
                file_mod.save(self.flatten()).unwrap();
            }
            Key::Delete => {
                self.delete(term);
            }
            Key::Up => {
                self.up(term);
            }

            Key::Down => {
                self.down(term);
            }

            Key::Left => {
                self.left();
            }

            Key::Right => {
                self.right();
            }

            Key::F(6) => {
                file_mod.shift();
                self.sync(file_mod).unwrap();
            }

            other => {
                dbg!(other);
            }
        }
    }

    fn set_cursor(&self, term: &Term) {
        let (curr_line, idx) = (self.curr_line, self.curr_idx);

        let max = self.get_vpos_max(term);
        let (mut csr_x, mut csr_y): (u16, u16) = self.get_text_pos(term);

        csr_x += (idx % max) as u16;

        for line in self.content.iter().take(curr_line).skip(self.scroll) {
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

    fn draw(&self, term: &Term) -> io::Result<()> {
        self.refresh(term);

        let (x_pos, y_pos) = self.get_pos(term);

        let height = self.end.1.unwrap(term.height);
        let width = self.end.0.unwrap(term.width);

        let is_show_num = self.settings.is_show_num;
        let line_num_offset = self.settings.num_offset;

        let mut line_num = self.scroll + 1;

        let max_line = width - x_pos - line_num_offset;
        let max_height = (height - y_pos) as usize;

        let (bclr, fclr) = (&self.bcolor, &self.fcolor);

        Cursor::set_csr(x_pos, y_pos);

        let mut height_cnt = 1;
        'out: for line in self.content.iter().skip(self.scroll) {
            let mut lines: Vec<String> = Vec::new();
            for (subline, cnt) in line.splitn_at(max_line as usize) {
                if is_show_num {
                    if cnt == 0 {
                        lines.push(format!(
                            "{:>width$}│{subline}",
                            line_num,
                            width = (line_num_offset - 1) as usize
                        ));
                    } else {
                        lines.push(format!(
                            "{width}│{subline}",
                            width = " ".repeat((line_num_offset - 1) as usize)
                        ));
                    }
                } else {
                    lines.push(subline);
                }
            }

            for subline in lines {
                Cursor::csr_setcol(x_pos);
                print!("{}", subline.color(bclr, fclr));
                height_cnt += 1;
                if height_cnt > max_height {
                    break 'out;
                }
                Cursor::csr_nextline();
            }
            line_num += 1;

            /*
            if height_cnt > max_height {
                break;
            }

            if is_show_num {
                Cursor::csr_setcol(x_pos);
                print!(
                    "{:>width$}│",
                    line_num,
                    width = (line_num_offset - 1) as usize
                );
            }

            if line.len() as u16 > max_line {
                for (subline, cnt) in line.splitn_at(max_line as usize) {
                    if height_cnt > max_height {
                        break;
                    }

                    if is_show_num && cnt >= 1 {
                        Cursor::csr_setcol(x_pos);
                        print!("{:>width$}", "│", width = line_num_offset as usize);
                    }

                    Cursor::csr_setcol(x_pos + line_num_offset);
                    println!("{}", subline);
                    height_cnt += 1;
                }
            } else {
                Cursor::csr_setcol(x_pos + line_num_offset);
                println!("{}", line);
                height_cnt += 1;
            }
            line_num += 1;
            */
        }
        io::stdout().flush()?;
        Ok(())
    }
}

impl MainView {
    pub fn init(&mut self, content: &str) {
        for line in content.lines() {
            self.content.push(line.to_string());
        }
    }

    pub fn sync(&mut self, file_mod: &mut FileMod) -> io::Result<()> {
        let content = file_mod.get_content();
        self.content = Vec::new();
        for line in content.lines() {
            self.content.push(line.to_string());
        }
        Ok(())
    }

    #[inline]
    pub fn settings(&mut self) -> &mut Settings {
        &mut self.settings
    }

    #[inline]
    pub fn get_pos(&self, term: &Term) -> (u16, u16) {
        let (height, width) = (term.height, term.width);
        usize::default();
        (self.start.0.unwrap(width), self.start.1.unwrap(height))
    }

    #[inline]
    pub fn get_text_pos(&self, term: &Term) -> (u16, u16) {
        let (height, width) = (term.height, term.width);
        (
            self.start.0.unwrap(width) + self.settings.num_offset,
            self.start.1.unwrap(height),
        )
    }

    #[inline]
    fn get_vpos_max(&self, term: &Term) -> usize {
        (self.end.0.unwrap(term.width) - self.get_text_pos(term).0) as usize
    }

    #[inline]
    fn pre_all_lines(&self, term: &Term) -> usize {
        let (curr_line, idx) = (self.curr_line, self.curr_idx);

        let mut line_cnt = 0;
        let max = self.get_vpos_max(term);
        for line in self.content.iter().take(curr_line).skip(self.scroll) {
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
    fn line_inc(&mut self, term: &Term) {
        let pre_all_lines = self.pre_all_lines(term);

        let height = (self.end.1.unwrap(term.height) - self.start.1.unwrap(term.height)) as usize;

        if pre_all_lines + 4 > height {
            self.scroll += 1;
        }
        self.curr_line += 1;
    }

    #[inline]
    fn line_dec(&mut self, term: &Term) {
        if self.scroll != 0 {
            let pre_all_lines = self.pre_all_lines(term);

            if pre_all_lines < 4 {
                self.scroll -= 1;
            }
        }
        self.curr_line -= 1;
    }

    pub fn push(&mut self, ch: char) {
        if self.content.is_empty() {
            self.content.push(String::new());
        }
        self.content[self.curr_line].insert(self.curr_idx, ch);
        self.curr_idx += 1;
    }

    pub fn push_str(&mut self, string: &str) {
        self.content[self.curr_line].insert_str(self.curr_idx, string);
        self.curr_idx += string.len();
    }

    pub fn push_line(&mut self, term: &Term) {
        let (line, idx) = (self.curr_line, self.curr_idx);

        let content = self.content[line].clone();
        let (first, last) = content.split_at(idx);
        self.content[line] = String::from(first);
        self.content.insert(line + 1, String::from(last));

        //self.curr_line += 1;
        self.line_inc(term);
        self.curr_idx = 0;
    }

    pub fn delete(&mut self, term: &Term) {
        let (line, idx) = (self.curr_line, self.curr_idx);

        if idx == 0 {
            if line == 0 {
                return;
            }

            let content = self.content.clone();

            self.curr_idx = content[line - 1].len();
            self.content[line - 1].push_str(&content[line]);

            self.content.remove(line);
            self.line_dec(term);
        } else {
            self.content[line].remove(idx - 1);
            self.curr_idx -= 1;
        }
    }

    pub fn up(&mut self, term: &Term) {
        let (line, idx) = (self.curr_line, self.curr_idx);

        if line > 0 {
            if idx > self.content[line - 1].len() {
                self.curr_idx = self.content[line - 1].len()
            };
            self.line_dec(term);
        }
    }

    pub fn down(&mut self, term: &Term) {
        let (line, idx) = (self.curr_line, self.curr_idx);

        if line < self.content.len() - 1 {
            if idx > self.content[line + 1].len() {
                self.curr_idx = self.content[line + 1].len()
            };
            self.line_inc(term);
        }
    }

    pub fn left(&mut self) {
        if self.curr_idx > 0 {
            self.curr_idx -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.curr_idx < self.content[self.curr_line].len() {
            self.curr_idx += 1;
        }
    }

    pub fn flatten(&self) -> String {
        self.content
            .iter()
            .fold(String::new(), |init: String, line| {
                if init.is_empty() {
                    line.to_string()
                } else {
                    format!("{}\n{}", init, line)
                }
            })
    }
}
