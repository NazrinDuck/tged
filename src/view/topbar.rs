/// TopBar为第二行的文件名
///
/// 显示所有已经打开的文件，并高亮目前的文件
use crate::prelude::*;

#[view("TopBar")]
#[silent]
#[start=(26, 2)]
#[end=(-1, 3)]
#[bcolor=(0x14, 0x14, 0x14)]
#[fcolor=(0xa0, 0x40, 0x40)]
pub struct TopBar {
    // stress color (used for chosen tab)
    scolor: Color,
    // dark color  (used for background)
    dcolor: Color,
    // dark color  (used for background)
    green: Color,
    content: String,
}

impl View for TopBar {
    fn init(&mut self, module: &mut Module) {
        let settings = &mut module.settings;
        let (bclr, fclr) = (&settings.theme.black, &settings.theme.weak_fclr);
        let (sclr, dclr) = (&settings.theme.yellow, &settings.theme.normal_bclr);
        self.bcolor = bclr.clone();
        self.fcolor = fclr.clone();
        self.scolor = sclr.clone();
        self.dcolor = dclr.clone();
        self.green = settings.theme.green.clone();
    }

    fn update(&mut self, module: &mut Module) {
        let file_mod = &mut module.file_mod;
        let (bclr, fclr) = (&self.bcolor, &self.fcolor);
        let dclr = &self.dcolor;
        let sclr = &self.scolor;
        let green = &self.green;
        let curr_id = file_mod.curr_id();
        let mut content = file_mod.to_vec();
        content.sort_unstable_by_key(|x| x.0);

        let content: String =
            content
                .into_iter()
                .fold(String::new(), |init: String, (id, file_buf)| {
                    let mut name = file_buf.name();
                    if name.is_empty() {
                        name = "[No Name]";
                    }
                    let dirty = if file_buf.is_dirty() {
                        "".fclr_head(green)
                    } else {
                        " ".to_string()
                    };
                    let banner = if *id == curr_id {
                        format!(
                            "{}{}{}",
                            "".color(bclr, dclr),
                            //"".color(bclr, dclr),
                            format!("  {id}. {}  {dirty} ", name).color(dclr, sclr),
                            "".color(bclr, dclr) //"".color(bclr, dclr)
                        )
                    } else {
                        let clr = &dclr.darken(0x4);
                        format!(
                            "{}{}{}",
                            "".color(bclr, clr),
                            //"".color(bclr, dclr),
                            format!("  {id}. {}  {dirty} ", name).color(clr, fclr),
                            "".color(bclr, clr) //"".color(bclr, dclr)
                        )
                    };
                    if init.is_empty() {
                        format!("{}  {banner}", bclr.bclr_head())
                    } else {
                        format!("{init}{}  {banner}", bclr.bclr_head())
                    }
                });
        self.content = content;
    }
    fn matchar(&mut self, _: &mut Module, _: getch_rs::Key) {}
    fn set_cursor(&self, _: &mut Module) {}
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
