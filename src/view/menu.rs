/// Menu为最上方的状态条
///
/// 在被聚焦时可以输入命令，通过<Enter>来提交命令并尝试执行
/// 在其他时间会根据目前的状态显示一些信息
use crate::prelude::*;
use getch_rs::Key;

#[view("Menu")]
#[start=(1, 1)]
#[end=(-1, 2)]
pub struct Menu {
    offset: usize,
    search_bclr: Color,
    search_fclr: Color,
    content: String,
    input: String,
    input_idx: usize,
}

impl Default for Menu {
    fn default() -> Self {
        Menu::new()
    }
}

impl View for Menu {
    fn init(&mut self, module: &mut Module) {
        let settings = &module.settings;
        self.bcolor = settings.theme.weak_bclr.clone();
        self.search_bclr = settings.theme.stress_bclr.clone();
        self.search_fclr = settings.theme.bright_white.clone();
        self.fcolor = settings.theme.stress_fclr.clone();
        self.offset = 15;
    }
    fn update(&mut self, module: &mut Module) {
        let (bclr, fclr, sclr) = (&self.bcolor, &self.fcolor, &self.search_bclr);
        let offset = self.offset;
        let term = &module.term;
        let max = self.get_end(term).0 - self.get_start(term).0;
        let start = format!(
            "{}{}{}",
            sclr.fclr_head(),
            fclr.fclr_head(),
            sclr.bclr_head()
        );

        let end = format!(
            "{}{}{}",
            sclr.fclr_head(),
            bclr.bclr_head(),
            fclr.fclr_head()
        );

        let arrow = "❯ ".fclr_head(&module.settings.theme.yellow);
        let search_len = if max > 50 {
            max as usize - 2 * offset
        } else {
            20
        };

        let search_content = if module.curr_view == self.name {
            let recv = module.recvmsg(&self.name).unwrap_or_default();
            self.input_idx += recv.len();
            self.input.push_str(&recv);
            &self.input
        } else {
            &module
                .recvmsg(&self.name)
                .unwrap_or(String::from("Press <F1> for help"))
        };

        let mut content = bclr.bclr_head().fclr_head(fclr);
        content += &"─".repeat(offset);
        content += &start;
        content += &arrow;
        content += &format!(
            "{}{:<width$}",
            self.search_fclr.fclr_head(),
            search_content,
            width = search_len - 4
        );
        content += &end;
        content += &"─".repeat(offset);
        content += END;

        self.content = content;
    }
    fn matchar(&mut self, module: &mut Module, key: getch_rs::Key) {
        let term = &module.term;
        let offset = self.offset;
        let max = self.get_end(term).0 - self.get_start(term).0;
        match key {
            Key::Char('\r') => {
                self.exec(module);
            }
            Key::Char(ch) => {
                if self.input.len() < max as usize - 2 * offset - 4 {
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
        let offset = self.offset;
        let (x, y) = self.get_start(term);
        let csr_x = x + offset as u16 + 3 + self.input_idx as u16;
        Cursor::set_csr(csr_x, y);
    }
    fn draw(&self, module: &mut Module) -> std::io::Result<()> {
        let term = &module.term;
        self.refresh(term);
        let (x, y) = self.get_start(term);
        Cursor::set_csr(x, y);
        print!("{}", self.content);
        io::stdout().flush()?;
        Ok(())
    }
}

impl Menu {
    fn exec(&mut self, module: &mut Module) {
        let cmd = self.input.trim();
        match cmd {
            "quit" => {
                module.push_op(Op::Quit);
            }
            "save" => {
                let name = &module.file_mod.name();
                if !name.is_empty() {
                    module.sendmsg(String::from("Menu"), format!("File \"{name}\" Saved"));
                    module.file_mod.save().unwrap();
                } else {
                    module.sendmsg(String::from("Menu"), String::from("Use <Ctrl+s>"));
                }
            }
            other => {
                module.sendmsg(String::from("Menu"), format!("Unkonwn Command: `{other}`"));
            }
        }
        self.input.clear();
        self.input_idx = 0;
    }
}
