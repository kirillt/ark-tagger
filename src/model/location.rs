use crate::index::Index;

use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
}

pub struct Location {
    pub directories: Vec<Entry>,
    pub files: Vec<Entry>,
    pub depth: usize,

    ignores: Vec<String>,
    path: PathBuf
}

impl Location {
    pub fn root(path: PathBuf, ignores: Vec<String>, index: &mut Index) -> Self {
        let path = path.canonicalize().unwrap();

        let (directories, files) = Self::list_entries(&path, Some(&ignores));
        for file in files.iter() {
            index.provide(&file.path);
        }

        Location { directories, files, depth: 0, ignores, path }
    }

    pub fn ascend(&self, index: &mut Index) -> Self {
        assert!(self.depth > 0);

        let parent = self.path.parent().unwrap();
        println!("\t\tpath: {:?}", parent);

        let ignores = if self.depth == 1 {
            Some(&self.ignores)
        } else {
            None
        };

        let (directories, files) = Self::list_entries(&parent, ignores);
        for file in files.iter() {
            index.provide(&file.path);
        }

        Location {
            directories,
            files,
            depth: self.depth - 1,

            ignores: self.ignores.clone(),
            path: parent.to_path_buf()
        }
    }

    pub fn descend(&self, index: &mut Index, i: usize) -> Self {
        let target = &self.directories[i].path;
        println!("\t\tpath: {:?}", target);

        let (directories, files) = Self::list_entries(&target, None);
        for file in files.iter() {
            index.provide(&file.path);
        }

        Location {
            directories,
            files,
            depth: self.depth + 1,

            ignores: self.ignores.clone(),
            path: target.clone()
        }
    }

    pub fn activate(&self, i: usize) {
        let path = &self.files[i].path;
        println!("\t\tpath {:?}", path);
        opener::open(path).unwrap();
    }

    fn list_entries(path: &Path, ignores: Option<&Vec<String>>) -> (Vec<Entry>, Vec<Entry>) {
        let entries = fs::read_dir(&path)
            .unwrap()
            .map(|entry| entry.unwrap())
            .filter_map(move |entry| {
                let name = entry.file_name().into_string().unwrap();
                if name.starts_with('.') ||
                    ignores.map(|ignores| ignores.contains(&name))
                        .unwrap_or(false) {
                    None
                } else {
                    let is_dir = entry.file_type().unwrap().is_dir();
                    Some((is_dir, Entry { name, path: entry.path() }))
                }
            });

        let mut directories = vec![];
        let mut files = vec![];
        for (is_dir, entry) in entries {
            if is_dir {
                directories.push(entry);
            } else {
                files.push(entry);
            }
        }

        (directories, files)
    }
}