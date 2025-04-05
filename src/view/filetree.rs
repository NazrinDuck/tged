use crate::prelude::*;

use getch_rs::Key;
use std::{
    fs::{DirEntry, Metadata},
    io::{self, Write},
    path::PathBuf,
};

macro_rules! impl_icon_color {
    (
        $(
            $extension: expr, $icon: expr, $color: expr,
        )*
    ) => {
        #[inline]
        fn match_icon(string: Option<&str>) -> String {
            match string {
                $(
                Some($extension) => $icon.fclr_head(&Color::from($color)),
                )*
                _ => "".to_string(),
            }
        }
    };
}

impl_icon_color! {
    // #dea584
    "rs"  , "", 0xdea584,
    // #51a0cf
    "lua" , "", 0x51a0cf,
    // #a074c4
    "hs"  , "", 0xa074c4,
    // #599eff
    "c"   , "", 0x599eff,
    // #f34b7d
    "cpp" , "", 0xf34b7d,
    // #ffbc03
    "py"  , "", 0xffbc03,
    // #9f0500
    "out" , "", 0x9f0500,
    // #0091bd
    "S"   , "", 0x0091bd,
    "asm" , "", 0x0091bd,
    // #e44d26
    "html", "", 0xe44d26,
    // #a074c4
    "php" , "", 0xa074c4,
    // #42a5f5
    "css" , "", 0x42a5f5,
    // #cc3e44
    "java", "", 0xcc3e44,
    // #cbcb41
    "js"  , "", 0xcbcb41,
    // #cbcb41
    "json", "", 0xcbcb41,
    // #9c4221
    "toml", "", 0x9c4221,
    // #bbbbbb
    "lock", "", 0xbbbbbb,
    // #89e051
    "txt" , "󰈙", 0x89e051,
}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    color: (Color, Color),
    dir_items: Vec<DirItem>,
    path: PathBuf,
    is_show: bool,
}

impl Dir {
    fn new(name: String, path: PathBuf, color: (Color, Color)) -> Self {
        Dir {
            name,
            color,
            dir_items: Vec::new(),
            path,
            is_show: false,
        }
    }

    fn len(&self) -> usize {
        if self.is_show {
            self.dir_items
                .iter()
                .fold(1usize, |sum, item| sum + item.len())
        } else {
            1
        }
    }

    fn open(&mut self) {
        let (bclr, fclr) = &self.color;
        self.is_show = true;
        let new_dir_name = format!(
            "{}{} {}",
            bclr.bclr_head(),
            fclr.fclr_head(),
            self.path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .fcolor(fclr)
        );
        self.name = new_dir_name;
        self.dir_items = read_dir_item(&self.path, bclr, fclr);
        self.is_show = true;
    }

    fn close(&mut self) {
        let (bclr, fclr) = &self.color;
        let new_dir_name = format!(
            "{}{} {}",
            bclr.bclr_head(),
            fclr.fclr_head(),
            self.path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .fcolor(fclr)
        );
        self.name = new_dir_name;
        self.is_show = false;
    }

    fn faltten(&self, bclr: &Color, depth: usize) -> Vec<String> {
        let mut flat = Vec::new();
        for item in &self.dir_items {
            let head = format!("{}   {}", bclr.bclr_head(), " ".repeat(depth * 2));
            match item {
                DirItem::Dir(dir) => {
                    flat.push(head + &dir.name.clone());
                    if dir.is_show {
                        let mut recu = dir.faltten(bclr, depth + 1);
                        flat.append(&mut recu);
                    }
                }
                DirItem::File(name, _) => {
                    flat.push(head + &name.clone());
                }
            }
        }
        flat
    }
}

fn read_dir_item(path: &PathBuf, bclr: &Color, fclr: &Color) -> Vec<DirItem> {
    let mut dir_items = Vec::new();
    for entry in std::fs::read_dir(path).unwrap() {
        let dir = entry.unwrap();
        let dir_item = get_item(dir, bclr, fclr);
        dir_items.push(dir_item);
    }
    dir_items.sort_unstable_by_key(|file| if let DirItem::Dir(_) = file { 0 } else { 1 });
    dir_items
}

