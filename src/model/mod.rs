pub mod id;

use id::Id;

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

    pub fn refresh(&mut self, path: PathBuf) -> Id {
        let new_id = query::id(&path);

        let old_id2 = self.id_by_path.get(&path);
        debug_assert!(old_id2.is_some());

        let old_id = self.id_by_path.insert(
            path.clone(),
            Some(new_id));
        debug_assert!(old_id.is_some()); //todo: otherwise, the file is created after start
        let old_id = old_id.unwrap();

        if let Some(old_id) = old_id {
            if old_id == new_id {
                debug_assert_eq!(&self.path_by_id[&old_id], &path);
                //todo: if paths are different then the file was moved
            }
            //todo: other corner cases?
        }

        let _old_path = self.path_by_id.insert(new_id, path);
        //todo: should I check paths?

        new_id
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

    pub fn insert(&mut self, ids: HashSet<Id>, tag: &Tag) -> bool {
        let mut new_tag = false;

        let bucket: HashSet<Id> = self.ids_by_tag.get(tag)
            .map(|ids| ids.into_iter().cloned().collect())
            .unwrap_or_else(|| {
                new_tag = true;
                HashSet::new()
            });

        for id in ids.iter() {
            action::label(&id, &tag);
        }

        let bucket: HashSet<Id> = bucket.union(&ids).cloned().collect();
        self.ids_by_tag.insert(tag.clone(), bucket);
        new_tag
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
            index.refresh(e.path.clone());
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