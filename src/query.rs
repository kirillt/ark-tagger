use crate::model::{id::Id, Bucket, Entry};
use crate::ROOT;

use walkdir::WalkDir;
use crc32fast::Hasher;
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
    let prefix_to_strip: Option<&Path> =
        if path.is_absolute() { Some(&ROOT) }
        else { None };

    fs::read_dir(&path)
        .unwrap()
        .map(|e| e.unwrap())
        .map(move |e| {
            let mut path = e.path();
            if let Some(root) = prefix_to_strip {
                path = path.strip_prefix(root).unwrap().to_path_buf();
            }

            Entry {
                name: e.file_name().to_str().unwrap().to_owned(),
                path: path,
                is_dir: e.file_type().unwrap().is_dir()
            }
        })
}

pub fn id(path: &Path) -> Id {
    use std::time::Instant;

    println!("\t\tpath = {:?}", path);
    let mut file = File::open(path).unwrap();

    let start = Instant::now();
    let size = size(&file);
    println!("\t\t\tsize retrieved in {}ns", start.elapsed().as_nanos());

    let start = Instant::now();
    let hash = crc32(&mut file);
    println!("\t\t\thash calculated in {}ns", start.elapsed().as_nanos());

    Id { size, hash }
}

fn size(file: &File) -> u64 {
    file.metadata().unwrap().len()
}

// in case of collisions, try sha1
fn crc32(file: &mut File) -> u32 {
    let mut hasher = Hasher::new();
    //use reset() method when it will become more serious

    let mut buffer: Vec<u8> = vec![0; 512 * 1024];
    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 { break; }
        hasher.update(&buffer);
    }

    hasher.finalize()
}