use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::{self, File, Metadata, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::{self, BufReader, Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::SystemTime;

pub(crate) type FileID = usize;
pub(crate) type Content = Rc<RefCell<Vec<String>>>;

impl From<&PathBuf> for FileBuf {
    fn from(value: &PathBuf) -> Self {
        let file: Option<File>;
        let metadata: Option<Metadata>;
        let mut open_options = OpenOptions::new();
        let mut buf = String::new();
        let pathbuf: PathBuf;

        if !value.try_exists().unwrap() {
            file = None;
            metadata = None;
            pathbuf = PathBuf::new();
        } else {
            let md = fs::metadata(value).unwrap();
            let open_options = if md.permissions().readonly() {
                open_options.read(true).write(false)
            } else {
                open_options.read(true).write(true)
            };

            let f = open_options.open(value).unwrap();

            let mut buf_reader = BufReader::new(&f);
            buf_reader.read_to_string(&mut buf).unwrap();

            pathbuf = fs::canonicalize(value).unwrap();
            metadata = Some(md);
            file = Some(f);
        };
        let name: String = pathbuf.file_name().unwrap().to_str().unwrap().to_string();

        let content = buf
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let content = Rc::new(RefCell::new(content));
        let copies = buf.into_bytes();

        FileBuf {
            name,
            file,
            dirty: false,
            pos: (0, 0),
            scroll: 0,
            pathbuf,
            metadata,
            content,
            copies,
        }
    }
}

#[derive(Debug)]
pub struct FileBuf {
    name: String,
    file: Option<File>,
    dirty: bool,
    pos: (usize, usize),
    scroll: usize,
    pathbuf: PathBuf,
    metadata: Option<Metadata>,
    content: Content,
    copies: Vec<u8>,
}

impl FileBuf {
    fn new(input: String) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new(&input);
        let file: Option<File>;
        let metadata: Option<Metadata>;
        let mut open_options = OpenOptions::new();
        let mut buf = String::new();
        let pathbuf: PathBuf;
        let name: String;

        if !path.try_exists()? {
            file = None;
            metadata = None;
            pathbuf = PathBuf::new();
            name = input;
        } else {
            let md = fs::metadata(path)?;
            let open_options = if md.permissions().readonly() {
                open_options.read(true).write(false)
            } else {
                open_options.read(true).write(true)
            };

            let f = open_options.open(path)?;

            let mut buf_reader = BufReader::new(&f);
            buf_reader.read_to_string(&mut buf)?;

            pathbuf = fs::canonicalize(path)?;
            metadata = Some(md);
            file = Some(f);
            name = pathbuf.file_name().unwrap().to_str().unwrap().to_string();
        };

        let content = buf
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let content = Rc::new(RefCell::new(content));
        let copies = buf.into_bytes();

        Ok(FileBuf {
            name,
            file,
            dirty: false,
            pos: (0, 0),
            scroll: 0,
            pathbuf,
            metadata,
            content,
            copies,
        })
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn pathbuf(&self) -> &PathBuf {
        &self.pathbuf
    }

    #[inline]
    pub fn try_open(&mut self) -> io::Result<()> {
        let file = OpenOptions::new().read(true).write(true).open(&self.name)?;
        self.metadata = Some(file.metadata()?);
        self.file = Some(file);
        self.pathbuf = fs::canonicalize(&self.name)?;
        self.name = self
            .pathbuf
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        Ok(())
    }

    #[inline]
    pub fn file_size(&self) -> u64 {
        match &self.metadata {
            Some(metadata) => metadata.len(),
            None => 0,
        }
    }

    #[inline]
    pub fn file_modified(&self) -> SystemTime {
        match &self.metadata {
            Some(metadata) => metadata.modified().unwrap(),
            None => SystemTime::now(),
        }
    }

    fn sync(&mut self) -> io::Result<()> {
        if let Some(ref mut file) = self.file {
            file.rewind()?;
            let mut buf = String::new();
            let mut buf_reader = BufReader::new(file);
            buf_reader.read_to_string(&mut buf)?;

            let content = buf
                .split('\n')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let content = Rc::new(RefCell::new(content));
            let copies = buf.into_bytes();

            self.content = content;
            self.copies = copies;
        }
        Ok(())
    }

    fn content(&self) -> &Content {
        &self.content
    }

    fn save(&mut self) -> io::Result<()> {
        let content = self.flatten();
        match self.file {
            Some(ref mut file) => {
                if self.dirty {
                    file.rewind()?;
                    file.write_all(&content)?;
                    file.set_len(content.len() as u64)?;
                    self.copies = content.clone();
                    file.sync_all()?;
                    self.metadata = Some(fs::metadata(&self.pathbuf)?);
                    self.dirty = false;
                }
            }
            None => {
                if self.name.is_empty() {
                    todo!()
                } else {
                    let mut open_options = OpenOptions::new();
                    let open_options = open_options.read(true).write(true).create(true);
                    let mut file = open_options.open(&self.name)?;
                    file.write_all(&content)?;
                    self.copies = content.clone();
                    file.sync_all()?;
                    self.pathbuf = fs::canonicalize(&self.name)?;
                    self.file = Some(file);
                    self.metadata = Some(fs::metadata(&self.name)?);
                    self.name = self
                        .pathbuf
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    self.dirty = false;
                }
            }
        }
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

    #[inline]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    #[inline]
    pub fn flatten(&self) -> Vec<u8> {
        self.content
            .borrow()
            .iter()
            .fold(String::new(), |init: String, line| {
                format!("{init}\n{line}")
            })[1..]
            .into()
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
    pub fn new(dir: PathBuf) -> Self {
        let curr_dir = fs::canonicalize(dir).unwrap();
        let mut file_map = HashMap::new();
        file_map.insert(1, FileBuf::new(String::new()).unwrap());
        FileMod {
            file_map,
            file_cnt: 2,
            curr_file: Some(1),
            curr_dir,
        }
    }

    pub fn set_dir(&mut self, dir: PathBuf) {
        self.curr_dir = fs::canonicalize(dir).unwrap();
    }

    #[inline]
    pub fn update(&mut self) -> io::Result<()> {
        for file in self.file_map.values_mut() {
            if let Some(f) = &file.file {
                if file.file_modified() != f.metadata().unwrap().modified()? {
                    file.sync()?;
                    //file.dirty = true;
                } else {
                    file.dirty = calculate_hash(&file.flatten()) != calculate_hash(&file.copies);
                }
            } else if file.pathbuf().try_exists()? {
                file.try_open()?;
                file.sync()?;
            } else {
                file.dirty = calculate_hash(&file.flatten()) != calculate_hash(&file.copies);
            }
        }
        Ok(())
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

    pub fn insert(&mut self, name: String) -> FileID {
        let cnt = self.file_cnt;
        let file_buf = FileBuf::new(name).unwrap();
        self.file_map.insert(cnt, file_buf);
        self.file_cnt += 1;
        self.file_cnt
    }

    pub fn insert_from_path(&mut self, path: &PathBuf) -> FileID {
        let cnt = self.file_cnt;
        let file_buf = FileBuf::from(path);
        let id = self.search(path);
        if id == 0 {
            self.file_map.insert(cnt, file_buf);
            self.file_cnt += 1;
            self.file_cnt
        } else {
            id - 1
        }
    }

    pub fn search(&mut self, path: &PathBuf) -> FileID {
        for (id, file_buf) in self.file_map.iter() {
            if file_buf.pathbuf() == path {
                return *id;
            }
        }
        0
    }

    pub fn mut_curr(&mut self) -> &mut FileBuf {
        let curr = self.curr_file.unwrap();
        self.file_map.get_mut(&curr).unwrap()
    }

    pub fn curr(&self) -> &FileBuf {
        let curr = self.curr_file.unwrap();
        self.file_map.get(&curr).unwrap()
    }

    pub fn get_content(&self) -> &Content {
        self.curr().content()
    }

    pub fn save(&mut self) -> io::Result<()> {
        self.mut_curr().save()?;
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

    #[inline]
    pub fn shift(&mut self, pos: (usize, usize), scroll: usize) -> (usize, usize, usize) {
        self.mut_curr().save_status(pos, scroll);
        let mut curr_file = self.curr_file.unwrap();
        curr_file = curr_file % (self.file_cnt - 1) + 1;
        self.curr_file = Some(curr_file);
        self.file_map.get(&curr_file).unwrap().get_status()
    }

    #[inline]
    pub fn shift_to(
        &mut self,
        file_id: FileID,
        pos: (usize, usize),
        scroll: usize,
    ) -> (usize, usize, usize) {
        self.mut_curr().save_status(pos, scroll);
        let id = file_id % (self.file_cnt - 1) + 1;
        self.curr_file = Some(id);
        self.file_map.get(&id).unwrap().get_status()
    }
}

impl From<Vec<String>> for FileMod {
    fn from(names: Vec<String>) -> Self {
        let curr_dir = PathBuf::new();
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

#[inline]
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
