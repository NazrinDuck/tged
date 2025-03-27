use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{self, File, Metadata, OpenOptions};
use std::io::{self, BufReader, Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use clap::builder::IntoResettable;

type FileID = usize;

#[derive(Debug)]
pub struct FileBuf {
    name: String,
    file: File,
    dirty: bool,
    pos: (usize, usize),
    scroll: usize,
    pathbuf: PathBuf,
    metadata: Metadata,
    buffer: Vec<u8>,
}

impl FileBuf {
    fn new(name: String) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new(&name);
        let file: File;
        let metadata: Metadata;
        let mut open_options = OpenOptions::new();

        if !path.try_exists()? {
            let open_options = open_options.read(true).write(true).create(true);
            file = open_options.open(path)?;
            metadata = fs::metadata(path)?;
        } else {
            metadata = fs::metadata(path)?;
            let open_options = if metadata.permissions().readonly() {
                open_options.read(true).write(false)
            } else {
                open_options.read(true).write(true)
            };
            file = open_options.open(path)?;
        };

        let pathbuf = fs::canonicalize(path)?;
        let mut buffer = Vec::new();

        let mut buf_reader = BufReader::new(&file);
        buf_reader.read_to_end(&mut buffer)?;

        Ok(FileBuf {
            name,
            file,
            dirty: false,
            pos: (0, 0),
            scroll: 0,
            pathbuf,
            metadata,
            buffer,
        })
    }

    pub fn name(&self) -> &str {
        self.pathbuf.file_name().unwrap().to_str().unwrap()
    }

    fn sync(&mut self) -> io::Result<()> {
        self.file.rewind()?;
        self.file.write_all(&self.buffer)?;
        Ok(())
    }

    fn content(&self) -> &Vec<u8> {
        &self.buffer
    }

    fn write(&mut self, content: String) {
        self.buffer = content.into();
    }

    fn save(&mut self, content: String) -> io::Result<()> {
        self.write(content);
        self.sync()?;
        Ok(())
    }

    fn save_status(&mut self, pos: (usize, usize), scroll: usize) {
        self.pos = pos;
        self.scroll = scroll;
    }

    fn get_status(&self) -> (usize, usize, usize) {
        let (x, y) = self.pos;
        (x, y, self.scroll)
    }

    fn file_size(&self) -> u64 {
        self.metadata.len()
    }
}

#[derive(Debug)]
pub struct FileMod {
    file_map: HashMap<FileID, FileBuf>,
    file_cnt: FileID,
    curr_file: Option<FileID>,
    curr_dir: PathBuf,
}

impl FileMod {
    pub fn new() -> Self {
        let curr_dir = fs::canonicalize(PathBuf::from(".")).unwrap();
        FileMod {
            file_map: HashMap::new(),
            file_cnt: 1,
            curr_file: None,
            curr_dir,
        }
    }

    pub fn to_vec(&self) -> Vec<(&FileID, &FileBuf)> {
        self.file_map.iter().collect::<Vec<(&FileID, &FileBuf)>>()
    }

    pub fn curr_id(&self) -> FileID {
        self.curr_file.unwrap_or(0)
    }

    pub fn curr_dir(&self) -> &PathBuf {
        &self.curr_dir
    }

    pub fn insert(&mut self, file_buf: FileBuf) {
        let cnt = self.file_cnt;
        self.file_map.insert(cnt, file_buf);
        self.file_cnt += 1;
    }

    pub fn mut_curr(&mut self) -> &mut FileBuf {
        let curr = self.curr_file.unwrap();
        self.file_map.get_mut(&curr).unwrap()
    }

    pub fn curr(&self) -> &FileBuf {
        let curr = self.curr_file.unwrap();
        self.file_map.get(&curr).unwrap()
    }

    pub fn get_content(&self) -> &Vec<u8> {
        self.curr().content()
    }

    pub fn save(&mut self, content: String) -> io::Result<()> {
        self.mut_curr().save(content)?;
        Ok(())
    }

    pub fn write(&mut self, content: String) {
        self.mut_curr().write(content);
    }

    pub fn name(&self) -> &String {
        &self.curr().name
    }

    pub fn names(&self) -> Vec<&String> {
        self.file_map
            .values()
            .map(|file| &file.name)
            .collect::<Vec<&String>>()
    }

    pub fn shift(&mut self, pos: (usize, usize), scroll: usize) -> (usize, usize, usize) {
        self.mut_curr().save_status(pos, scroll);
        let mut curr_file = self.curr_file.unwrap();
        curr_file = curr_file % (self.file_cnt - 1) + 1;
        self.curr_file = Some(curr_file);
        self.file_map.get(&curr_file).unwrap().get_status()
    }
}

impl From<Vec<String>> for FileMod {
    fn from(names: Vec<String>) -> Self {
        let curr_dir = fs::canonicalize(PathBuf::from(".")).unwrap();
        let mut file_cnt: FileID = 1;
        let mut file_map = HashMap::new();
        for name in names {
            let file_buf = FileBuf::new(name).unwrap();
            file_map.insert(file_cnt, file_buf);
            file_cnt += 1;
        }

        FileMod {
            file_map,
            file_cnt,
            curr_file: Some(1),
            curr_dir,
        }
    }
}
