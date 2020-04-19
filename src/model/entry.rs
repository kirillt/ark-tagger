use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug)]
pub struct DirEntry {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,

    pub size: u64,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub accessed: SystemTime,
}