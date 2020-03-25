#[macro_use]
extern crate lazy_static;

//use serde::{Serialize, Deserialize};
//use serde_json::Result;

use crc32fast::Hasher;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::env;

lazy_static! {
    static ref ROOT: PathBuf =
        env::current_dir().unwrap()
            .canonicalize().unwrap();

    static ref DATA: PathBuf = {
        let mut path = ROOT.clone();
        path.push(".ark-tags");
        path
    };
}

//#[derive(Serialize, Deserialize, Debug)]
//struct Entity {
//    id: u32,
//    tags: Vec<String>
//}

fn main() {
    println!("Root: {:?}", *ROOT);

    let mut args = env::args();
    args.next();

    let path = args.next().unwrap_or_else(|| {
        println!("Please, specify a path to the file");
        std::process::exit(1);
    });
    let path = Path::new(&path).canonicalize().unwrap();
    println!("Canonical path: {:?}", path);
    let path = path.strip_prefix(&*ROOT).unwrap();
    println!("Relative path: {:?}", path);

    let tags: Vec<String> = args.collect();
    if tags.is_empty() {
        println!("Please, provide at least one tag");
        std::process::exit(1);
    }

    let id: u32 = crc32(path);
    //let entity = Entity { id, tags };
    
    for tag in tags.iter() {
        label(id, &path, &tag);
    }

    println!("{:?}", id);
}

fn label(id: u32, target: &Path, tag: &str) {
    let mut path: PathBuf = DATA.clone();
    path.push(tag);

    fs::create_dir_all(&path).unwrap();
    path.push(format!("{}", id));

    let mut ref_file = File::create(&path).unwrap();
    ref_file.write(
        target.to_str().unwrap()
            .as_bytes()).unwrap();
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
