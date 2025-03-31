use crate::{
    color::{Color, Colorful},
    file::FileMod,
    settings::Settings,
    terminal::{cursor::Cursor, term::Term},
};
use getch_rs::{Getch, Key};
use std::{
    collections::HashMap,
    io::{self, stdout, Write},
};

use crate::view::{
    bottombar::BottomBar, filetree::FileTree, mainview::MainView, menu::Menu, topbar::TopBar, Pos,
    View, ViewID,
};

pub struct Screen {
    focus: ViewID,
    id_cnt: u64,
    view_map: HashMap<ViewID, Box<dyn View>>,
    tmp_buffer: String,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            focus: 0,
            id_cnt: 1,
            view_map: HashMap::new(),
            tmp_buffer: String::new(),
        }
    }

    pub fn init(
        &mut self,
        term: &Term,
        file_mod: &mut FileMod,
        settings: &mut Settings,
    ) -> io::Result<()> {
        let main_view = MainView::new();
        let top_bar = TopBar::new();
        let bottom_bar = BottomBar::new();
        let file_tree = FileTree::new();
        //let menu = Menu::new();

        //main_view.init(term, file_mod, settings);
        settings.num_offset = 6;
        settings.is_show_num = true;

        //self.register(Box::new(main_view.menu));
        self.register(Box::new(main_view));
        self.register(Box::new(top_bar));
        self.register(Box::new(bottom_bar));
        self.register(Box::new(file_tree));

        for (_, view) in self.view_map.iter_mut() {
            view.init(term, file_mod, settings);
        }

        self.focus = 1;

        Cursor::reset_csr();
        stdout().flush()?;
        Ok(())
    }

    fn refresh(term: &Term) {
        Cursor::save_csr();
        Cursor::reset_csr();
        print!(
            "{}",
            " ".repeat(term.size())
                .color(&Color::new(0x28, 0x28, 0x28), &Color::new(0x10, 0x10, 0x10)),
        );

        //print!("{}", " ".repeat(term.size()));
        Cursor::restore_csr();
    }

    fn clean(term: &Term) -> std::io::Result<()> {
        Cursor::reset_csr();
        print!("{}", " ".repeat(term.size()));
        Cursor::reset_csr();
        stdout().flush()?;
        Ok(())
    }

    fn register(&mut self, view: Box<dyn View>) {
        self.view_map.insert(self.id_cnt, view);
        self.id_cnt += 1;
    }

    fn shift(&mut self) {
        let mut new = self.focus % (self.id_cnt - 1) + 1;
        while self.view_map.get(&new).unwrap().is_silent() {
            new = new % (self.id_cnt - 1) + 1;
        }
        self.focus = new;
    }

    pub fn interact(
        &mut self,
        term: Term,
        file_mod: &mut FileMod,
        settings: &Settings,
    ) -> io::Result<()> {
        Screen::clean(&term)?;

        let mut cls = true;
        loop {
            let ch = Getch::new();

            if cls {
                //Screen::refresh(&term);

                for (id, view) in self.view_map.iter_mut() {
                    view.update(&term, file_mod);
                    if *id != self.focus {
                        view.draw(&term, settings)?;
                    }
                }
                let main_view = self.view_map.get_mut(&self.focus).unwrap();
                main_view.draw(&term, settings)?;
                main_view.set_cursor(&term, settings);
            }
            let main_view = self.view_map.get_mut(&self.focus).unwrap();
            stdout().flush()?;

            cls = true;
            match ch.getch() {
                // press ESC to leave
                Ok(Key::Esc) => break,

                Ok(Key::F(5)) => {
                    if !main_view.is_lock() {
                        self.shift();
                    }
                }

                // reserve key F1 ~ F5 for fixed function
                Ok(Key::F(f)) if f <= 5 => {
                    cls = false;
                    dbg!(&f);
                }

                // for debug
                Ok(Key::Ctrl('d')) => {
                    cls = false;
                    let con = &file_mod.curr().flatten();
                    dbg!(String::from_utf8_lossy(con));
                }
                Ok(Key::Ctrl('k')) => {
                    cls = false;
                    dbg!(&file_mod);
                }

                Ok(Key::Ctrl('s')) => {
                    file_mod.save()?;
                }

                Ok(Key::Alt(key)) => {
                    cls = false;
                    dbg!(key);
                }

                Ok(Key::Other(key)) => {
                    match key[..] {
                        // Alt(Left)
                        [27, 91, 49, 59, 51, 68] => {
                            main_view.resize(-1, 0, 0, 0);
                        }
                        // Alt(Right)
                        [27, 91, 49, 59, 51, 67] => {
                            main_view.resize(1, 0, 0, 0);
                        }
                        _ => (),
                    };
                }

                // measure input key
                Ok(key) => {
                    main_view.matchar(&term, file_mod, settings, key);
                }
                Err(e) => panic!("{}", e),
            }
            file_mod.update()?;
        }

        Screen::clean(&term)?;
        Ok(())
    }
}

/*
#[derive(Default)]
struct BufferLine {
    //pos: (u16, u16),
    height: u16,
    width: u16,
    file_names: Vec<String>,
}

struct TopBar {}

struct BottomBar {}

struct FileTree {}
*/
