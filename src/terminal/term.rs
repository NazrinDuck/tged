use std::arch::asm;
use std::process::exit;

pub struct Term {
    pub height: u16,
    pub width: u16,
}

#[derive(Default, Debug)]
#[repr(C)]
struct WinSize {
    ws_col: u16,
    ws_row: u16,
    _ws_xpixel: u16,
    _ws_ypixel: u16,
}

impl Term {
    pub fn new() -> Self {
        Term {
            height: 0,
            width: 0,
        }
    }

    pub fn size(&self) -> usize {
        (self.height * self.width) as usize
    }

    pub fn get_term_size(&mut self) {
        let winsize: WinSize = Default::default();
        let mut res: i32;
        unsafe {
            asm!(
                "xor rdi, rdi", // STDIN_FILENO
                "mov rsi, 0x5413", // TIOCGWINSZ
                "mov rax, 16", // __NR_ioctl
                "syscall",
                in("rdx") &winsize,
                out("rax") res,
            );
        }
        if res < 0 {
            panic!("can't get tty size!");
        }
        //dbg!(&winsize);
        self.height = winsize.ws_col;
        self.width = winsize.ws_row;
    }
}
