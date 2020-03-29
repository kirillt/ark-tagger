use crate::model::{Model, Id, Index, Bucket, Location, Entry};


use walkdir::WalkDir;
use crc32fast::Hasher;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Read;

pub fn list_tree(root: &Path, data: &Path) -> impl Iterator<Item = PathBuf> {
    let data = data.to_path_buf();

    WalkDir::new(root)
        .follow_links(false) //todo: enable when paths grouping by id is implemented
        .max_open(8) //small limit -- more memory spent
        //.sort_by()
        .into_iter()
        .filter_entry(move |e| e.path() != data)
        .map(|e| e.unwrap()
            .into_path())
}

pub fn scan_buckets(path: &Path) -> Vec<Bucket> {
    fs::read_dir(&path)
        .unwrap()
        .map(|e| e.unwrap())
        .map(|e| {
            assert!(e.file_type().unwrap().is_dir());
            let ids = fs::read_dir(e.path())
                .unwrap()
                .map(|e| e.unwrap())
                .map(|e| {
                    assert!(!e.file_type().unwrap().is_dir());
                    e.file_name().to_str().unwrap().parse::<Id>().unwrap()
                })
                .collect();

            let tag = e.file_name().into_string().unwrap();
            Bucket { tag, ids }
        })
        .collect()
}

pub fn list_entries(path: &Path) -> impl Iterator<Item = Entry> {
    fs::read_dir(&path)
        .unwrap()
        .map(|e| e.unwrap())
        .map(|e| Entry {
            name: e.file_name().to_str().unwrap().to_owned(),
            path: e.path(),
            is_dir: e.file_type().unwrap().is_dir()
        })
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
