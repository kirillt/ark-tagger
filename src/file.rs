use crate::model::id::Id;
use crate::utils::measure;

use crc32fast::Hasher;
use std::path::Path;
use std::fs::File;
use std::io::Read;

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