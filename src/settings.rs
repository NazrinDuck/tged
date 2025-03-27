use crate::color::Color;

pub struct Theme {
    pub stress_fclr: Color,
    pub stress_bclr: Color,
    pub normal_fclr: Color,
    pub normal_bclr: Color,
    pub weak_fclr: Color,
    pub weak_bclr: Color,

    // base color
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,

    // bright color
    pub bright_black: Color,
    pub bright_red: Color,
    pub bright_green: Color,
    pub bright_yellow: Color,
    pub bright_blue: Color,
    pub bright_magenta: Color,
    pub bright_cyan: Color,
    pub bright_white: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            // #82aaff
            stress_fclr: Color::new(0x82, 0xaa, 0xff),
            // #2d3f76
            stress_bclr: Color::new(0x2d, 0x3f, 0x76),
            // #c8d3f5
            normal_fclr: Color::new(0xc8, 0xd3, 0xf5),
            // #222436
            normal_bclr: Color::new(0x22, 0x24, 0x36),
            // #545c7e
            weak_fclr: Color::new(0x54, 0x5c, 0x7e),
            // #2f334d
            weak_bclr: Color::new(0x2f, 0x33, 0x4d),
            // #1b1d2b
            black: Color::new(0x1b, 0x1d, 0x2b),
            // #ff757f
            red: Color::new(0xff, 0x75, 0x7f),
            // #c3e88d
            green: Color::new(0xc3, 0xe8, 0x8d),
            // #ffc777
            yellow: Color::new(0xff, 0xc7, 0x77),
            // #82aaff
            blue: Color::new(0x82, 0xaa, 0xff),
            // #c099ff
            magenta: Color::new(0xc0, 0x99, 0xff),
            // #86e1fc
            cyan: Color::new(0x86, 0xe1, 0xfc),
            // #828bb8
            white: Color::new(0x82, 0x8b, 0xb8),
            // #444a73
            bright_black: Color::new(0x44, 0x4a, 0x73),
            // #ff8d94
            bright_red: Color::new(0xff, 0x8d, 0x94),
            // #c7fb6d
            bright_green: Color::new(0xc7, 0xfb, 0x6d),
            // #ffd8ab
            bright_yellow: Color::new(0xff, 0xd8, 0xab),
            // #9ab8ff
            bright_blue: Color::new(0x9a, 0xb8, 0xff),
            // #caabff
            bright_magenta: Color::new(0xca, 0xab, 0xff),
            // #b2ebff
            bright_cyan: Color::new(0xb2, 0xeb, 0xff),
            // #c8d3f5
            bright_white: Color::new(0xc8, 0xd3, 0xf5),
        }
    }
}

#[derive(Default)]
pub struct Settings {
    pub theme: Theme,
    pub is_show_num: bool,
    pub num_offset: u16,
}
