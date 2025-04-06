/// MainView为主视图
///
/// 通过方向键移动光标，键盘输入字符
///
/// - 键入<F6>顺序切换当前文件
/// - 键入<F7>逆序切换当前文件
/// - 键入<F8>根据输入切换文件
///
/// - 键入<Ctrl+s>保存当前文件，若没有名字则会有弹窗来输入
/// - 键入<Ctrl+f>开启查找模式，输入字符串后通过方向键来定位所有匹配项
/// - 再次键入<Ctrl+f>可以开启替换模式，输入要替换的内容并回车完成替换
///
/// - 键入<Alt+Left>/<Alt+Right>改变主视图大小
use crate::prelude::*;
use crate::MsgBox;

use super::SplitNAt;
use crate::file::Content;
use getch_rs::Key;
use std::rc::Rc;
use widestring::{utf16str, Utf16Str, Utf16String};

#[derive(Clone, Debug, Default)]
enum Mode {
    Search,
    #[default]
    Normal,
}

#[view("MainView")]
#[start=(26, 3)]
#[end=(-1, -2)]
pub struct MainView {
    //line number's color
    lnum_clr: Color,
    //line number's stressed color
    lnum_sclr: Color,
    curr_line: usize,
    curr_idx: usize,
    content: Content,
    scroll: usize,
    mode: Mode,
    search_stack: Vec<(usize, usize)>,
    search_str: String,
    search_idx: usize,
}

impl View for MainView {
    fn update(&mut self, module: &mut Module) {
        match self.mode {
            Mode::Search => {}
            Mode::Normal => {
                if let Some(msg) = module.recvmsg(&self.name) {
                    let id = msg.parse::<usize>().unwrap();
                    module.sendmsg(
                        String::from("Menu"),
                        format!("Change to File No.{}", id + 1),
                    );

                    let curr_pos = (self.curr_idx, self.curr_line);
                    let scroll = self.scroll;
                    let file_mod = &mut module.file_mod;
                    let new_status = file_mod.shift_to(id, curr_pos, scroll);
                    self.sync(file_mod, new_status).unwrap();
                }
            }
        }
    }
    fn matchar(&mut self, module: &mut Module, key: getch_rs::Key) {
        match self.mode {
            Mode::Normal => self.normal_mode(module, key),
            Mode::Search => self.search_mode(module, key),
        }
    }

    fn set_cursor(&self, module: &mut Module) {
        let (term, settings) = (&module.term, &mut module.settings);
        let (curr_line, idx) = (self.curr_line, self.curr_idx);
        let content = self.content.borrow();

        let max = self.get_vpos_max(term, settings);
        let (mut csr_x, mut csr_y): (u16, u16) = self.get_text_pos(term, settings);

        csr_x += (idx % max) as u16;

        for line in content.iter().take(curr_line).skip(self.scroll) {
            if line.is_empty() {
                csr_y += 1;
                continue;
            }

            let len = line.len();
            csr_y += (len / max) as u16 + if (len % max) == 0 { 0u16 } else { 1u16 };
        }

        if idx >= max {
            csr_y += (idx / max) as u16
        }

        Cursor::set_csr(csr_x, csr_y);
    }

