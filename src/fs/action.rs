use crate::model::{id::Id, tag::Tag};

use crate::DATA;

use std::path::PathBuf;
use std::fs::{self, File};

pub fn label(id: &Id, tag: &Tag) {
    let mut path: PathBuf = DATA.clone();
    path.push(tag);

    fs::create_dir_all(&path).unwrap();
    path.push(format!("{}", id.to_string()));

    File::create(&path).unwrap();
}