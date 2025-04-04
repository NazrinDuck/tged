use widestring::Utf16String;

pub const END: &str = "\x1b[0m";

#[derive(Debug, Default, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[allow(unused)]
pub trait Colorful {
    fn color(&self, bclr: &Color, fclr: &Color) -> String;
    fn fcolor(&self, clr: &Color) -> String;
    fn bcolor(&self, clr: &Color) -> String;
    fn fclr_head(&self, clr: &Color) -> String;
    fn bclr_head(&self, clr: &Color) -> String;
    fn bold(&self) -> String;
    fn bold_head(&self) -> String;
}

impl Colorful for Utf16String {
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

    fn bclr_head(&self, clr: &Color) -> String {
        format!("\x1b[48;2;{};{};{}m{self}", clr.r, clr.g, clr.b)
    }

    fn fclr_head(&self, clr: &Color) -> String {
        format!("\x1b[38;2;{};{};{}m{self}", clr.r, clr.g, clr.b)
    }

    fn bold(&self) -> String {
        format!("\x1b[1m{self}{}", END)
    }
    fn bold_head(&self) -> String {
        format!("\x1b[1m{self}")
    }
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

    fn bclr_head(&self, clr: &Color) -> String {
        format!("\x1b[48;2;{};{};{}m{self}", clr.r, clr.g, clr.b)
    }

    fn fclr_head(&self, clr: &Color) -> String {
        format!("\x1b[38;2;{};{};{}m{self}", clr.r, clr.g, clr.b)
    }

    fn bold(&self) -> String {
        format!("\x1b[1m{self}{}", END)
    }
    fn bold_head(&self) -> String {
        format!("\x1b[1m{self}")
    }
}

impl Colorful for str {
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

    fn bclr_head(&self, clr: &Color) -> String {
        format!("\x1b[48;2;{};{};{}m{self}", clr.r, clr.g, clr.b)
    }

    fn fclr_head(&self, clr: &Color) -> String {
        format!("\x1b[38;2;{};{};{}m{self}", clr.r, clr.g, clr.b)
    }

    fn bold(&self) -> String {
        format!("\x1b[1m{self}{}", END)
    }
    fn bold_head(&self) -> String {
        format!("\x1b[1m{self}")
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn fclr_head(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }

    pub fn bclr_head(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
    }

    pub fn darken(&self, val: u8) -> Color {
        let (r, g, b) = (self.r, self.g, self.b);
        Color {
            r: r - val,
            g: g - val,
            b: b - val,
        }
    }

    pub fn lighten(&self, val: u8) -> Color {
        let (r, g, b) = (self.r, self.g, self.b);
        Color {
            r: r + val,
            g: g + val,
            b: b + val,
        }
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        let r: u8 = ((value & 0xff0000) >> 16).try_into().unwrap();
        let g: u8 = ((value & 0xff00) >> 8).try_into().unwrap();
        let b: u8 = (value & 0xff).try_into().unwrap();
        Color { r, g, b }
    }
}
