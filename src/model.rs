use crate::query;

use std::path::{Path, PathBuf};
use std::collections::HashMap;

pub struct Model {
    pub index: Index,
    pub location: Location,
}

pub type Id = u32;

pub type Tag = &'static str;

#[derive(Debug, Clone)]
pub struct Index {
    pub id_by_path: HashMap<PathBuf, Option<Id>>,
    pub path_by_id: HashMap<Id, PathBuf>
}

pub struct Location {
    pub path: PathBuf,
    pub entries: Vec<Entry>,
    pub depth: usize
}

impl Location {
    pub fn new(path: &Path, depth: usize) -> Self {
        let entries = query::list_entries(&path);
        Location { path: path.to_path_buf(), entries, depth }
    }

    pub fn ascend(&self) -> Location {
        assert!(self.depth > 0);
        Location::new(self.path.parent().unwrap(), self.depth - 1)
    }

    pub fn descend(&self, i: usize) -> Location {
        let entry = &self.entries[i];
        Location::new(&entry.path, self.depth + 1)
    }
}

pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool
}
