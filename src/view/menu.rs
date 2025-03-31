use crate::prelude::*;

#[view("Menu")]
#[start=(1, 1)]
#[end=(-1, 2)]
pub struct Menu {
    content: String,
}

impl Default for Menu {
    fn default() -> Self {
        Menu::new()
    }
}

impl View for Menu {
    fn init(&mut self, _: &Term, _: &mut FileMod, settings: &Settings) {
        //let (bclr, fclr) = (&settings.theme.stress_bclr, &settings.theme.stress_fclr);
        self.bcolor = settings.theme.black.clone();
        self.fcolor = settings.theme.normal_fclr.clone();
    }
    fn update(&mut self, _: &Term, file_mod: &mut FileMod) {
        let (bclr, fclr) = (&self.bcolor, &self.fcolor);

        let mut content = String::new();

        content.push_str("aaabbb");

        self.content = content;
    }
    fn matchar(&mut self, _: &Term, _: &mut FileMod, settings: &Settings, _: getch_rs::Key) {}
    fn set_cursor(&self, _: &Term, settings: &Settings) {}
    fn draw(&self, term: &Term, settings: &Settings) -> std::io::Result<()> {
        self.refresh(term);
        let (x, y) = self.get_start(term);
        Cursor::set_csr(x, y);
        print!("{}", self.content);
        io::stdout().flush()?;
        Ok(())
    }
}
