use crate::{ROOT, DATA, DATA_NAME};
use crate::model::{
    id::Id,
    database::Bucket,
    location::Entry
};
use crate::utils::measure;

use crc32fast::Hasher;
use std::path::Path;
use std::fs::{self, File};
use std::io::{Read, ErrorKind};

pub fn scan_buckets(path: &Path) -> Vec<Bucket> {
    let directory = fs::read_dir(&path);

    match directory {
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    println!("This is, apparently, fresh directory since the data folder doesn't exist");
                    return vec![];
                },
                _ => panic!(error.to_string())
            }
        },

        Ok(directory) => {
            directory
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
    }
}

pub fn list_entries(path: &Path) -> impl Iterator<Item = (bool, Entry)> {
    let prefix_to_strip: Option<&Path> =
        if path.is_absolute() { Some(&ROOT) }
        else { None };

    fs::read_dir(&path)
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter_map(move |entry| {
            let name = entry.file_name().into_string().unwrap();
            if name.starts_with('.') || name == *DATA_NAME {
                None
            } else {
                let mut path = entry.path();
                if path == *DATA {
                    None
                } else {
                    if let Some(root) = prefix_to_strip {
                        path = path.strip_prefix(root).unwrap().to_path_buf();
                    }

                    let is_dir = entry.file_type().unwrap().is_dir();
                    Some((is_dir, Entry { name, path }))
                }
            }
        })
}

pub fn id(path: &Path) -> Id {
    println!("\t\tpath = {:?}", path);
    let mut file = File::open(path).unwrap();

    let size = size(&file);
    let hash = crc32(&mut file);

    Id { size, hash }
}

fn size(file: &File) -> u64 {
    measure("size", || {
        file.metadata().unwrap().len()
    })
}

// in case of collisions, try sha1
fn crc32(file: &mut File) -> u32 {
    measure("crc32", || {
        let mut hasher = Hasher::new();
        //use reset() method when it will become more serious

        let mut buffer: Vec<u8> = vec![0; 512 * 1024];
        while let Ok(n) = file.read(&mut buffer) {
            if n == 0 { break; }
            hasher.update(&buffer);
        }

        hasher.finalize()
    })
}