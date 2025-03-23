use crate::terminal::cursor::CsrMove;
use std::{
    collections::HashMap,
    io::{self, Write},
    usize,
};

use super::ViewID;

use crate::terminal::term::Term;
use crate::{screen::settings::Settings, terminal::cursor::Cursor};

#[derive(Debug)]
pub enum Pos {
    Fixed(u16),
    Opposite(u16),
}

impl Pos {
    fn unwrap(&self, value: u16) -> u16 {
        match self {
            Pos::Fixed(val) => *val,
            Pos::Opposite(val) => value - val,
        }
    }
}

struct Line {
    line: String,
    height: u16,
}

impl Line {
    fn new() -> Self {
        Line {
            line: String::new(),
            height: 1u16,
        }
    }
}

#[derive(Debug)]
pub struct View {
    id: ViewID,
    pos: (Pos, Pos),
    pub curr_line: usize,
    pub curr_idx: usize,
    content: Vec<String>,
    pub scroll: usize,
    height: Pos,
    width: Pos,
    prior: u8,
    settings: Settings,
}

impl View {
    pub fn new(pos: (Pos, Pos), height: Pos, width: Pos) -> Self {
        View {
            id: 0,
            pos,
            curr_line: 0,
            curr_idx: 0,
            content: Vec::from([String::new()]),
            scroll: 0,
            height,
            width,
            prior: 0,
            settings: Settings::default(),
        }
    }

    pub fn settings(&mut self) -> &mut Settings {
        &mut self.settings
    }

    #[inline]
    pub fn get_pos(&self, term: &Term) -> (u16, u16) {
        let (height, width) = (term.height, term.width);
        (self.pos.0.unwrap(width), self.pos.1.unwrap(height))
    }

    #[inline]
    pub fn get_text_pos(&self, term: &Term) -> (u16, u16) {
        let (height, width) = (term.height, term.width);
        (
            self.pos.0.unwrap(width) + self.settings.num_offset,
            self.pos.1.unwrap(height),
        )
    }

    #[inline]
    fn get_csr_vpos(&self, csr: &mut Cursor, term: &Term) -> (usize, usize) {
        let (x, y) = self.get_text_pos(term);
        let col: usize = (csr.get_x() - x) as usize;
        let row: usize = (csr.get_y() - y) as usize;
        (col, row)
    }

    #[inline]
    fn get_vpos_max(&self, term: &Term) -> usize {
        (self.width.unwrap(term.width) - self.get_text_pos(term).0) as usize
    }

    #[inline]
    fn vpos(&self, term: &Term, val: u16) -> u16 {
        self.get_text_pos(term).0 + val
    }

    #[inline]
    fn wrap_col(&self, term: &Term, col: usize) -> usize {
        let length = self.content[self.curr_line].len();
        let max = self.get_vpos_max(term);
        let mut col = col;

        let cnt: usize = length / max - if col == max { 1 } else { 0 };
        col += max * cnt;
        col
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

        let height = (self.height.unwrap(term.height) - self.pos.1.unwrap(term.height)) as usize;

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
        self.content[self.curr_line].insert(self.curr_idx, ch);
        self.curr_idx += 1;
    }

    pub fn push_str(&mut self, string: &str) {
        self.content[self.curr_line].push_str(string);
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
            self.content.remove(line);

            self.curr_idx = self.content[line - 1].len();
            self.line_dec(term);
            //self.curr_line -= 1;
        } else {
            self.content[line].remove(idx - 1);
            self.curr_idx -= 1;
        }
    }

    pub fn up(&mut self, term: &Term) {
        let (line, idx) = (self.curr_line, self.curr_idx);

        if line > 0 {
            //self.curr_line -= 1;
            if idx > self.content[line - 1].len() {
                self.curr_idx = self.content[line - 1].len()
            };
            self.line_dec(term);
        }
    }

    pub fn down(&mut self, term: &Term) {
        let (line, idx) = (self.curr_line, self.curr_idx);

        if line < self.content.len() - 1 {
            //self.curr_line += 1;
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

    pub fn set_cursor(&self, term: &Term) {
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

    pub fn draw(&self, term: &Term) -> io::Result<()> {
        let (height, width) = (term.height, term.width);

        let x_pos = self.pos.0.unwrap(width);
        let y_pos = self.pos.1.unwrap(height);

        let height = self.height.unwrap(height);
        let width = self.width.unwrap(width);

        let is_show_num = self.settings.is_show_num;
        let line_num_offset = self.settings.num_offset;
        let mut line_num = self.scroll + 1;

        let max_line = width - x_pos - line_num_offset;
        let max_height = (height - y_pos) as usize;

        Cursor::set_csr(x_pos, y_pos);

        let mut height_cnt = 0;
        for line in self.content.iter().skip(self.scroll)
        //.take((height - y_pos) as usize)
        {
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
        }
        io::stdout().flush()?;
        Ok(())
    }
}

pub trait SplitN {
    fn splitn_at(&self, mid: usize) -> SplitNAt;
}

impl SplitN for String {
    fn splitn_at(&self, mid: usize) -> SplitNAt {
        SplitNAt {
            string: self.clone(),
            is_end: false,
            count: 0,
            mid,
        }
    }
}

pub struct SplitNAt {
    string: String,
    is_end: bool,
    count: u64,
    mid: usize,
}

impl Iterator for SplitNAt {
    type Item = (String, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end {
            return None;
        }
        let mid = self.mid;
        let string = self.string.clone();
        let count = self.count;
        self.count += 1;
        if string.len() > mid {
            let (first, last) = string.split_at(mid);
            self.string = last.to_string();
            Some((first.to_string(), count))
        } else {
            self.is_end = true;
            Some((string, count))
        }
    }
}
