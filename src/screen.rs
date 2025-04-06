use crate::{
    color::END,
    file::FileMod,
    settings::Settings,
    terminal::{cursor::Cursor, term::Term},
    view::{help::Help, msgbox::MsgBox, Position},
};
use crossbeam_channel::Receiver;
use getch_rs::Key;
use std::{
    collections::{HashMap, VecDeque},
    io::{self, stdout, Write},
};

use crate::view::{
    bottombar::BottomBar, filetree::FileTree, mainview::MainView, menu::Menu, topbar::TopBar, View,
    ViewID,
};

/// 处理核心逻辑
///
/// <F1>～<F5>按键被保留作固定功能
/// <F1>: 打开帮助
/// <F2>: 聚焦至主视图
/// <F3>: 聚焦至文件树
/// <F4>: 聚焦至菜单
/// <F5>: 顺序切换视图
pub struct Screen {
    focus: ViewID,
    id_cnt: u64,
    view_map: HashMap<ViewID, Box<dyn View>>,
    name_map: HashMap<String, ViewID>,
}

/// 操作枚举
pub enum Op {
    Nothing,
    Shift(String),
    Resize(String, (i16, i16, i16, i16)),
    Quit,
}

/// 集成模块，提供各种信息以及视图间通信功能
pub struct Module {
    pub term: Term,
    pub file_mod: FileMod,
    pub settings: Settings,
    pub curr_view: String,
    message: HashMap<String, VecDeque<String>>,
    operation: Vec<Op>,
    /// 键盘事件接受管道
    key_recv: Receiver<Key>,
}

impl Module {
    pub fn new(
        term: Term,
        file_mod: FileMod,
        settings: Settings,
        key_recv: Receiver<Key>,
    ) -> Module {
        Module {
            term,
            file_mod,
            settings,
            curr_view: String::new(),
            message: HashMap::new(),
            operation: Vec::new(),
            key_recv,
        }
    }

    /// 向`to`发送信息`content`
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

    /// 从`name`处接受信息
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

    pub fn key_channel(&self) -> Receiver<Key> {
        self.key_recv.clone()
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
        let help = Help::new();

        module.settings.num_offset = 6;
        module.settings.is_show_num = true;
        module.curr_view = main_view.get_name().clone();

        self.register(Box::new(main_view));
        self.register(Box::new(top_bar));
        self.register(Box::new(bottom_bar));
        self.register(Box::new(file_tree));
        self.register(Box::new(menu));
        self.register(Box::new(help));

        for (_, view) in self.view_map.iter_mut() {
            view.init(module);
        }

        self.focus = 1;

        Cursor::reset_csr();
        stdout().flush()?;
        Ok(())
    }

    pub fn clean(term: &Term) -> std::io::Result<()> {
        Cursor::reset_csr();
        print!("{}", END);
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

    fn shift(&mut self) -> &String {
        let mut new = self.focus % (self.id_cnt - 1) + 1;
        let mut view = self.view_map.get(&new).unwrap();
        while view.is_silent() || !view.is_show() {
            new = new % (self.id_cnt - 1) + 1;
            view = self.view_map.get(&new).unwrap();
        }
        self.focus = new;
        self.view_map.get(&new).unwrap().get_name()
    }

    fn shift_to<'a>(&mut self, name: &'a String) -> &'a String {
        let id = self.name_map.get(name).unwrap();
        self.focus = *id;
        name
    }

    pub fn update(&mut self, module: &mut Module) -> io::Result<()> {
        let main_view = self.view_map.get_mut(&self.focus).unwrap();
        main_view.update(module);

        for (id, view) in self.view_map.iter_mut() {
            if *id != self.focus {
                view.update(module);
                if view.is_show() {
                    view.draw(module)?;
                }
            }
        }

        let main_view = self.view_map.get_mut(&self.focus).unwrap();
        main_view.draw(module)?;
        main_view.set_cursor(module);

        stdout().flush()?;
        Ok(())
    }

    pub fn interact(&mut self, module: &mut Module, key: Key) -> io::Result<bool> {
        let main_view = self.view_map.get_mut(&self.focus).unwrap();
        match key {
            // press ESC to leave
            Key::Esc => {
                if !module.file_mod.is_all_saved() {
                    let ret = MsgBox::new()
                        .title("Save All?(y/n)")
                        .default_pos(module)
                        .wait::<String>(module)
                        .unwrap_or_default();
                    match &ret[..] {
                        "n" | "N" => (),
                        _ => module.file_mod.save_all()?,
                    };
                    return Ok(true);
                } else {
                    let ret = MsgBox::new()
                        .title("Quit?(y/n)")
                        .default_pos(module)
                        .wait::<String>(module)
                        .unwrap_or_default();
                    match &ret[..] {
                        "n" | "N" => (),
                        _ => return Ok(true),
                    };
                }
            }

            // reserve key F1 ~ F5 for fixed function
            Key::F(1) => {
                let help = String::from("Help");
                if module.curr_view == help {
                    module.curr_view = self.shift().clone();
                } else {
                    module.curr_view = self.shift_to(&help).clone();
                }
            }
            Key::F(2) => {
                let main = String::from("MainView");
                module.curr_view = self.shift_to(&main).clone();
            }

            Key::F(3) => {
                let file_tree = String::from("FileTree");
                module.curr_view = self.shift_to(&file_tree).clone();
            }

            Key::F(4) => {
                let menu = String::from("Menu");
                module.curr_view = self.shift_to(&menu).clone();
            }

            Key::F(5) => {
                if !main_view.is_lock() {
                    module.curr_view = self.shift().clone();
                }
            }

            // measure input key
            key => {
                main_view.matchar(module, key);
            }
        }
        module.file_mod.update()?;

        // 处理模块操作队列
        while !module.operation.is_empty() {
            let op = module.operation.pop().unwrap_or(Op::Nothing);
            match &op {
                Op::Nothing => (),
                Op::Shift(name) => {
                    module.curr_view = self.shift_to(name).clone();
                }
                Op::Resize(name, size) => {
                    let (dx_s, dy_s, dx_e, dy_e) = *size;
                    let id = self.name_map.get(name).unwrap();
                    let view = self.view_map.get_mut(id).unwrap();
                    view.resize(&module.term, dx_s, dy_s, dx_e, dy_e);
                }
                Op::Quit => return Ok(true),
            }
        }

        // 更新状态
        self.update(module)?;

        Ok(false)
    }
}
