use crate::fs::query;

use super::id::Id;

use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Index {
    pub id_by_path: HashMap<PathBuf, Option<Id>>,
    pub path_by_id: HashMap<Id, PathBuf>
}

impl Index {
    pub fn new(root: &Path, data_dir: &Path) -> Index {
        Index {
            id_by_path: query::list_tree(root, data_dir)
                .map(|e|
                    (e.strip_prefix(root).unwrap()
                      .to_path_buf(),
                     None))
                .collect(),

            path_by_id: HashMap::new()
        }
    }

    pub fn provide(&mut self, path: &Path) -> Id {
        let new_id = query::id(path);

        let old_id2 = self.id_by_path.get(path);
        debug_assert!(old_id2.is_some());

        let old_id = self.id_by_path.insert(
            path.to_path_buf(),
            Some(new_id));
        debug_assert!(old_id.is_some()); //todo: otherwise, the file is created after start
        let old_id = old_id.unwrap();

        if let Some(old_id) = old_id {
            if old_id == new_id {
                debug_assert_eq!(&self.path_by_id[&old_id], path);
                //todo: if paths are different then the file was moved
            }
            //todo: other corner cases?
        }

        let _old_path = self.path_by_id.insert(new_id, path.to_path_buf());
        //todo: should I check paths?

        new_id
    }
}