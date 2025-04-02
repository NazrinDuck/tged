use crate::prelude::*;

#[view("BottomBar")]
#[start=(1, -2)]
#[end=(-1, -1)]
#[silent]
pub struct BottomBar {
    bcolor_lv1: Color,
    fcolor_lv1: Color,
    bcolor_lv2: Color,
    fcolor_lv2: Color,

    content: String,
}

impl View for BottomBar {
    fn init(&mut self, module: &mut Module) {
        let settings = &module.settings;
        //let (bclr, fclr) = (&settings.theme.stress_bclr, &settings.theme.stress_fclr);
        self.bcolor_lv1 = settings.theme.stress_fclr.clone();
        self.fcolor_lv1 = settings.theme.black.clone();
        self.bcolor_lv2 = settings.theme.stress_bclr.clone();
        self.fcolor_lv2 = settings.theme.stress_fclr.clone();
        self.bcolor = settings.theme.black.clone();
        self.fcolor = settings.theme.normal_fclr.clone();
    }
    fn update(&mut self, module: &mut Module) {
        let file_mod = &mut module.file_mod;
        let (bclr_lv1, fclr_lv1) = (&self.bcolor_lv1, &self.fcolor_lv1);
        let (bclr_lv2, fclr_lv2) = (&self.bcolor_lv2, &self.fcolor_lv2);
        let (bclr, fclr) = (&self.bcolor, &self.fcolor);

        let divider_lv1 = format!(
            "{}{}{}",
            bclr_lv1.fclr_head(),
            bclr_lv2.bclr_head(),
            fclr_lv2.fclr_head()
        );
        let divider_lv2 = format!(
            "{}{}{}",
            bclr_lv2.fclr_head(),
            bclr.bclr_head(),
            fclr.fclr_head()
        );

        let mut content = String::new();
        let file_size = pretty_size(file_mod);

        let first_part = &module.curr_view;
        let second_part = if file_mod.curr().name().is_empty() {
            "[New File]"
        } else {
            file_mod.curr().name()
        };
        let third_part = &format!("size  {}", file_size);

        let bottom_bar = format!(
            "{}{} {first_part} {divider_lv1} {second_part} {divider_lv2} {third_part} {}",
            bclr_lv1.bclr_head(),
            fclr_lv1.fclr_head(),
            END,
        );

        content.push_str(&bottom_bar);

        self.content = content;
    }
    fn matchar(&mut self, module: &mut Module, _: getch_rs::Key) {}
    fn set_cursor(&self, module: &mut Module) {}
    fn draw(&self, module: &mut Module) -> std::io::Result<()> {
        let (term, _) = (&module.term, &mut module.settings);
        self.refresh(term);
        let (x, y) = self.get_start(term);
        Cursor::set_csr(x, y);
        print!("{}", self.content);
        io::stdout().flush()?;
        Ok(())
    }
}

impl BottomBar {
    pub fn push_str(&mut self, string: &str) {
        self.content.push_str(string);
    }
}

fn pretty_size(file_mod: &mut FileMod) -> String {
    let file_size = file_mod.curr().file_size();
    if file_size < 1000 {
        format!("{file_size}b")
    } else if file_size < 1000_0000 {
        format!("{:.2}kb", file_size as f64 / 1000.0f64)
    } else {
        format!("{:.2}mb", file_size as f64 / 1000_0000.0f64)
    }
}
