use crate::model::{Id, Tag};

use crate::DATA;

use std::path::{Path, PathBuf};
use std::fs::{self, File};

pub fn label(id: Id, target: &Path, tag: Tag) {
    let mut path: PathBuf = DATA.clone();
    path.push(tag);

    fs::create_dir_all(&path).unwrap();
    path.push(format!("{}", id));

    File::create(&path).unwrap();
}

