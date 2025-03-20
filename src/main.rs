use clap::Parser;
use screen::{Draw, Screen};
use terminal::{cursor::Cursor, term::Term};

mod color;
mod file;
mod screen;
mod terminal;

/*
*       TgEd
*        |
*  |-----|----------|
* File  Screen  Settings
*        |
*       View
*/

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
    screen.init();

    // start interact
    screen.interact(&mut Cursor::new(), term)?;
    Ok(())
}
