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
            (Pos::Fixed(3), Pos::Fixed(3)),
            Pos::Opposite(2),
            Pos::Opposite(1),
        );
        main_view.settings().num_offset = 5;
        self.register(main_view);
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
                .color(&Color::new(0xa0, 0xa0, 0xa0), &Color::new(0x10, 0x10, 0x10)),
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
        let main_view = self.view_map.get_mut(&self.focus).unwrap();
        cursor.set(main_view.get_text_pos(&term));
        loop {
            let ch = Getch::new();
            let mut cls = true;
            match ch.getch() {
                Ok(Key::Char('\r')) => {
                    main_view.push_line(cursor, &term);
                }
                Ok(Key::Char('\t')) => {
                    main_view.push_str("    ");
                    cursor.move_csr(4, CsrMove::Right);
                }
                Ok(Key::Char(char)) => {
                    //cls = false;
                    main_view.push(char, cursor, &term);
                    /*
                    self.tmp_buffer.push(char);
                    if char == '\r' {
                        //view.push('\n');
                    }
                    */
                }
                Ok(Key::Delete) => {
                    main_view.delete(cursor, &term);
                }
                Ok(Key::Up) => {
                    main_view.up(cursor, &term);
                }

                Ok(Key::Down) => {
                    main_view.down(cursor, &term);
                }

                Ok(Key::Left) => {
                    //cls = false;
                    main_view.left(cursor, &term);
                }

                Ok(Key::Right) => {
                    //cls = false;
                    main_view.right(cursor, &term);
                }

                Ok(Key::Esc) => break,
                Ok(Key::Ctrl('d')) => {
                    cls = false;
                    dbg!(&cursor);
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

            main_view.draw(&term)?;
            cursor.sync();
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