    fn draw(&self, module: &mut Module) -> io::Result<()> {
        let (term, settings) = (&module.term, &mut module.settings);
        self.refresh(term);
        let content: Vec<Utf16String> = self
            .content
            .borrow()
            .iter()
            .map(|line| {
                line.to_string()
                    .replace("\r", "↵")
                    .replace("\t", "    ")
                    .into()
            })
            .collect();

        let (x_pos, y_pos) = self.get_pos(term);

        let height = self.end.1.unwrap(term.height);
        let width = self.end.0.unwrap(term.width);

        let is_show_num = settings.is_show_num;
        let line_num_offset = settings.num_offset;

        let mut line_num = self.scroll + 1;

        let max_line = width - x_pos - line_num_offset;
        let max_height = (height - y_pos) as usize;

        let (bclr, fclr) = (&self.bcolor, &self.fcolor);
        let (lnum_clr, lnum_sclr) = (&self.lnum_clr, &self.lnum_sclr);

        Cursor::set_csr(x_pos, y_pos);

        let mut height_cnt = 1;
        'out: for line in content.iter().skip(self.scroll) {
            let mut lines: Vec<Utf16String> = Vec::new();
            for (subline, cnt) in line.splitn_at(max_line as usize) {
                if is_show_num {
                    let mut number = if cnt == 0 {
                        format!(
                            "{:>width$}│",
                            line_num,
                            width = (line_num_offset - 1) as usize
                        )
                    } else {
                        " ".repeat((line_num_offset - 1) as usize) + "│"
                    };

                    let subline = if line_num == self.curr_line + 1 {
                        number = number
                            .fcolor(lnum_sclr)
                            .bold()
                            .bclr_head(&bclr.lighten(0x6));
                        let mut highlight = subline.clone();
                        highlight.push_str(&" ".repeat((max_line) as usize - subline.len()));

                        highlight.clr_head(&bclr.lighten(0x6), fclr)
                    } else {
                        number = number.fcolor(lnum_clr).bclr_head(bclr);
                        subline.clr_head(bclr, fclr)
                    };

                    lines.push(format!("{number}{subline}").into());
                } else {
                    lines.push(subline);
                }
            }

            for subline in lines {
                Cursor::csr_setcol(x_pos);
                print!("{}{}", subline, END);
                height_cnt += 1;
                if height_cnt > max_height {
                    break 'out;
                }
                Cursor::csr_nextline();
            }
            line_num += 1;
        }
        io::stdout().flush()?;
        Ok(())
    }

    fn init(&mut self, module: &mut Module) {
        let (file_mod, settings) = (&mut module.file_mod, &mut module.settings);
        self.content = Rc::clone(file_mod.get_content());

        let (bclr, fclr) = (&settings.theme.normal_bclr, &settings.theme.normal_fclr);
        let (lnum_clr, lnum_sclr) = (&settings.theme.weak_fclr, &settings.theme.yellow);
        self.bcolor = bclr.clone();
        self.fcolor = fclr.clone();
        self.lnum_clr = lnum_clr.clone();
        self.lnum_sclr = lnum_sclr.clone();
    }
}

impl MainView {
    pub fn sync(
        &mut self,
        file_mod: &mut FileMod,
        new_status: (usize, usize, usize),
    ) -> io::Result<()> {
        self.content = Rc::clone(file_mod.get_content());
        self.curr_idx = new_status.0;
        self.curr_line = new_status.1;
        self.scroll = new_status.2;
        Ok(())
    }

    #[inline]
    pub fn get_pos(&self, term: &Term) -> (u16, u16) {
        let (height, width) = (term.height, term.width);
        (self.start.0.unwrap(width), self.start.1.unwrap(height))
    }

    #[inline]
    pub fn get_text_pos(&self, term: &Term, settings: &Settings) -> (u16, u16) {
        let (height, width) = (term.height, term.width);
        (
            self.start.0.unwrap(width) + settings.num_offset,
            self.start.1.unwrap(height),
        )
    }

    #[inline]
    fn get_vpos_max(&self, term: &Term, settings: &Settings) -> usize {
        (self.end.0.unwrap(term.width) - self.get_text_pos(term, settings).0) as usize
    }

    #[inline]
    fn pre_all_lines(&self, term: &Term, settings: &Settings) -> usize {
        let (curr_line, idx) = (self.curr_line, self.curr_idx);
        let content = self.content.borrow();

        let mut line_cnt = 0;
        let max = self.get_vpos_max(term, settings);
        for line in content.iter().take(curr_line).skip(self.scroll) {
            if line.is_empty() {
                line_cnt += 1;
                continue;
            }

            let len = line.len();
            line_cnt += (len / max) + if (len % max) == 0 { 0 } else { 1 };
        }

        if idx >= max {
            line_cnt += idx / max
        }

        line_cnt
    }

