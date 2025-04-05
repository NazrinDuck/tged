use crate::prelude::*;
use getch_rs::Key;

#[view("Help")]
#[start=(8, 4)]
#[end=(-8, -4)]
pub struct Help {
    content: Vec<String>,
    curr_page: usize,
}

impl View for Help {
    fn init(&mut self, module: &mut Module) {
        let settings = &module.settings;
        self.fcolor = settings.theme.normal_fclr.clone();
        self.bcolor = settings.theme.normal_bclr.clone();
        let page1 = r#" General Help

    1.  Move the Cursor:
        Use the cursor keys
    
            [^]
        [<] [v] [>]

    2. Input the Content
        Use keyboard input whatever you want

        The line will wrap if the line's length is over the max length

    3. How to Exit
        Press <Esc> to exit the editor
        Press <F1~5> to exit the help

    4. Save
        Press <Ctrl+s> to save the content

        If the content is changed and don't save yet, the topbar will remind you

    5. Search & Replace
        Press <Ctrl+f> to switch to search mode
        Press <Ctrl+f> again to replace the chosen string

    6. Terminal Help
        Input `tged --help` for more information"#;

        let page2 = r#" View Help

    1. <Fn> Keys
        Press <F2>: shift to Main View
        Press <F3>: shift to File Tree
        Press <F4>: shift to Menu
        Press <F5>: shift the view in order

    2. FileTree
        Press <Enter>: open the directory or open the file

    3. Menu
        See `Menu Help`"#;

        self.content.push(String::from(page1));
        self.content.push(String::from(page2));
        self.show = false;
    }
    fn update(&mut self, module: &mut Module) {
        self.show = module.curr_view == self.name;
    }
    fn matchar(&mut self, _: &mut Module, key: Key) {
        match key {
            Key::Left | Key::Up => {
                if self.curr_page > 0 {
                    self.curr_page -= 1;
                }
            }
            Key::Right | Key::Down => {
                if self.curr_page < self.content.len() - 1 {
                    self.curr_page += 1;
                }
            }
            Key::Home => {
                self.curr_page = 0;
            }
            Key::End => {
                self.curr_page = self.content.len() - 1;
            }
            _ => (),
        };
    }
    fn set_cursor(&self, module: &mut Module) {
        let term = &module.term;
        let (x, y) = self.get_start(term);
        Cursor::set_csr(x + 1, y + 1);
    }
    fn draw(&self, module: &mut Module) -> io::Result<()> {
        let term = &module.term;
        self.refresh(term);
        let (x, y) = self.get_start(term);
        let (x_e, y_e) = self.get_end(term);
        let max_x = (x_e - x) as usize;
        let mut max_y = (y_e - y) as usize;
        let content = &self.content[self.curr_page];
        Cursor::set_csr(x, y);
        print!("{}{}", self.fcolor.fclr_head(), self.bcolor.bclr_head());
        println!("╭{}╮", "─".repeat(max_x - 2));

        for line in content.lines() {
            Cursor::csr_setcol(x);
            print!("{}{}", self.fcolor.fclr_head(), self.bcolor.bclr_head());
            println!("│{:<width$}│", line, width = max_x - 2);
            max_y -= 1;
        }

        while max_y > 3 {
            Cursor::csr_setcol(x);
            print!("{}{}", self.fcolor.fclr_head(), self.bcolor.bclr_head());
            println!("│{}│", " ".repeat(max_x - 2));
            max_y -= 1;
        }

        let left_arrow = if self.curr_page != 0 { " <" } else { "  " };
        let right_arrow = if self.curr_page < self.content.len() - 1 {
            "> "
        } else {
            "  "
        };

        let page_num = format!("({}/{})", self.curr_page + 1, self.content.len());

        Cursor::csr_setcol(x);
        print!("{}{}", self.fcolor.fclr_head(), self.bcolor.bclr_head());
        println!(
            "│{left_arrow}{:^width$}{right_arrow}│",
            page_num,
            width = max_x - 6
        );

        Cursor::csr_setcol(x);
        print!("{}{}", self.fcolor.fclr_head(), self.bcolor.bclr_head());
        println!("╰{}╯", "─".repeat(max_x - 2));
        io::stdout().flush()?;
        Ok(())
    }
}
