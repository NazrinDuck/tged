use crate::screen::Module;
use crate::settings::Settings;
use crate::terminal::term::Term;
use crate::FileMod;
use getch_rs::Key;
use std::io;
use std::ops::Add;

pub mod bottombar;
pub mod filetree;
pub mod help;
pub mod mainview;
pub mod menu;
pub mod settings;
pub mod topbar;

pub type ViewID = u64;

#[derive(Debug, Clone)]
pub enum Pos {
    Fixed(u16),
    Opposite(u16),
}

impl Pos {
    pub fn unwrap(&self, value: u16) -> u16 {
        match self {
            Pos::Fixed(val) => *val + 1,
            Pos::Opposite(val) => value - *val + 1,
        }
    }

    pub fn get(&self) -> u16 {
        match self {
            Pos::Fixed(val) => *val,
            Pos::Opposite(val) => *val,
        }
    }
}

impl Add<i16> for Pos {
    type Output = Self;
    fn add(self, rhs: i16) -> Self::Output {
        if rhs == 0 {
            return self;
        }
        let abs = rhs.unsigned_abs();
        let mut val = self.get();
        if rhs > 0 {
            val += abs;
        } else {
            if val < abs {
                return self;
            }
            val -= abs;
        }

        match self {
            Pos::Fixed(_) => Pos::Fixed(val),
            Pos::Opposite(_) => Pos::Opposite(val),
        }
    }
}

pub trait Position {
    fn get_name(&self) -> &String;
    fn get_start(&self, term: &Term) -> (u16, u16);
    fn get_end(&self, term: &Term) -> (u16, u16);
    fn resize(&mut self, dx_s: i16, dy_s: i16, dx_e: i16, dy_e: i16);
    fn is_silent(&self) -> bool;
    fn is_lock(&self) -> bool;
    fn is_show(&self) -> bool;
}

pub trait View: Position {
    fn init(&mut self, module: &mut Module);
    fn matchar(&mut self, module: &mut Module, key: Key);
    fn set_cursor(&self, module: &mut Module);
    fn update(&mut self, module: &mut Module);
    fn draw(&self, module: &mut Module) -> io::Result<()>;
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
