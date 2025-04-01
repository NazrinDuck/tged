use crate::{
    file::FileMod,
    settings::Settings,
    terminal::{cursor::Cursor, term::Term},
};
use getch_rs::{Getch, Key};
use std::{
    collections::{HashMap, VecDeque},
    io::{self, stdout, Write},
};
use syn::parse::Nothing;

use crate::view::{
    bottombar::BottomBar, filetree::FileTree, mainview::MainView, menu::Menu, topbar::TopBar, View,
    ViewID,
};

pub struct Screen {
    focus: ViewID,
    id_cnt: u64,
    view_map: HashMap<ViewID, Box<dyn View>>,
    name_map: HashMap<String, ViewID>,
}

pub enum Op {
    Nothing,
    Shift(String),
    Resize(String, (i16, i16, i16, i16)),
}

pub struct Module {
    pub term: Term,
    pub file_mod: FileMod,
    pub settings: Settings,
    message: HashMap<String, VecDeque<String>>,
    operation: Vec<Op>,
}

impl Module {
    pub fn new(term: Term, file_mod: FileMod, settings: Settings) -> Module {
        Module {
            term,
            file_mod,
            settings,
            message: HashMap::new(),
            operation: Vec::new(),
        }
    }

    pub fn sendmsg(&mut self, to: String, content: String) {
        let msg_queue = self.message.get_mut(&to);
        match msg_queue {
            Some(queue) => {
                queue.push_back(content);
            }
            None => {
                let mut queue = VecDeque::new();
                queue.push_back(content);
                self.message.insert(to, queue);
            }
        }
    }

    pub fn recvmsg(&mut self, name: &String) -> Option<String> {
        let msg_queue = self.message.get_mut(name);
        match msg_queue {
            Some(queue) => queue.pop_front(),
            None => None,
        }
    }

    pub fn push_op(&mut self, op: Op) {
        self.operation.push(op);
    }
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            focus: 0,
            id_cnt: 1,
            view_map: HashMap::new(),
            name_map: HashMap::new(),
        }
    }

    pub fn init(&mut self, module: &mut Module) -> io::Result<()> {
        let main_view = MainView::new();
        let top_bar = TopBar::new();
        let bottom_bar = BottomBar::new();
        let file_tree = FileTree::new();
        let menu = Menu::new();

        module.settings.num_offset = 6;
        module.settings.is_show_num = true;

        self.register(Box::new(main_view));
        self.register(Box::new(top_bar));
        self.register(Box::new(bottom_bar));
        self.register(Box::new(file_tree));
        self.register(Box::new(menu));

        for (_, view) in self.view_map.iter_mut() {
            view.init(module);
        }

        self.focus = 1;

        Cursor::reset_csr();
        stdout().flush()?;
        Ok(())
    }

    fn clean(term: &Term) -> std::io::Result<()> {
        Cursor::reset_csr();
        print!("{}", " ".repeat(term.size()));
        Cursor::reset_csr();
        stdout().flush()?;
        Ok(())
    }

    fn register(&mut self, view: Box<dyn View>) {
        self.name_map
            .insert(view.get_name().to_string(), self.id_cnt);
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

    fn shift_to(&mut self, name: &String) {
        let id = self.name_map.get(name).unwrap();
        self.focus = *id;
    }

    pub fn interact(&mut self, module: &mut Module) -> io::Result<()> {
        Screen::clean(&module.term)?;

        let mut cls = true;
        loop {
            let ch = Getch::new();

            if cls {
                let main_view = self.view_map.get_mut(&self.focus).unwrap();
                main_view.update(module);

                for (id, view) in self.view_map.iter_mut() {
                    if *id != self.focus {
                        view.update(module);
                        view.draw(module)?;
                    }
                }

                let main_view = self.view_map.get_mut(&self.focus).unwrap();
                main_view.draw(module)?;
                main_view.set_cursor(module);
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
                Ok(Key::Ctrl('r')) => {
                    cls = false;
                    dbg!(&module.message);
                }
                Ok(Key::Ctrl('d')) => {
                    cls = false;
                    let con = &module.file_mod.curr().flatten();
                    dbg!(String::from_utf8_lossy(con));
                }
                Ok(Key::Ctrl('k')) => {
                    cls = false;
                    dbg!(&module.file_mod);
                }

                Ok(Key::Ctrl('s')) => {
                    module.file_mod.save()?;
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
                    main_view.matchar(module, key);
                }
                Err(e) => panic!("{}", e),
            }
            module.file_mod.update()?;

            while !module.operation.is_empty() {
                let op = module.operation.pop().unwrap_or(Op::Nothing);
                match &op {
                    Op::Nothing => (),
                    Op::Shift(name) => {
                        self.shift_to(name);
                    }
                    Op::Resize(name, size) => {
                        let (dx_s, dy_s, dx_e, dy_e) = *size;
                        let id = self.name_map.get(name).unwrap();
                        let view = self.view_map.get_mut(&id).unwrap();
                        view.resize(dx_s, dy_s, dx_e, dy_e);
                    }
                }
            }
        }

        Screen::clean(&module.term)?;
        Ok(())
    }
}
