use crate::model::{Model, Id, Index, Location, Entry};

use crate::{ROOT, DATA, DATA_NAME};

use walkdir::WalkDir;
use crc32fast::Hasher;
use std::collections::HashMap;
use std::path::{Path};
use std::fs::{self, File};
use std::io::Read;

pub fn init_model() -> Model {
    Model {
        index: init_index(),
        location: Location::new(&ROOT, 0),
    }
}

pub fn init_index() -> Index {
    let mut id_by_path = HashMap::new();
    let mut path_by_id = HashMap::new();

    let walker = WalkDir::new(&*ROOT)
        .follow_links(false) //todo: enable when paths grouping by id is implemented
        .max_open(8) //small limit -- more memory spent
        //.sort_by()
        .into_iter()
        .filter_entry(|e| e.path() != *DATA);

    for entry in walker {
        let entry = entry.unwrap();
        let path  = entry.path();
        let path = path.strip_prefix(&*ROOT)
            .unwrap().into();

        id_by_path.insert(path, None);
    }
    println!("Total {} paths found", id_by_path.keys().len());

    Index { id_by_path, path_by_id }
}

pub fn list_entries(path: &Path) -> Vec<Entry> {
    fs::read_dir(&path).unwrap()
        .map(|e| e.unwrap())
        .map(|e| Entry {
            name: e.file_name().to_str().unwrap().to_owned(),
            path: e.path(),
            is_dir: e.file_type().unwrap().is_dir()
        })
        .filter(|e| e.name != *DATA_NAME
                    && e.path != *DATA)
        .collect()
}

// CRC-32
// in case of collisions, try sha1
pub fn id(path: &Path) -> Id {
    let mut file = File::open(path).unwrap();

    let mut hasher = Hasher::new();
    //use reset() method when it will become more serious

    let mut buffer: Vec<u8> = vec![0; 512 * 1024];
    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 { break; }
        hasher.update(&buffer);
    }

    hasher.finalize()
}
