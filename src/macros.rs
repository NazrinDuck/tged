#[macro_export]
macro_rules! rprintln {
    ($start: expr, $max_line: expr, $max_height: expr, $height_cnt: ident, $($arg:tt)*) => {
        {
            let __string = format!($($arg)*);
            let __length = __string.len();
            let __col = $start;
            for (subline, _) in __string.splitn_at($max_line as usize) {
                print!("\x1b[{__col}G");
                println!("{}", subline);
                $height_cnt += 1;
            }
        }
    };
}
