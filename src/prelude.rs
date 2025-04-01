#[allow(unused)]
pub use crate::{
    color::{Color, Colorful, END},
    file::FileMod,
    screen::{Module, Op},
    settings::Settings,
    terminal::{cursor::Cursor, term::Term},
    view::{Pos, Position, View, ViewID},
};
pub use std::io::{self, Write};
pub use tged::view;
