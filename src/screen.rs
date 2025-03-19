use crate::terminal::{
    self,
    cursor::{CsrMove, Cursor},
    term::{self, Term},
};
use getch_rs::{Getch, Key};
use std::{
    collections::{HashMap, HashSet},
    io::{self, stdout, Read, Write},
};
use view::{Pos, View};

mod interact;
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
    term: Term,
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
            term: Term::new(),
            tmp_buffer: String::new(),
        }
    }

    pub fn init(&mut self) {
        self.term.get_term_size();
        let main_view = View::new(
            (Pos::Fixed(3), Pos::Fixed(3)),
            Pos::Opposite(2),
            Pos::Opposite(1),
        );
        self.register(main_view);
        self.focus = 1;

        //self.setting.pos = (2, 2);
        Cursor::reset_csr();
        print!("{}", " ".repeat(self.term.size()));
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

    fn clean(&self) -> std::io::Result<()> {
        Cursor::reset_csr();
        print!("{}", " ".repeat(self.term.size()));
        Cursor::reset_csr();
        stdout().flush()?;
        Ok(())
    }

    fn register(&mut self, view: View) {
        self.view_map.insert(self.id_cnt, view);
        self.id_cnt += 1;
    }

    pub fn interact(&mut self, cursor: &mut Cursor) -> io::Result<()> {
        let view = self.view_map.get_mut(&self.focus).unwrap();
        cursor.set(view.get_pos(self.term.height, self.term.width));
        loop {
            let ch = Getch::new();
            match ch.getch() {
                Ok(Key::Char('\r')) => {
                    view.push_line();
                    cursor.move_csr(1, CsrMove::Down);
                }
                Ok(Key::Char('\t')) => {
                    view.push_str("    ");
                    cursor.move_csr(4, CsrMove::Right);
                }
                Ok(Key::Char(char)) => {
                    view.push(char, cursor);
                    cursor.move_csr(1, CsrMove::Right);
                    /*
                    self.tmp_buffer.push(char);
                    if char == '\r' {
                        //view.push('\n');
                    }
                    */
                }
                Ok(Key::Up) => {
                    view.up();
                    cursor.move_csr(1, CsrMove::Up);
                }
                Ok(Key::Esc) => break,
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }
            view.draw(self.term.height, self.term.width)?;
            //cursor.sync();
            //dbg!(&cursor);
            stdout().flush()?;
        }
        self.clean()?;
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
