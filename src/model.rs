use crate::query;

use crate::{ROOT, DATA, DATA_NAME};

use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};

pub struct Model {
    pub index: Index,
    pub database: Database,
    pub location: Location,
}

impl Model {
    pub fn new() -> Self {
        Model {
            index: Index::new(&ROOT, &DATA),
            database: Database::new(&DATA),
            location: Location::new(&ROOT, 0),
        }
    }
}

pub type Id = u32;

pub type Tag = String;

#[derive(Debug, Clone)]
pub struct Index {
    pub id_by_path: HashMap<PathBuf, Option<Id>>,
    pub path_by_id: HashMap<Id, PathBuf>
}

impl Index {
    pub fn new(root: &Path, data: &Path) -> Index {
        Index {
            id_by_path: query::list_tree(root, data)
                .map(|e|
                    (e.strip_prefix(root).unwrap()
                      .to_path_buf(),
                    None))
                .collect(),

            path_by_id: HashMap::new()
        }
    }
}

pub struct Database {
    pub ids_by_tag: HashMap<Tag, HashSet<Id>>,
}

pub struct Bucket {
    pub tag: Tag,
    pub ids: HashSet<Id>
}

impl Database {
    pub fn new(path: &Path) -> Self {
        let mut ids_by_tag = HashMap::new();

        let buckets = query::scan_buckets(&path);
        for Bucket { tag, ids } in buckets.into_iter() {
            ids_by_tag.insert(tag.clone(), ids);
        }

        Database { ids_by_tag }
    }
}

pub struct Location {
    pub path: PathBuf,
    pub entries: Vec<Entry>,
    pub depth: usize
}

impl Location {
    pub fn new(path: &Path, depth: usize) -> Self {
        let entries = query::list_entries(&path)
            .filter(|e| e.name != *DATA_NAME
                    && e.path != *DATA)
            .collect();

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
