use clap::Parser;
use file::FileMod;
use screen::Screen;
use settings::Settings;
use std::io::{self, IsTerminal};
use terminal::term::Term;

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

#[derive(Parser)]
#[command(version = "0.1.0",author = "NazrinDuck", about, long_about = None)]

pub struct Args {
    pub files_name: Vec<String>,

    #[arg(short = 'g', long = "debug", action = clap::ArgAction::Count)]
    pub debug: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !io::stdout().is_terminal() {
        return Err("Please use in terminal/tty!".into());
    }

    let args = Args::parse();

    let files_name = args.files_name;

    if files_name.is_empty() {}

    let mut file_mod = FileMod::from(files_name);
    let mut term = Term::new();
    let mut settings = Settings::default();
    let mut screen = Screen::new();

    term.init();

    screen.init(&term, &mut file_mod, &mut settings)?;

    // start interact
    screen.interact(term, &mut file_mod, &mut settings)?;
    Ok(())
}
