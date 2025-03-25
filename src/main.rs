use clap::Parser;
use screen::{Draw, Screen};
use terminal::{cursor::Cursor, term::Term};
use view::Pos;

mod color;
mod file;
mod macros;
mod screen;
mod settings;
mod terminal;
mod view;

/*
*       TgEd
*        |
*  |-----|----------|
* File  Screen  Settings
*        |
*       View
*/

use crate::view::settings::Settings;
use tged::view;

#[derive(Parser)]
#[command(version = "0.1.0",author = "NazrinDuck", about, long_about = None)]
pub struct Args {
    pub files_name: Vec<String>,

    #[arg(short = 'g', long = "debug", action = clap::ArgAction::Count)]
    pub debug: u8,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut term = Term::new();
    term.init();

    let mut screen = Screen::new();

    screen.init(&term);

    // start interact
    screen.interact(term)?;
    Ok(())
}
