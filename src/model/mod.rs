pub mod id;
pub mod tag;
pub mod index;
pub mod database;
pub mod location;

use database::Database;
use index::Index;
use location::Location;

use crate::{ROOT, DATA};

pub struct Model<'a> {
    pub index: Index<'a>,
    pub database: Database<'a>,
    pub location: Location,
}

impl<'a> Model<'a> {
    pub fn new() -> Self {
        let mut index = Index::new(&ROOT, &DATA);
        let location = Location::new(&mut index, &ROOT, 0);
        let database = Database::new(&DATA);
        Model { index, database, location }
    }
}