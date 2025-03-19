use std::collections::HashMap;
use std::io::{self, Write};

use super::ViewID;
use crate::terminal::cursor::Cursor;

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

pub struct View {
    id: ViewID,
    pos: (Pos, Pos),
    curr_line: usize,
    content: Vec<String>,
    scroll: u64,
    height: Pos,
    width: Pos,
    prior: u8,
    //settings: Settings
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
        }
    }

    pub fn get_pos(&self, term_height: u16, term_width: u16) -> (u16, u16) {
        (
            self.pos.0.unwrap(term_width),
            self.pos.1.unwrap(term_height),
        )
    }

    pub fn push(&mut self, ch: char, csr: &mut Cursor) {
        let col = csr.get_x() as usize;
        let length = self.content[self.curr_line].len();
        if col > length {
            self.content[self.curr_line].push(ch);
        }
    }

    pub fn push_str(&mut self, string: &str) {
        self.content[self.curr_line].push_str(string);
    }

    pub fn push_line(&mut self) {
        self.content.push(String::new());
        self.curr_line += 1;
    }

    pub fn up(&mut self) {
        if self.curr_line > 0 {
            self.curr_line -= 1;
        }
    }

    pub fn draw(&self, term_height: u16, term_width: u16) -> io::Result<()> {
        let x_pos = self.pos.0.unwrap(term_width);
        let y_pos = self.pos.1.unwrap(term_height);
        let height = self.height.unwrap(term_height);
        let width = self.width.unwrap(term_width);

        let line_num_offset = 5;
        let mut line_num = 1;

        let max_line = width - x_pos - line_num_offset;

        Cursor::set_csr(x_pos, y_pos);

        for line in self.content.iter().take(self.content.len() - 1) {
            Cursor::csr_setcol(x_pos);
            print!("{:>4}│", line_num);
            if line.len() as u16 > max_line {
                for (subline, cnt) in line.splitn_at(max_line as usize) {
                    if cnt >= 1 {
                        Cursor::csr_setcol(x_pos);
                        print!("    │",);
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

        print!("{:>4}│", line_num);
        let last_line = self.content.last().unwrap();
        if last_line.len() as u16 > max_line {
            for (subline, cnt) in last_line.splitn_at(max_line as usize) {
                if cnt >= 1 {
                    Cursor::csr_setcol(x_pos);
                    print!("    │",);
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
