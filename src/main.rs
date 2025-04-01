use clap::Parser;
use file::FileMod;
use screen::{Module, Screen};
use settings::Settings;
use std::{
    io::{self, IsTerminal},
    path::Path,
};
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
    #[arg(value_name = "FILE")]
    pub files_name: Vec<String>,

    #[arg(short = 'd', long = "dir", value_name = "DIR", default_value_t = String::from("."))]
    pub dir: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !io::stdout().is_terminal() {
        return Err("Please use in terminal/tty!".into());
    }
    let mut file_mod: FileMod;
    let mut term = Term::new();
    let mut settings = Settings::default();
    let mut screen = Screen::new();

    let args = Args::parse();

    let files_name = args.files_name;
    if files_name.is_empty() {
        file_mod = FileMod::new(args.dir.into());
    } else {
        if files_name.iter().fold(false, |flag, file| {
            let path = Path::new(file);
            if path.is_dir() {
                true
            } else {
                flag
            }
        }) {
            return Err("Please don't include directory in [FILE]".into());
        };
        file_mod = FileMod::from(files_name);
        file_mod.set_dir(args.dir.into());
    }
    term.init();

    let mut module = Module::new(term, file_mod, settings);
    screen.init(&mut module)?;
    // start interact
    screen.interact(&mut module)?;

    Ok(())
}
