use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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

impl FileEntry {
    pub fn created_secs(&self) -> u64 {
        Self::unix_time(self.created)
    }

    pub fn modified_secs(&self) -> u64 {
        Self::unix_time(self.modified)
    }

    pub fn accessed_secs(&self) -> u64 {
        Self::unix_time(self.accessed)
    }

    fn unix_time(time: SystemTime) -> u64 {
        let since_the_epoch = time.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs()
    }
}