fn get_item(dir: DirEntry, bclr: &Color, fclr: &Color) -> DirItem {
    let file_type = dir.file_type().unwrap();
    let icon = if file_type.is_dir() {
        "".to_string().fclr_head(fclr)
    } else if file_type.is_file() {
        if let Some(os_str) = dir.path().extension() {
            match_icon(os_str.to_str())
        } else {
            "".to_string()
        }
    } else {
        "?".to_string()
    };
    let dir_name = format!(
        "{}{icon} {}",
        bclr.bclr_head(),
        dir.file_name().into_string().unwrap().fcolor(fclr)
    );

    let dir_path = dir.path().clone();
    if file_type.is_dir() {
        DirItem::Dir(Dir::new(dir_name, dir_path, (bclr.clone(), fclr.clone())))
    } else {
        DirItem::File(dir_name, dir_path)
    }
}

#[derive(Debug, Clone)]
enum DirItem {
    Dir(Dir),
    File(String, PathBuf),
}

impl DirItem {
    fn len(&self) -> usize {
        if let DirItem::Dir(dir) = self {
            dir.len()
        } else {
            1usize
        }
    }

    /*
    fn is_exist(&self, path: &PathBuf) -> bool {
        match self {
            DirItem::Dir(dir) => {
                if *path == dir.path {
                    true
                } else {
                    for item in &dir.dir_items {
                        if item.is_exist(path) {
                            return true;
                        };
                    }
                    false
                }
            }
            DirItem::File(_, pathbuf) => path == pathbuf,
        }
    }
    */

    fn search_mut_dir(&mut self, path: &PathBuf) -> Option<&mut Dir> {
        if let DirItem::Dir(dir) = self {
            if *path == dir.path {
                Some(dir)
            } else {
                for item in dir.dir_items.iter_mut() {
                    let res = item.search_mut_dir(path);
                    if res.is_some() {
                        return res;
                    }
                }
                None
            }
        } else {
            None
        }
    }
}

#[view("FileTree")]
#[start=(1, 2)]
#[end=(26, -2)]
pub struct FileTree {
    dir: String,
    path: PathBuf,
    dir_items: Vec<DirItem>,
    flat: Vec<String>,
    metadata: Option<Metadata>,
    curr_line: usize,
    scroll: usize,
}

impl View for FileTree {
    fn init(&mut self, module: &mut Module) {
        let (file_mod, settings) = (&mut module.file_mod, &mut module.settings);

        let (bclr, fclr) = (
            &settings.theme.normal_bclr.darken(0x4),
            &settings.theme.stress_fclr,
        );
        let curr_dir = file_mod.curr_dir();
        self.path = curr_dir.to_path_buf();
        self.bcolor = bclr.clone();
        self.fcolor = fclr.clone();
        self.metadata = Some(self.path.metadata().unwrap());
        let curr_dir = curr_dir.to_str().unwrap().to_string();
        /*
        curr_dir.truncate(max as usize);
        self.dir = curr_dir.color(bclr, fclr).bold();
        */
        self.dir = curr_dir;
        self.dir_items = read_dir_item(&self.path, bclr, fclr);
    }

