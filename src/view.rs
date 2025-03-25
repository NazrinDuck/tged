use crate::terminal::term::Term;
use getch_rs::Key;
use std::io;

pub mod bottombar;
pub mod mainview;
pub mod settings;
pub mod topbar;

pub type ViewID = u64;

#[derive(Debug)]
pub enum Pos {
    Fixed(u16),
    Opposite(u16),
}

impl Pos {
    pub fn unwrap(&self, value: u16) -> u16 {
        match self {
            Pos::Fixed(val) => *val + 1,
            Pos::Opposite(val) => value - val + 1,
        }
    }
}

pub trait View {
    fn matchar(&mut self, term: &Term, key: Key);
    fn set_cursor(&self, term: &Term);
    fn draw(&self, term: &Term) -> io::Result<()>;
}

pub trait SplitNAt {
    fn splitn_at(&self, mid: usize) -> SplitNAtIter;
}

impl SplitNAt for String {
    fn splitn_at(&self, mid: usize) -> SplitNAtIter {
        SplitNAtIter {
            string: self.clone(),
            is_end: false,
            count: 0,
            mid,
        }
    }
}

pub struct SplitNAtIter {
    string: String,
    is_end: bool,
    count: u64,
    mid: usize,
}

impl Iterator for SplitNAtIter {
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
