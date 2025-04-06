use crate::color::Color;

#[allow(unused)]
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

#[allow(unused)]
impl Default for Theme {
    /// 更改此项来改变主题
    fn default() -> Self {
        Theme::tokyonight()
    }
}

#[allow(unused)]
impl Theme {
    /// 参考：https://github.com/EdenEast/nightfox.nvim/
    pub fn duskfox() -> Self {
        Theme {
            // #569fba
            stress_fclr: Color::new(0x56, 0x9f, 0xba),
            // #433c59
            stress_bclr: Color::new(0x43, 0x3c, 0x59),
            // #e0def4
            normal_fclr: Color::new(0xe0, 0xde, 0xf4),
            // #232136
            normal_bclr: Color::new(0x23, 0x21, 0x36),
            // #444a73
            weak_fclr: Color::new(0x44, 0x4a, 0x73),
            // #393552
            weak_bclr: Color::new(0x2f, 0x33, 0x4d),
            // #393552
            black: Color::new(0x39, 0x35, 0x52),
            // #eb6f92
            red: Color::new(0xeb, 0x6f, 0x92),
            // #a3be8c
            green: Color::new(0xa3, 0xbe, 0x8c),
            // #f6c177
            yellow: Color::new(0xf6, 0xc1, 0x77),
            // #569fba
            blue: Color::new(0x56, 0x9f, 0xba),
            // #c4a7e7
            magenta: Color::new(0xc4, 0xa7, 0xe7),
            // #9ccfd8
            cyan: Color::new(0x9c, 0xcf, 0xd8),
            // #e0def4
            white: Color::new(0xe0, 0xde, 0xf4),
            // #444a73
            bright_black: Color::new(0x47, 0x40, 0x7d),
            // #f083a2
            bright_red: Color::new(0xf0, 0x83, 0xa2),
            // #b1d196
            bright_green: Color::new(0xb1, 0xd1, 0x96),
            // #f9cb8c
            bright_yellow: Color::new(0xf9, 0xcb, 0x8c),
            // #65b1cd
            bright_blue: Color::new(0x65, 0xb1, 0xcd),
            // #ccb1ed
            bright_magenta: Color::new(0xcc, 0xb1, 0xed),
            // #a6dae3
            bright_cyan: Color::new(0xa6, 0xda, 0xe3),
            // #e2e0f7
            bright_white: Color::new(0xe2, 0xe0, 0xf7),
        }
    }

    /// 参考：https://github.com/folke/tokyonight.nvim
    pub fn tokyonight() -> Self {
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
