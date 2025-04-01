use crate::prelude::*;

#[view("Menu")]
#[start=(1, 1)]
#[end=(-1, 2)]
pub struct Menu {
    search_clr: Color,
    content: String,
    input: String,
    input_idx: u16,
}

impl Default for Menu {
    fn default() -> Self {
        Menu::new()
    }
}

impl View for Menu {
    fn init(&mut self, module: &mut Module) {
        let settings = &module.settings;
        //let (bclr, fclr) = (&settings.theme.stress_bclr, &settings.theme.stress_fclr);
        self.bcolor = settings.theme.weak_bclr.clone();
        self.search_clr = settings.theme.stress_bclr.clone();
        self.fcolor = settings.theme.bright_white.clone();
    }
    fn update(&mut self, module: &mut Module) {
        let (bclr, fclr, sclr) = (&self.bcolor, &self.fcolor, &self.search_clr);
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
        let search_len = if max > 50 { max - 30 } else { 20 };

        let mut content = bclr.bclr_head().fclr_head(fclr);
        content += &"─".repeat(15);
        content += &start;
        content += &format!(
            "{:<width$}",
            "❯ Press <F1> for help",
            width = search_len as usize - 2
        );
        //content += &" ".repeat(search_len as usize - 2); // todo
        content += &end;
        content += &"─".repeat(15);
        content += END;

        self.content = content;
    }
    fn matchar(&mut self, module: &mut Module, _: getch_rs::Key) {}
    fn set_cursor(&self, module: &mut Module) {}
    fn draw(&self, module: &mut Module) -> std::io::Result<()> {
        let (term, settings) = (&module.term, &mut module.settings);
        self.refresh(term);
        let (x, y) = self.get_start(term);
        Cursor::set_csr(x, y);
        print!("{}", self.content);
        io::stdout().flush()?;
        Ok(())
    }
}
