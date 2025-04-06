/// MsgBox提供一个能输入的弹窗
///
/// 通过<Enter>来提交输入的内容，会返回输入的内容
/// 可以指定返回的类型
use crate::prelude::*;
use getch_rs::Key;
use std::str::FromStr;

#[view("MsgBox")]
#[start=(1, 1)]
#[end=(2, 2)]
pub struct MsgBox {
    title: String,
    input: String,
    input_idx: usize,
}

impl View for MsgBox {
    fn init(&mut self, module: &mut Module) {
        let settings = &module.settings;
        self.fcolor = settings.theme.normal_fclr.clone();
        self.bcolor = settings.theme.normal_bclr.clone();
    }
    fn update(&mut self, _: &mut Module) {}
    fn matchar(&mut self, module: &mut Module, key: getch_rs::Key) {
        let term = &module.term;
        let max = self.get_end(term).0 - self.get_start(term).0 - 2;
        match key {
            Key::Esc => {
                self.input = String::new();
                self.lock = false;
            }
            Key::Char('\r') => {
                self.lock = false;
            }
            Key::Char(ch) => {
                if self.input.len() < max as usize {
                    self.input.insert(self.input_idx, ch);
                    self.input_idx += 1;
                }
            }
            Key::Delete => {
                if !self.input.is_empty() {
                    self.input_idx -= 1;
                    self.input.remove(self.input_idx);
                }
            }

            Key::Left => {
                if self.input_idx > 0 {
                    self.input_idx -= 1;
                }
            }

            Key::Right => {
                if self.input_idx < self.input.len() {
                    self.input_idx += 1;
                }
            }

            _ => (),
        }
    }

    fn set_cursor(&self, module: &mut Module) {
        let term = &module.term;
        let (x, y) = self.get_start(term);
        let csr_x = x + 1 + self.input_idx as u16;
        Cursor::set_csr(csr_x, y + 1);
    }

    fn draw(&self, module: &mut Module) -> io::Result<()> {
        let term = &module.term;
        self.refresh(term);
        let (x, y) = self.get_start(term);
        let (x_e, y_e) = self.get_end(term);
        let max_x = (x_e - x) as usize;
        let mut max_y = (y_e - y) as usize;
        Cursor::set_csr(x, y);
        print!("{}{}", self.fcolor.fclr_head(), self.bcolor.bclr_head());
        println!("╭{:─^width$}╮", self.title, width = max_x - 2);

        Cursor::csr_setcol(x);
        print!("{}{}", self.fcolor.fclr_head(), self.bcolor.bclr_head());
        println!("│{:<width$}│", self.input, width = max_x - 2);
        max_y -= 1;

        while max_y > 2 {
            Cursor::csr_setcol(x);
            print!("{}{}", self.fcolor.fclr_head(), self.bcolor.bclr_head());
            println!("│{}│", " ".repeat(max_x - 2));
            max_y -= 1;
        }

        Cursor::csr_setcol(x);
        print!("{}{}", self.fcolor.fclr_head(), self.bcolor.bclr_head());
        println!("╰{}╯", "─".repeat(max_x - 2));
        io::stdout().flush()?;

        Ok(())
    }
}

#[allow(unused)]
impl MsgBox {
    pub fn pos(&mut self, start: (i16, i16), end: (i16, i16)) -> &mut Self {
        let (x_s, y_s) = start;
        let x_s = Pos::try_from(x_s).unwrap();
        let y_s = Pos::try_from(y_s).unwrap();
        let (x_e, y_e) = end;
        let x_e = Pos::try_from(x_e).unwrap();
        let y_e = Pos::try_from(y_e).unwrap();

        self.start = (x_s, y_s);
        self.end = (x_e, y_e);

        self
    }

    pub fn default_pos(&mut self, module: &mut Module) -> &mut Self {
        let (height, width) = (module.term.height as i16, module.term.width as i16);
        let length = (self.title.len() / 2) as i16 + 10;
        let start = (
            Pos::try_from(width / 2 - length).unwrap(),
            Pos::try_from(height / 2 - 1).unwrap(),
        );
        let end = (
            Pos::try_from(width / 2 + length).unwrap(),
            Pos::try_from(height / 2 + 2).unwrap(),
        );
        self.start = start;
        self.end = end;
        self
    }

    pub fn title(&mut self, content: &str) -> &mut Self {
        self.title = content.into();
        self
    }

    pub fn wait<T>(&mut self, module: &mut Module) -> Result<T, <T as FromStr>::Err>
    where
        T: FromStr,
    {
        let key_events = module.key_channel();
        self.lock = true;
        self.init(module);
        self.update(module);
        self.draw(module).unwrap();
        self.set_cursor(module);
        io::stdout().flush().unwrap();

        while self.lock {
            let key = key_events.recv().unwrap();
            self.matchar(module, key);

            self.update(module);
            self.draw(module).unwrap();
            self.set_cursor(module);
            io::stdout().flush().unwrap();
        }

        self.input.parse::<T>()
    }
}
