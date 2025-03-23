pub enum CsrMove {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Cursor {
    x: u16,
    y: u16,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor { x: 0, y: 0 }
    }

    pub fn set(&mut self, pos: (u16, u16)) {
        self.x = pos.0;
        self.y = pos.1;
    }

    pub fn set_x(&mut self, x: u16) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: u16) {
        self.y = y;
    }

    pub fn get_x(&self) -> u16 {
        self.x
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }

    pub fn sync(&self) {
        Self::set_csr(self.y, self.x);
    }

    pub fn move_csr(&mut self, val: u16, csr_move: CsrMove) {
        match csr_move {
            CsrMove::Up => {
                if self.y > val {
                    self.y -= val
                }
            }
            CsrMove::Down => self.y += val,
            CsrMove::Left => {
                if self.x > val {
                    self.x -= val
                }
            }
            CsrMove::Right => self.x += val,
        };
    }

    #[inline]
    pub fn reset_csr() {
        print!("\x1b[1;1H")
    }

    #[inline]
    pub fn set_csr(x: u16, y: u16) {
        print!("\x1b[{y};{x}H")
    }

    #[inline]
    pub fn csr_nextline() {
        print!("\x1b[1E")
    }

    #[inline]
    pub fn csr_setcol(col: u16) {
        print!("\x1b[{col}G")
    }

    pub fn hide_csr() {
        print!("\x1b[?25l")
    }

    pub fn save_csr() {
        print!("\x1b[s")
    }

    pub fn restore_csr() {
        print!("\x1b[u")
    }

    pub fn show_csr() {
        print!("\x1b[?25h")
    }
}

/*
pub fn reset_term() {
    show_csr();
    print!("\x1b[0m");
    exit(0);
}
*/