    #[inline]
    fn line_inc(&mut self, term: &Term, settings: &Settings) {
        let pre_all_lines = self.pre_all_lines(term, settings);

        let height = (self.end.1.unwrap(term.height) - self.start.1.unwrap(term.height)) as usize;

        if pre_all_lines + 4 > height {
            self.scroll += 1;
        }
        self.curr_line += 1;
    }

    #[inline]
    fn line_dec(&mut self, term: &Term, settings: &Settings) {
        if self.scroll != 0 {
            let pre_all_lines = self.pre_all_lines(term, settings);

            if pre_all_lines < 4 {
                self.scroll -= 1;
            }
        }
        self.curr_line -= 1;
    }

    #[inline]
    pub fn push(&mut self, ch: char) {
        let mut content = self.content.borrow_mut();
        if content.is_empty() {
            content.push(Utf16String::new());
        }
        content[self.curr_line].insert(self.curr_idx, ch);
        self.curr_idx += 1;
    }

    #[inline]
    pub fn push_str(&mut self, string: &Utf16Str) {
        let mut content = self.content.borrow_mut();
        content[self.curr_line].insert_utfstr(self.curr_idx, string);
        self.curr_idx += string.len();
    }

    #[inline]
    pub fn push_line(&mut self, term: &Term, settings: &Settings) {
        let (line, idx) = (self.curr_line, self.curr_idx);
        let mut content = self.content.borrow_mut();

        let curr_line = content[line].clone();
        let (first, last) = curr_line.split_at(idx);
        content[line] = Utf16String::from(first);
        content.insert(line + 1, Utf16String::from(last));
        drop(content);

        //self.curr_line += 1;
        self.line_inc(term, settings);
        self.curr_idx = 0;
    }

    #[inline]
    pub fn delete(&mut self, term: &Term, settings: &Settings) {
        let (line, idx) = (self.curr_line, self.curr_idx);

        if idx == 0 {
            if line == 0 {
                return;
            }

            let content = self.content.borrow().clone();
            let mut content_mut = self.content.borrow_mut();

            self.curr_idx = content[line - 1].len();
            content_mut[line - 1].push_utfstr(&content[line]);

            content_mut.remove(line);
            drop(content_mut);

            self.line_dec(term, settings);
        } else {
            self.content.borrow_mut()[line].remove(idx - 1);
            self.curr_idx -= 1;
        }
    }

    #[inline]
    pub fn up(&mut self, term: &Term, settings: &Settings) {
        let (line, idx) = (self.curr_line, self.curr_idx);

        if line > 0 {
            let content = self.content.borrow();
            if idx > content[line - 1].len() {
                self.curr_idx = content[line - 1].len()
            };
            drop(content);
            self.line_dec(term, settings);
        }
    }

    #[inline]
    pub fn down(&mut self, term: &Term, settings: &Settings) {
        let (line, idx) = (self.curr_line, self.curr_idx);

        let content = self.content.borrow();
        if line < content.len() - 1 {
            if idx > content[line + 1].len() {
                self.curr_idx = content[line + 1].len()
            };
            drop(content);
            self.line_inc(term, settings);
        }
    }

    #[inline]
    pub fn left(&mut self) {
        if self.curr_idx > 0 {
            self.curr_idx -= 1;
        }
    }

    #[inline]
    pub fn right(&mut self) {
        let max = self.content.borrow()[self.curr_line].len();
        if self.curr_idx < max {
            self.curr_idx += 1;
        }
    }

    #[inline]
    pub fn end(&mut self) {
        let idx = self.content.borrow()[self.curr_line].len();
        self.curr_idx = idx;
    }

    #[inline]
    pub fn home(&mut self) {
        self.curr_idx = 0;
    }

    #[inline]
    pub fn set_pos(&mut self, pos: (usize, usize)) {
        self.curr_line = pos.0;
        self.curr_idx = pos.1;
    }

