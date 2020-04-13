use crate::file;
use crate::model::id::Id;
use crate::utils::measure;

use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Index {
    id_by_path: HashMap<PathBuf, Id>,
    path_by_id: HashMap<Id, PathBuf>
}

impl Index {
    pub fn new(_path: PathBuf) -> Index {
        Index {
            id_by_path: HashMap::new(),
            path_by_id: HashMap::new()
        }
    }

    pub fn provide(&mut self, path: &Path) {
        let id = measure("id", || file::id(path));

        measure("index.id.insertion", ||
            self.id_by_path.insert(path.to_path_buf(), id));
    }

    pub fn id(&mut self, path: &Path) -> Id {
        self.id_by_path[path]
    }

    pub fn path(&mut self, _id: Id) -> () {
        //todo: this map should be used after database filter
        // the whole files tree not only top level
        unimplemented!()
    }
}