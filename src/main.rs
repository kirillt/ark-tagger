use serde::{Serialize, Deserialize};
use serde_json::Result;

use crc32fast::Hasher;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Entity {
    id: u32,
    tags: Vec<String>
}

fn main() {
    let root = env::current_dir().unwrap();
    let root = root.canonicalize().unwrap();
    println!("Root: {:?}", root);

    let mut args = env::args();
    args.next();

    let path = args.next().unwrap_or_else(|| {
        println!("Please, specify a path to the file");
        std::process::exit(1);
    });
    let path = Path::new(&path).canonicalize().unwrap();
    println!("Canonical path: {:?}", path);
    let path = path.strip_prefix(root).unwrap();
    println!("Relative path: {:?}", path);

    let tags: Vec<String> = args.collect();
    if tags.is_empty() {
        println!("Please, provide at least one tag");
        std::process::exit(1);
    }

    let entity = Entity {
        id: crc32(path),
        tags
    };

    println!("{:?}", entity);
}

fn crc32(path: &Path) -> u32 {
    //use sha1 in case of collisions

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
