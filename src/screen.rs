use crate::{
    color::{Color, Colorful},
    terminal::{cursor::Cursor, term::Term},
};
use getch_rs::{Getch, Key};
use std::{
    collections::HashMap,
    io::{self, stdout, Write},
};

use crate::view::{bottombar::BottomBar, mainview::MainView, topbar::TopBar, Pos, View, ViewID};

pub trait Draw {
    fn draw(&self) -> std::io::Result<()>;
    fn nextline(&self);
}

pub struct Screen {
    focus: ViewID,
    id_cnt: u64,
    view_map: HashMap<ViewID, Box<dyn View>>,
    tmp_buffer: String,
}

impl Draw for Screen {
    fn draw(&self) -> std::io::Result<()> {
        //term::set_csr(self.setting.pos.0, self.setting.pos.1);
        print!("{}", self.tmp_buffer);
        stdout().flush()?;
        Ok(())
    }

    fn nextline(&self) {
        Cursor::csr_nextline();
        Cursor::csr_setcol(5);
    }
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

    pub fn init(&mut self, term: &Term) {
        let mut main_view = MainView::new(
            (Pos::Fixed(5), Pos::Fixed(1)),
            Pos::Opposite(1),
            Pos::Opposite(1),
        );
        let mut top_bar = TopBar::new();
        let mut bottom_bar = BottomBar::new();
        /*
        let mut top_bar = MainView::new(
            (Pos::Fixed(0), Pos::Fixed(0)),
            Pos::Fixed(1),
            Pos::Opposite(0),
        );
        */
        top_bar.push_str("Hello B3r");
        bottom_bar.push_str("Hello Bottom");

        main_view.settings().num_offset = 5;
        main_view.settings().is_show_num = true;
        self.register(Box::new(main_view));
        self.register(Box::new(top_bar));
        self.register(Box::new(bottom_bar));
        self.focus = 1;

        //self.setting.pos = (2, 2);
        Cursor::reset_csr();
        //term::set_csr(self.setting.pos.0, self.setting.pos.1);
        stdout().flush().unwrap();
    }

    fn refresh(term: &Term) {
        Cursor::save_csr();
        Cursor::reset_csr();
        print!(
            "{}",
            " ".repeat(term.size())
                .color(&Color::new(0x20, 0x20, 0x20), &Color::new(0x10, 0x10, 0x10)),
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

    pub fn interact(&mut self, term: Term) -> io::Result<()> {
        Screen::clean(&term)?;
        loop {
            let main_view = self.view_map.get_mut(&self.focus).unwrap();
            let ch = Getch::new();
            let mut cls = true;
            match ch.getch() {
                // press ESC to leave
                Ok(Key::Esc) => break,

                // reserve key F1 ~ F5 for fixed function
                Ok(Key::F(f)) if f <= 5 => {
                    cls = false;
                    dbg!(&f);
                }

                // for debug
                Ok(Key::Ctrl('d')) => {
                    cls = false;
                }
                Ok(Key::Ctrl('k')) => {
                    cls = false;
                    //dbg!(&main_view);
                }

                // measure input key
                Ok(key) => {
                    main_view.matchar(&term, key);
                }
                Err(e) => panic!("{}", e),
            }

            if cls {
                Screen::refresh(&term);
            }

            for (id, view) in self.view_map.iter() {
                if *id != self.focus {
                    view.draw(&term)?;
                }
            }
            let main_view = self.view_map.get_mut(&self.focus).unwrap();
            main_view.draw(&term)?;
            main_view.set_cursor(&term);
            stdout().flush()?;
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
