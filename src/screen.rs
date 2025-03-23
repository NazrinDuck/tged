use crate::{
    color::{Color, Colorful},
    terminal::{
        self,
        cursor::{CsrMove, Cursor},
        term::{self, Term},
    },
};
use getch_rs::{Getch, Key};
use std::{
    collections::{HashMap, HashSet},
    io::{self, stdout, Read, Write},
};
use view::{Pos, View};

mod interact;
mod settings;
mod view;

pub trait Draw {
    fn draw(&self) -> std::io::Result<()>;
    fn nextline(&self);
}

type ViewID = u64;

pub struct Screen {
    focus: ViewID,
    id_cnt: u64,
    view_map: HashMap<ViewID, View>,
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

    pub fn init(&mut self) {
        let mut main_view = View::new(
            (Pos::Fixed(6), Pos::Fixed(12)),
            Pos::Opposite(5),
            Pos::Opposite(12),
        );
        let mut bottom_bar = View::new(
            (Pos::Fixed(0), Pos::Fixed(0)),
            Pos::Fixed(1),
            Pos::Opposite(1),
        );
        bottom_bar.push_str("Hello Bar");

        main_view.settings().num_offset = 5;
        main_view.settings().is_show_num = true;
        bottom_bar.settings().is_show_num = false;
        self.register(main_view);
        self.register(bottom_bar);
        self.focus = 1;

        //self.setting.pos = (2, 2);
        Cursor::reset_csr();
        //term::set_csr(self.setting.pos.0, self.setting.pos.1);
        stdout().flush().unwrap();
    }

    fn update(&self) -> std::io::Result<()> {
        Cursor::save_csr();
        Cursor::reset_csr();
        self.draw()?;
        Cursor::restore_csr();
        Ok(())
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

    fn register(&mut self, view: View) {
        self.view_map.insert(self.id_cnt, view);
        self.id_cnt += 1;
    }

    pub fn interact(&mut self, cursor: &mut Cursor, term: Term) -> io::Result<()> {
        Screen::clean(&term)?;
        //cursor.set(main_view.get_text_pos(&term));
        loop {
            let main_view = self.view_map.get_mut(&self.focus).unwrap();
            let ch = Getch::new();
            let mut cls = true;
            match ch.getch() {
                Ok(Key::Char('\r')) => {
                    main_view.push_line(&term);
                }
                Ok(Key::Char('\t')) => {
                    main_view.push_str("    ");
                }
                Ok(Key::Char(char)) => {
                    main_view.push(char);
                    /*
                    self.tmp_buffer.push(char);
                    if char == '\r' {
                        //view.push('\n');
                    }
                    */
                }
                Ok(Key::Delete) => {
                    main_view.delete(&term);
                }
                Ok(Key::Up) => {
                    main_view.up(&term);
                }

                Ok(Key::Down) => {
                    main_view.down(&term);
                }

                Ok(Key::Left) => {
                    //cls = false;
                    main_view.left();
                }

                Ok(Key::Right) => {
                    //cls = false;
                    main_view.right();
                }

                Ok(Key::Esc) => break,
                Ok(Key::Ctrl('d')) => {
                    Cursor::reset_csr();
                    cls = false;
                    dbg!(&main_view.curr_line);
                    dbg!(&main_view.curr_idx);
                    dbg!(&main_view.scroll);
                }
                Ok(Key::Ctrl('k')) => {
                    cls = false;
                    dbg!(&main_view);
                }
                Ok(other) => {
                    cls = false;
                    dbg!(other);
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
            //cursor.sync();
            //dbg!(&cursor);
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
