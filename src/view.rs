use crate::screen::Module;
use crate::terminal::term::Term;
use getch_rs::Key;
use std::cmp::Ordering;
use std::io;
use std::ops::Add;
use widestring::Utf16String;

pub mod bottombar;
pub mod filetree;
pub mod help;
pub mod mainview;
pub mod menu;
pub mod msgbox;
pub mod topbar;

pub type ViewID = u64;

/// 枚举类型`Pos`
/// - `Fixed`：离左/上边框的距离
/// - `Opposite`：离右/下边框的距离
#[derive(Debug, Clone)]
pub enum Pos {
    Fixed(u16),
    Opposite(u16),
}

impl Pos {
    /// 将相对距离转变为绝对距离
    pub fn unwrap(&self, value: u16) -> u16 {
        match self {
            Pos::Fixed(val) => *val + 1,
            Pos::Opposite(val) => value - *val + 1,
        }
    }

    /// 直接取值
    pub fn get(&self) -> u16 {
        match self {
            Pos::Fixed(val) => *val,
            Pos::Opposite(val) => *val,
        }
    }
}

/// 加法操作：直接对枚举值进行加法
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

/// 类型转换
/// - 将正数转换为`Pos::Fixed(val_abs)`
/// - 将负数转换为`Pos::Opposite(val_abs)`
impl TryFrom<i16> for Pos {
    type Error = &'static str;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let val_abs = value.unsigned_abs() - 1;

        match value.cmp(&0) {
            Ordering::Greater => Ok(Pos::Fixed(val_abs)),
            Ordering::Less => Ok(Pos::Opposite(val_abs)),
            Ordering::Equal => Err("value can't be zero"),
        }
    }
}

/// 后端trait，定义了视图的显示/杂项功能
pub trait Position {
    fn get_name(&self) -> &String;
    fn get_start(&self, term: &Term) -> (u16, u16);
    fn get_end(&self, term: &Term) -> (u16, u16);
    fn resize(&mut self, term: &Term, dx_s: i16, dy_s: i16, dx_e: i16, dy_e: i16);
    fn is_silent(&self) -> bool;
    fn is_lock(&self) -> bool;
    fn is_show(&self) -> bool;
}

/// 前端核心trait，定义了视图的基本功能
/// 继承自`Position`
pub trait View: Position {
    /// 初始化
    fn init(&mut self, module: &mut Module);
    /// 匹配字符，编写字符处理逻辑
    fn matchar(&mut self, module: &mut Module, key: Key);
    /// 编写光标位置处理逻辑
    fn set_cursor(&self, module: &mut Module);
    /// 更新操作，在`draw`前调用
    fn update(&mut self, module: &mut Module);
    /// 核心绘图操作，用于显示内容
    fn draw(&self, module: &mut Module) -> io::Result<()>;
}

/// 为字符串相关类型附加的迭代器trait
pub trait SplitNAt {
    /// 把字符串分成长度为`mid`的子串，并返回迭代器
    fn splitn_at(&self, mid: usize) -> SplitNAtIter<Self>
    where
        Self: std::marker::Sized;
}

impl SplitNAt for String {
    fn splitn_at(&self, mid: usize) -> SplitNAtIter<Self> {
        SplitNAtIter {
            string: self.clone(),
            is_end: false,
            count: 0,
            mid,
        }
    }
}

impl SplitNAt for Utf16String {
    fn splitn_at(&self, mid: usize) -> SplitNAtIter<Self> {
        SplitNAtIter {
            string: self.clone(),
            is_end: false,
            count: 0,
            mid,
        }
    }
}

pub struct SplitNAtIter<T>
where
    T: std::marker::Sized,
{
    string: T,
    is_end: bool,
    count: u64,
    mid: usize,
}

impl Iterator for SplitNAtIter<String> {
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
            let (first, last): (String, String);

            if string.is_char_boundary(mid) {
                let (f, l) = string.split_at(mid);
                first = f.to_string();
                last = l.to_string();
            } else {
                let (f, l) = string.split_at(mid - 1);
                first = String::from(f) + " ";
                last = l.to_string();
            };
            self.string = last;
            Some((first, count))
        } else {
            self.is_end = true;
            Some((string, count))
        }
    }
}

impl Iterator for SplitNAtIter<Utf16String> {
    type Item = (Utf16String, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end {
            return None;
        }
        let mid = self.mid;
        let string = self.string.clone();
        let count = self.count;
        self.count += 1;
        if string.len() > mid {
            let (first, last): (Utf16String, Utf16String);

            if string.is_char_boundary(mid) {
                let (f, l) = string.split_at(mid);
                first = f.into();
                last = l.into();
            } else {
                let (f, l) = string.split_at(mid - 1);
                first = Utf16String::from(f) + " ";
                last = l.into();
            };
            self.string = last;
            Some((first, count))
        } else {
            self.is_end = true;
            Some((string, count))
        }
    }
}
