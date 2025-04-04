use clap::Parser;
use crossbeam_channel::{bounded, select, Receiver};
use file::FileMod;
use getch_rs::{Getch, Key};
use screen::{Module, Screen};
use settings::Settings;
use std::{
    io::{self, IsTerminal},
    path::Path,
    thread,
};
use terminal::term::Term;
use view::{
    msgbox::{self, MsgBox},
    View,
};

use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;

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

fn key_channel() -> Receiver<Key> {
    let ch = Getch::new();
    let (sender, receiver) = bounded(500);
    thread::spawn(move || loop {
        let key = ch.getch().unwrap();
        sender.send(key).unwrap();
    });
    receiver
}

fn term_channel() -> Receiver<Term> {
    let mut signals = Signals::new([SIGWINCH]).unwrap();
    let (sender, receiver) = bounded(500);
    thread::spawn(move || {
        for _ in signals.forever() {
            let mut new_tem = Term::new();
            new_tem.init();
            sender.send(new_tem).unwrap();
        }
    });
    receiver
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !io::stdout().is_terminal() {
        return Err("Please use in terminal/tty".into());
    }

    let args = Args::parse();

    let mut file_mod: FileMod;
    let mut term = Term::new();
    let mut settings = Settings::default();
    let mut screen = Screen::new();

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

    let key_events = key_channel();
    let term_events = term_channel();

    term.init();
    let mut module = Module::new(term, file_mod, settings, key_events.clone());

    screen.init(&mut module)?;
    Screen::clean(&module.term)?;
    screen.update(&mut module)?;
    loop {
        // start interact
        select! {
            recv(key_events) -> key => {
                if screen.interact(&mut module, key?)? {
                    break;
                };
            }

            recv(term_events) -> term => {
                module.term = term?;
                screen.update(&mut module)?;
            }
        };
    }

    Screen::clean(&module.term)?;

    Ok(())
}
