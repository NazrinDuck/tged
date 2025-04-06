use std::arch::asm;

/// 存储目前的终端长宽
pub struct Term {
    pub height: u16,
    pub width: u16,
}

/// 参考: https://www.man7.org/linux/man-pages/man2/TIOCGWINSZ.2const.html
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

    /// 内联汇编实现系统调用，得到窗口尺寸
    pub fn init(&mut self) {
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
        self.height = winsize.ws_col;
        self.width = winsize.ws_row;
    }
}
