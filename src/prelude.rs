pub use crate::file::FileMod;
pub use crate::{
    color::{Color, Colorful, END},
    settings::Settings,
    terminal::{cursor::Cursor, term::Term},
    view::{Pos, Position, View, ViewID},
};
pub use std::io::{self, Write};
pub use tged::view;
