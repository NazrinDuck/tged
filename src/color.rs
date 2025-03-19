use std::error::Error;

pub const END: &str = "\x1b[0m";

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub trait Colorful {
    fn color(&self, bclr: &Color, fclr: &Color) -> String;
    fn fcolor(&self, clr: &Color) -> String;
    fn bcolor(&self, clr: &Color) -> String;
}

impl Colorful for String {
    fn color(&self, bclr: &Color, fclr: &Color) -> String {
        format!(
            "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m{self}{}",
            bclr.r, bclr.g, bclr.b, fclr.r, fclr.g, fclr.b, END
        )
    }

    fn bcolor(&self, clr: &Color) -> String {
        format!("\x1b[48;2;{};{};{}m{self}{}", clr.r, clr.g, clr.b, END)
    }

    fn fcolor(&self, clr: &Color) -> String {
        format!("\x1b[38;2;{};{};{}m{self}{}", clr.r, clr.g, clr.b, END)
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn from_hex(hex: u32) -> Result<Self, Box<dyn Error>> {
        let r: u8 = ((hex & 0xff0000) >> 16).try_into()?;
        let g: u8 = ((hex & 0xff00) >> 8).try_into()?;
        let b: u8 = (hex & 0xff).try_into()?;
        Ok(Color { r, g, b })
    }
}
