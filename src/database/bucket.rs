use crate::model::id::Id;

use std::path::PathBuf;
use std::fs::{self, File};
use std::collections::HashSet;

pub struct Bucket {
    path: PathBuf,
    ids: HashSet<Id>
}

impl Bucket {
    pub fn init(path: PathBuf, id: Id) -> Self {
        let mut ids = HashSet::new();
        fs::create_dir_all(&path).unwrap();
        ids.insert(id);

        Bucket { path, ids }
    }

    pub fn load(path: PathBuf) -> Self {
        let ids = fs::read_dir(&path)
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                debug_assert!(entry.file_type().unwrap().is_file());
                entry.file_name().to_str().unwrap().parse::<Id>().unwrap()
            })
            .collect();

        Bucket { path, ids }
    }

    pub fn values(&self) -> &HashSet<Id> {
        &self.ids
    }

    pub fn insert_all<I>(&mut self, ids: I)
    where I: Iterator<Item = Id> {
        ids.for_each(|id| self.insert(id))
    }

    fn insert(&mut self, id: Id) {
        if !self.ids.contains(&id) {
            self.ids.insert(id);

            let mut path = self.path.clone();
            path.push(id.to_string());

            File::create(&path).unwrap();
        }
    }
}