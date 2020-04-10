pub mod id;
pub mod tag;
pub mod index;
pub mod database;

use id::Id;
use tag::Tag;
use database::Database;
use index::Index;

use crate::query;
use crate::action;
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
        let mut index = Index::new(&ROOT, &DATA);
        let location = Location::new(&mut index, &ROOT, 0);
        let database = Database::new(&DATA);
        Model { index, database, location }
    }
}

pub struct Location {
    pub path: PathBuf,
    pub directories: Vec<Entry>,
    pub files: Vec<Entry>,
    pub depth: usize
}

impl Location {
    pub fn new(index: &mut Index, path: &Path, depth: usize) -> Self {
        let mut directories = vec![];
        let mut files = vec![];

        for (is_dir, entry) in query::list_entries(&path) {
            if is_dir {
                directories.push(entry);
            } else {
                files.push(entry);
            }
        }

        if depth == 0 {
            directories = directories.into_iter()
                .filter(|e| e.name != *DATA_NAME && e.path != *DATA)
                .collect();
        }

        for e in files.iter() {
            index.provide(&e.path);
        }

        Location { path: path.canonicalize().unwrap(), directories, files, depth }
    }

    pub fn ascend(&self, index: &mut Index) -> Location {
        assert!(self.depth > 0);
        Location::new(index,self.path.parent().unwrap(), self.depth - 1)
    }

    pub fn descend(&self, index: &mut Index, i: usize) -> Location {
        let dir = &self.directories[i];
        Location::new(index, &dir.path, self.depth + 1)
    }
}

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
}