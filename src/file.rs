use std::cell::RefCell;
use std::fs::Metadata;
use std::path::{Path, PathBuf};
use std::rc::Rc;

struct FileBuf {
    name: String,
    path: PathBuf,
    metadata: Metadata,
    buffer: String,
}

type FilePtr = Rc<RefCell<FileBuf>>;

struct FileMod {
    file_ptrs: Vec<FilePtr>,
    curr_file: Option<FilePtr>,
}

impl FileMod {
    pub fn new() -> Self {
        FileMod {
            file_ptrs: Vec::new(),
            curr_file: None,
        }
    }
}

impl From<Vec<FileBuf>> for FileMod {
    fn from(file_bufs: Vec<FileBuf>) -> Self {
        let mut file_ptrs: Vec<FilePtr> = Vec::new();

        let curr_file = if file_bufs.is_empty() {
            None
        } else {
            for file_buf in file_bufs {
                file_ptrs.push(Rc::new(RefCell::new(file_buf)));
            }
            Some(file_ptrs[0].clone())
        };

        FileMod {
            file_ptrs,
            curr_file,
        }
    }
}
