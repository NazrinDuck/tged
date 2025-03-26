use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{self, File, Metadata, OpenOptions};
use std::io::{self, BufReader, Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;

type FileID = usize;

#[derive(Debug)]
pub struct FileBuf {
    name: String,
    file: File,
    pathbuf: PathBuf,
    metadata: Metadata,
    buffer: String,
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
        let mut buffer = String::new();

        let mut buf_reader = BufReader::new(&file);
        buf_reader.read_to_string(&mut buffer)?;

        Ok(FileBuf {
            name,
            file,
            pathbuf,
            metadata,
            buffer,
        })
    }

    fn sync(&mut self) {}

    fn content(&self) -> &String {
        &self.buffer
    }

    fn save(&mut self, content: String) -> io::Result<()> {
        self.file.rewind()?;
        self.file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn name(&self) -> &String {
        &self.name
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
}

impl FileMod {
    pub fn new() -> Self {
        FileMod {
            file_map: HashMap::new(),
            file_cnt: 0,
            curr_file: None,
        }
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

    pub fn get_content(&self) -> &String {
        self.curr().content()
    }

    pub fn save(&mut self, content: String) -> io::Result<()> {
        self.mut_curr().save(content)?;
        Ok(())
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

    pub fn shift(&mut self) {
        let mut curr_file = self.curr_file.unwrap();
        curr_file = (curr_file + 1) % self.file_cnt;
        self.curr_file = Some(curr_file);
    }
}

impl From<Vec<String>> for FileMod {
    fn from(names: Vec<String>) -> Self {
        let mut file_cnt: FileID = 0;
        let mut file_map = HashMap::new();
        for name in names {
            let file_buf = FileBuf::new(name).unwrap();
            file_map.insert(file_cnt, file_buf);
            file_cnt += 1;
        }

        FileMod {
            file_map,
            file_cnt,
            curr_file: Some(0),
        }
    }
}
