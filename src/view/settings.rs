use crate::color::Color;

#[derive(Default, Debug)]
pub struct Settings {
    pub fcolor: Color,
    pub bcolor: Color,
    pub num_offset: u16,
    pub is_show_num: bool,
}

impl Settings {
    pub fn init(&mut self) {
        self.num_offset = 5;
        self.is_show_num = true;
    }
}
