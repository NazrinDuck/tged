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

#[derive(Debug)]
pub struct FileBuf {
    name: String,
    file: File,
    dirty: bool,
    pos: (usize, usize),
    scroll: usize,
    pathbuf: PathBuf,
    metadata: Metadata,
    content: Content,
    copies: Vec<u8>,
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
        let name = pathbuf.file_name().unwrap().to_str().unwrap().to_string();

        let mut buf_reader = BufReader::new(&file);
        let mut buf = String::new();
        buf_reader.read_to_string(&mut buf)?;

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
    pub fn file_size(&self) -> u64 {
        self.metadata.len()
    }

    #[inline]
    pub fn file_modified(&self) -> SystemTime {
        self.metadata.modified().unwrap()
    }

    fn sync(&mut self) -> io::Result<()> {
        self.file.rewind()?;
        let mut buf = String::new();
        let mut buf_reader = BufReader::new(&self.file);
        buf_reader.read_to_string(&mut buf)?;

        let content = buf
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let content = Rc::new(RefCell::new(content));
        let copies = buf.into_bytes();

        self.content = content;
        self.copies = copies;
        Ok(())
    }

    fn content(&self) -> &Content {
        &self.content
    }

    fn save(&mut self) -> io::Result<()> {
        if self.dirty {
            self.file.rewind()?;
            let content = self.flatten();
            self.file.write_all(&content)?;
            self.file.set_len(content.len() as u64)?;
            self.copies = content.clone();
            self.file.sync_all()?;
            self.metadata = fs::metadata(&self.pathbuf)?;
            self.dirty = false;
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
    pub fn new() -> Self {
        let curr_dir = fs::canonicalize(PathBuf::from(".")).unwrap();
        FileMod {
            file_map: HashMap::new(),
            file_cnt: 1,
            curr_file: None,
            curr_dir,
        }
    }

    #[inline]
    pub fn update(&mut self) -> io::Result<()> {
        for file in self.file_map.values_mut() {
            if file.metadata.modified()? != file.file.metadata().unwrap().modified()? {
                file.sync()?;
                //file.dirty = true;
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

#[inline]
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