    pub fn search_mode(&mut self, module: &mut Module, key: getch_rs::Key) {
        let term = &module.term;
        match key {
            Key::Ctrl('f') => {
                let ret = MsgBox::new()
                    .title("Replace")
                    .default_pos(module)
                    .wait::<String>(module)
                    .unwrap_or_default();
                if !ret.is_empty() {
                    let mut content = self.content.borrow_mut();
                    let start = self.curr_idx;
                    let end = start + self.search_str.len();
                    content[self.curr_line]
                        .replace_range(start..end, &Utf16String::from(ret.clone()));

                    let search_str = self.search_str.clone();

                    let mut lines: usize = 0;
                    let match_str: Vec<_> = content
                        .iter()
                        .flat_map(move |line| {
                            lines += 1;
                            line.to_string()
                                .match_indices(&search_str)
                                .map(|pat| (lines - 1, pat.0))
                                .collect::<Vec<(usize, usize)>>()
                        })
                        .collect();
                    if !match_str.is_empty() {
                        self.search_stack = match_str;
                    } else {
                        self.mode = Mode::Normal;
                    }

                    module.sendmsg(
                        String::from("Menu"),
                        format!(
                            "Replace String \"{}\" at Index {} with \"{}\"",
                            self.search_str, self.search_idx, &ret
                        ),
                    );
                    return;
                }
            }
            Key::Char('\r') => {
                self.mode = Mode::Normal;
                module.sendmsg(String::from("Menu"), String::from("Return to Normal Mode"));
                return;
            }
            Key::Up | Key::PageUp | Key::Left => {
                let len = self.search_stack.len();
                if self.search_idx < 1 {
                    self.search_idx = len - 1;
                } else {
                    self.search_idx = (self.search_idx - 1) % self.search_stack.len();
                }
                self.set_pos(self.search_stack[self.search_idx]);
            }

            Key::Down | Key::PageDown | Key::Right => {
                self.search_idx = (self.search_idx + 1) % self.search_stack.len();
                self.set_pos(self.search_stack[self.search_idx]);
            }

            Key::Home => {
                self.set_pos(self.search_stack[0]);
            }

            Key::End => {
                let len = self.search_stack.len();
                self.set_pos(self.search_stack[len - 1]);
            }

            Key::Other(key) => {
                match key[..] {
                    // Alt(Left)
                    [27, 91, 49, 59, 51, 68] => {
                        self.resize(term, -1, 0, 0, 0);
                        module.push_op(Op::Resize(String::from("FileTree"), (0, 0, -1, 0)));
                        module.push_op(Op::Resize(String::from("TopBar"), (-1, 0, 0, 0)));
                    }
                    // Alt(Right)
                    [27, 91, 49, 59, 51, 67] => {
                        self.resize(term, 1, 0, 0, 0);
                        module.push_op(Op::Resize(String::from("FileTree"), (0, 0, 1, 0)));
                        module.push_op(Op::Resize(String::from("TopBar"), (1, 0, 0, 0)));
                    }
                    _ => (),
                };
            }

            _ => (),
        }
        module.sendmsg(
            String::from("Menu"),
            format!(
                "Search for String \"{}\" at Index {}",
                self.search_str, self.search_idx
            ),
        );
    }

