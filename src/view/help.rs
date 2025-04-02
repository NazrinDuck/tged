use crate::prelude::*;

#[view("Help")]
#[start=(8, 4)]
#[end=(-8, -4)]
pub struct Help {
    content: String,
}

impl View for Help {
    fn init(&mut self, module: &mut Module) {
        let settings = &module.settings;
        self.fcolor = settings.theme.normal_fclr.clone();
        self.bcolor = settings.theme.normal_bclr.clone();
        self.content.push_str(
            r#" General Help

    1.  Move the Cursor:
        use the cursor keys
    
            [^]
        [<] [v] [>]

    2. Input the Content
        use keyboard input whatever you want

        the line will wrap if the line's length is over the max length

        delete: press key <Backspace> or <Delete>

    3. Shift the View
        press key <F1> to open this Help, and press again to close it

        press key <F5> to shift the view 

        the bottom bar will tell you the corrent view's name

    4. Save
        press key <Ctrl+s> to save the content

        if the content is changed and don't save yet, the topbar will remind you

    5. Terminal help
        input `tged --help` for more information
        "#,
        );
        self.show = false;
    }
    fn update(&mut self, module: &mut Module) {
        self.show = module.curr_view == self.name;
    }
    fn matchar(&mut self, _: &mut Module, _: getch_rs::Key) {}
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
        Cursor::set_csr(x, y);
        print!("{}{}", self.fcolor.fclr_head(), self.bcolor.bclr_head());
        println!("╭{}╮", "─".repeat(max_x - 2));

        for line in self.content.lines() {
            Cursor::csr_setcol(x);
            print!("{}{}", self.fcolor.fclr_head(), self.bcolor.bclr_head());
            println!("│{:<width$}│", line, width = max_x - 2);
            max_y -= 1;
        }

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
