pub mod id;
pub mod tag;
pub mod location;

use crate::database::Database;
use crate::index::Index;
use crate::{DATA_NAME, INDEX_NAME};

use location::Location;

use std::path::PathBuf;

pub struct Model {
    pub index: Index,
    pub database: Database,
    pub location: Location,
}

impl Model {
    pub fn new(root: PathBuf) -> Self {
        let mut index_dir = root.clone();
        let mut data_dir = root.clone();
        index_dir.push(INDEX_NAME.to_owned());
        data_dir.push(DATA_NAME.to_owned());

        let ignores = vec![INDEX_NAME.to_string(), DATA_NAME.to_string()];
            //todo: remove clone()

        let mut index = Index::new(index_dir);
        let database = Database::new(data_dir);

        let location = Location::root(root, ignores, &mut index);
        Model { index, database, location }
    }
}