    pub fn normal_mode(&mut self, module: &mut Module, key: getch_rs::Key) {
        let (term, file_mod, settings) = (&module.term, &mut module.file_mod, &mut module.settings);
        match key {
            Key::Ctrl('f') => {
                let ret = MsgBox::new()
                    .title("Search")
                    .default_pos(module)
                    .wait::<String>(module)
                    .unwrap_or_default();
                if !ret.is_empty() {
                    self.search_str = ret.clone();

                    let content = self.content.borrow();
                    let mut lines: usize = 0;
                    let match_str: Vec<_> = content
                        .iter()
                        .flat_map(move |line| {
                            lines += 1;
                            line.to_string()
                                .match_indices(&ret)
                                .map(|pat| (lines - 1, pat.0))
                                .collect::<Vec<(usize, usize)>>()
                        })
                        .collect();
                    drop(content);

                    if !match_str.is_empty() {
                        self.search_stack = match_str;
                        self.mode = Mode::Search;
                        self.set_pos(self.search_stack[self.search_idx]);
                        module.sendmsg(
                            String::from("Menu"),
                            format!(
                                "Search for String \"{}\" at Index {}",
                                self.search_str, self.search_idx
                            ),
                        );
                    } else {
                        module.sendmsg(
                            String::from("Menu"),
                            format!("Can't Find String \"{}\"", self.search_str),
                        );
                    }
                }
            }
            Key::Ctrl('s') => {
                let curr_file = module.file_mod.name();
                if curr_file.is_empty() {
                    let ret = MsgBox::new()
                        .title("Save as")
                        .default_pos(module)
                        .wait::<String>(module)
                        .unwrap_or_default();
                    if !ret.is_empty() {
                        module.sendmsg(String::from("Menu"), format!("File \"{ret}\" Saved"));
                        module.file_mod.set_name(ret);
                        module.file_mod.save().unwrap();
                    }
                } else {
                    module.sendmsg(String::from("Menu"), format!("File \"{curr_file}\" Saved"));
                    module.file_mod.save().unwrap();
                }
            }
            Key::Char('\r') => {
                self.push_line(term, settings);
            }
            Key::Char('\t') => {
                self.push_str(utf16str!("    "));
            }
            Key::Char(char) => {
                self.push(char);
            }
            Key::Delete => {
                self.delete(term, settings);
            }
            Key::Up => {
                self.up(term, settings);
            }
            Key::Down => {
                self.down(term, settings);
            }

            Key::Left => {
                self.left();
            }

            Key::Right => {
                self.right();
            }

            Key::Home => {
                self.home();
            }

            Key::End => {
                self.end();
            }

            Key::PageUp => {
                for _ in [0; 25] {
                    self.up(term, settings);
                }
            }

            Key::PageDown => {
                for _ in [0; 25] {
                    self.down(term, settings);
                }
            }

            Key::F(6) => {
                let curr_pos = (self.curr_idx, self.curr_line);
                let scroll = self.scroll;
                let new_status = file_mod.shift(curr_pos, scroll);
                self.sync(file_mod, new_status).unwrap();
            }

            Key::F(7) => {
                let curr_pos = (self.curr_idx, self.curr_line);
                let scroll = self.scroll;
                let new_status = file_mod.rshift(curr_pos, scroll);
                self.sync(file_mod, new_status).unwrap();
            }

            Key::F(8) => {
                let curr_pos = (self.curr_idx, self.curr_line);
                let scroll = self.scroll;
                let file_id = MsgBox::new()
                    .title("Input File Number")
                    .default_pos(module)
                    .wait::<usize>(module)
                    .unwrap_or_default();
                let curr_id = module.file_mod.curr_id();
                if file_id != 0 && file_id <= curr_id {
                    module.sendmsg(
                        String::from("Menu"),
                        format!("Change to File No.{}", file_id),
                    );

                    let new_status = module.file_mod.shift_to(file_id - 1, curr_pos, scroll);
                    self.sync(&mut module.file_mod, new_status).unwrap();
                }
            }

            Key::Other(key) => {
                match key[..] {
                    // Alt(Left)
                    [27, 91, 49, 59, 51, 68] => {
                        self.resize(term, -1, 0, 0, 0);
                        module.push_op(Op::Resize(String::from("FileTree"), (0, 0, -1, 0)));
                        module.push_op(Op::Resize(String::from("TopBar"), (-1, 0, 0, 0)));
                    }
                    // Alt(Right)
                    [27, 91, 49, 59, 51, 67] => {
                        self.resize(term, 1, 0, 0, 0);
                        module.push_op(Op::Resize(String::from("FileTree"), (0, 0, 1, 0)));
                        module.push_op(Op::Resize(String::from("TopBar"), (1, 0, 0, 0)));
                    }
                    _ => (),
                };
            }

            _ => (),
        }
    }
}