    fn update(&mut self, _: &mut Module) {
        let metadata = self.path.metadata().unwrap();
        let prev_metadata = self.metadata.as_ref().unwrap();
        let bclr = &self.bcolor;

        if metadata.modified().unwrap() != prev_metadata.modified().unwrap() {
            let (bclr, fclr) = (&self.bcolor, &self.fcolor);
            /*
            for entry in std::fs::read_dir(&self.path).unwrap() {
                let dir = entry.unwrap();
                let path = dir.path();
                for item in &self.dir_items {
                    if !item.is_exist(&path) {
                        let new = get_item(dir, bclr, fclr);
                        self.dir_items.push(new);
                        break;
                    }
                }
            }
            */
            self.dir_items = read_dir_item(&self.path, bclr, fclr);
            self.curr_line = 0;
            self.metadata = Some(metadata);
        };

        let flat = self.dir_items.iter().fold(Vec::new(), |init, item| {
            let mut flat = init;
            let head = format!("{}   ", bclr.bclr_head());
            match &item {
                DirItem::Dir(dir) => {
                    flat.push(head + &dir.name.clone());
                    if dir.is_show {
                        let mut recu = dir.faltten(bclr, 1);
                        flat.append(&mut recu);
                    }
                }
                DirItem::File(name, _) => {
                    flat.push(head + &name.clone());
                }
            }
            flat
        });
        self.flat = flat;
    }
    fn matchar(&mut self, module: &mut Module, key: getch_rs::Key) {
        let term = &module.term;
        match key {
            Key::Char('\r') => {
                self.enter(module);
            }
            Key::Delete => {}
            Key::Up => {
                self.up();
            }

            Key::Down => {
                self.down(term);
            }

            _ => (),
        }
    }
    fn set_cursor(&self, module: &mut Module) {
        let term = &module.term;
        let (csr_x, mut csr_y): (u16, u16) = self.get_start(term);
        csr_y += (self.curr_line - self.scroll) as u16 + 1;

        Cursor::set_csr(csr_x, csr_y);
    }
    fn draw(&self, module: &mut Module) -> std::io::Result<()> {
        let term = &module.term;
        let (bclr, fclr) = (&self.bcolor, &self.fcolor);
        self.refresh(term);
        let (x, y) = self.get_start(term);
        let (x_e, y_e) = self.get_end(term);

        let max_height = y_e - y;
        let max = x_e - x;

        Cursor::set_csr(x, y);

        let mut dir = self.dir.clone();
        dir.truncate(max as usize);

        println!("{}", dir.color(bclr, fclr));

        let mut height_cnt = 1;
        for line in self.flat.iter().skip(self.scroll) {
            print!("{}", line);
            height_cnt += 1;
            if height_cnt > max_height {
                break;
            }
            Cursor::csr_nextline();
        }

        io::stdout().flush()?;
        Ok(())
    }
}

impl FileTree {
    pub fn len(&self) -> usize {
        self.dir_items.iter().fold(0, |sum, item| sum + item.len())
    }

    #[inline]
    pub fn enter(&mut self, module: &mut Module) {
        let file_mod = &mut module.file_mod;
        let curr_line = self.curr_line;
        let flat = flatten(&self.dir_items);
        let path = flat.get(curr_line).unwrap();
        if path.is_dir() {
            for item in self.dir_items.iter_mut() {
                let res = item.search_mut_dir(path);
                if let Some(dir) = res {
                    if dir.is_show {
                        dir.close();
                    } else {
                        dir.open();
                    }
                    break;
                }
            }
        } else {
            let latest = file_mod.insert_from_path(path);
            module.sendmsg(String::from("MainView"), latest.to_string());
            module.push_op(Op::Shift(String::from("MainView")));
        }
    }

    #[inline]
    pub fn up(&mut self) {
        let line = self.curr_line;

        if line > 0 {
            self.line_dec();
        }
    }

    #[inline]
    pub fn down(&mut self, term: &Term) {
        let line = self.curr_line;

        if line < self.len() - 1 {
            self.line_inc(term);
        }
    }

    #[inline]
    fn line_inc(&mut self, term: &Term) {
        let pre_all_lines = self.curr_line - self.scroll;

        let height = (self.end.1.unwrap(term.height) - self.start.1.unwrap(term.height)) as usize;

        if pre_all_lines + 4 > height {
            self.scroll += 1;
        }
        self.curr_line += 1;
    }

    #[inline]
    fn line_dec(&mut self) {
        if self.scroll != 0 {
            let pre_all_lines = self.curr_line - self.scroll;

            if pre_all_lines < 4 {
                self.scroll -= 1;
            }
        }
        self.curr_line -= 1;
    }
}

#[inline]
fn flatten(dir_items: &[DirItem]) -> Vec<PathBuf> {
    let mut flat = Vec::new();
    dir_items.iter().for_each(|item| match item {
        DirItem::Dir(dir) => {
            flat.push(dir.path.clone());
            if dir.is_show {
                let mut recu = flatten(&dir.dir_items);
                flat.append(&mut recu);
            }
        }
        DirItem::File(_, path) => flat.push(path.clone()),
    });
    flat
}
