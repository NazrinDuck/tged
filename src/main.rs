use clap::Parser;
use file::FileMod;
use screen::Screen;
use settings::Settings;
use terminal::{cursor::Cursor, term::Term};
use view::Pos;

mod color;
mod file;
mod macros;
mod prelude;
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

//use crate::view::settings::Settings;
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

    let mut file_mod = FileMod::from(args.files_name);

    let mut term = Term::new();
    term.init();

    let mut screen = Screen::new();
    let mut settings = Settings::default();

    screen.init(&term, &mut file_mod, &mut settings)?;

    // start interact
    screen.interact(term, &mut file_mod, &mut settings)?;
    Ok(())
}
