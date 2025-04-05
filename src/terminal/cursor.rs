#[derive(Debug)]
pub struct Cursor();

#[allow(unused)]
impl Cursor {
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
