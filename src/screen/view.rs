use crate::terminal::cursor::CsrMove;
use std::{
    collections::HashMap,
    io::{self, Write},
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
    curr_line: usize,
    content: Vec<String>,
    scroll: u64,
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

    pub fn push(&mut self, ch: char, csr: &mut Cursor, term: &Term) {
        let (col, row): (usize, usize) = self.get_csr_vpos(csr, term);
        let max = self.get_vpos_max(term);

        if col >= max {
            csr.move_csr(1, CsrMove::Down);
            csr.set_x(self.vpos(term, 0));
        }
        csr.move_csr(1, CsrMove::Right);

        let col = self.wrap_col(term, col);
        self.content[self.curr_line].insert(col, ch);
    }

    pub fn push_str(&mut self, string: &str) {
        self.content[self.curr_line].push_str(string);
    }

    pub fn push_line(&mut self, csr: &mut Cursor, term: &Term) {
        let (col, row): (usize, usize) = self.get_csr_vpos(csr, term);

        csr.set_x(self.vpos(term, 0));
        csr.move_csr(1, CsrMove::Down);

        let col = self.wrap_col(term, col);
        let content = self.content[self.curr_line].clone();
        let (first, last) = content.split_at(col);

        self.content[self.curr_line] = String::from(first);
        self.content.insert(self.curr_line + 1, String::from(last));

        self.curr_line += 1;
    }

    pub fn delete(&mut self, csr: &mut Cursor, term: &Term) {
        let (col, row): (usize, usize) = self.get_csr_vpos(csr, term);

        if col > 0 {
            let col = self.wrap_col(term, col);
            self.content[self.curr_line].remove(col - 1);

            csr.move_csr(1, CsrMove::Left);
        } else if self.curr_line > 0 && self.wrap_col(term, col) == 0 {
            let x = self.get_text_pos(term).0;
            let rest = self.content[self.curr_line].clone();

            self.content.remove(row);
            self.curr_line -= 1;

            let length = self.content[self.curr_line].len();

            self.content[self.curr_line] += &rest;

            csr.move_csr(1, CsrMove::Up);
            csr.set_x(self.vpos(term, length as u16));
        }
    }

    pub fn up(&mut self, csr: &mut Cursor, term: &Term) {
        if self.curr_line > 0 {
            let x = self.get_text_pos(term).0;
            self.curr_line -= 1;
            let col: usize = (csr.get_x() - x) as usize;
            let length = self.content[self.curr_line].len();
            if col > length {
                csr.set_x(self.vpos(term, length as u16));
            }

            csr.move_csr(1, CsrMove::Up);
        }
    }

    pub fn down(&mut self, csr: &mut Cursor, term: &Term) {
        if self.curr_line < self.content.len() - 1 {
            let x = self.get_text_pos(term).0;
            self.curr_line += 1;
            let col: usize = (csr.get_x() - x) as usize;
            let length = self.content[self.curr_line].len();
            if col > length {
                csr.set_x(self.vpos(term, length as u16));
            }

            csr.move_csr(1, CsrMove::Down);
        }
    }

    pub fn left(&mut self, csr: &mut Cursor, term: &Term) {
        let x = self.get_text_pos(term).0;
        if csr.get_x() > x {
            csr.move_csr(1, CsrMove::Left);
        }
    }

    pub fn right(&mut self, csr: &mut Cursor, term: &Term) {
        let x = self.get_text_pos(term).0;
        let col: usize = (csr.get_x() - x) as usize;

        if col < self.content[self.curr_line].len() {
            csr.move_csr(1, CsrMove::Right);
        }
    }

    pub fn draw(&self, term: &Term) -> io::Result<()> {
        let (height, width) = (term.height, term.width);

        let x_pos = self.pos.0.unwrap(width);
        let y_pos = self.pos.1.unwrap(height);
        let height = self.height.unwrap(height);
        let width = self.width.unwrap(width);

        let line_num_offset = self.settings.num_offset;
        let mut line_num = 1;

        let max_line = width - x_pos - line_num_offset;

        Cursor::set_csr(x_pos, y_pos);

        for line in self.content.iter().take(self.content.len() - 1) {
            Cursor::csr_setcol(x_pos);
            print!(
                "{:>width$}│",
                line_num,
                width = (line_num_offset - 1) as usize
            );
            if line.len() as u16 > max_line {
                for (subline, cnt) in line.splitn_at(max_line as usize) {
                    if cnt >= 1 {
                        Cursor::csr_setcol(x_pos);
                        print!("{:>width$}", "│", width = line_num_offset as usize);
                    }
                    Cursor::csr_setcol(x_pos + line_num_offset);
                    print!("{}", subline);
                    Cursor::csr_nextline();
                    //dbg!(subline);
                }
            } else {
                Cursor::csr_setcol(x_pos + line_num_offset);
                println!("{}", line);
            }
            line_num += 1;
        }
        Cursor::csr_setcol(x_pos);

        print!(
            "{:>width$}│",
            line_num,
            width = (line_num_offset - 1) as usize
        );
        let last_line = self.content.last().unwrap();
        if last_line.len() as u16 > max_line {
            for (subline, cnt) in last_line.splitn_at(max_line as usize) {
                if cnt >= 1 {
                    Cursor::csr_setcol(x_pos);
                    print!("{:>width$}", "│", width = line_num_offset as usize);
                }
                if subline.len() < max_line as usize {
                    print!("{}", subline);
                } else {
                    print!("{}", subline);
                    Cursor::csr_nextline();
                    Cursor::csr_setcol(x_pos + line_num_offset);
                }
                //dbg!(subline);
            }
        } else {
            print!("{}", last_line);
        }
        io::stdout().flush()?;
        //dbg!(&self.content);
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

pub struct ViewMap {
    map: HashMap<ViewID, View>,
    curr_view: ViewID,
}

impl ViewMap {}
