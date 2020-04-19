mod bucket;

use crate::model::id::Id;
use crate::model::tag::{Tag, HighlightedTag};
use crate::utils::Filter;

use bucket::Bucket;

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::collections::{BTreeMap, HashSet};

pub struct Database {
    path: PathBuf,

    //BTreeMap is used because keys should be sorted when retrieved
    bucket_by_tag: BTreeMap<Tag, Bucket>
}

impl Database {
    pub fn new(path: PathBuf) -> Self {
        let bucket_by_tag = Self::scan_buckets(&path);
        Database { path, bucket_by_tag }
    }

    pub fn insert<I>(&mut self, ids: I, tag: &Tag) -> bool
        where I: Iterator<Item = Id> {

        let bucket = self.bucket_by_tag.get_mut(tag);
        match bucket {
            Some(bucket) => {
                bucket.insert_all(ids);

                false
            },
            None => {
                let mut path = self.path.clone();
                path.push(tag);

                let mut ids = ids;
                let mut bucket = Bucket::init(path, ids.next().unwrap());
                bucket.insert_all(ids);

                let old = self.bucket_by_tag.insert(tag.clone(), bucket);
                debug_assert!(old.is_none());
                true
            }
        }
    }

    //todo: implement inversion and "fresh" pseudo-tag
    //todo: implement union filters and combinations of unions/intersections
    pub fn filter<'b, I, T>(&self, ids: I, tags: T) -> Filter
        where I: Iterator<Item = Id>,
              T: Iterator<Item = &'b Tag> {
        let ids: Vec<Id> = ids.collect();

        let matches: HashSet<Id> = tags.fold(
            ids.iter().cloned().collect(),
            |acc, tag|
                acc.intersection(&self.bucket_by_tag.get(tag).unwrap().values())
                    .cloned()
                    .collect());

        ids.iter()
            .map(|id| matches.contains(id))
            .collect()
    }

    pub fn sieved_tags<I>(&self, ids: I) -> impl Iterator<Item = HighlightedTag>
        where I: Iterator<Item = Id> {
        let ids: HashSet<Id> = ids.collect();

        self.bucket_by_tag.iter()
            .map(move |(tag, bucket)|
                HighlightedTag {
                    highlighted: bucket.values()
                        .intersection(&ids)
                        .next().is_some(),
                    tag
                })
    }

    pub fn sieve<'a, I: 'a>(&'a self, ids: I) -> impl Iterator<Item = bool> + 'a
        where I: Iterator<Item = Id> {
        self.sieved_tags(ids).map(|HighlightedTag { highlighted, tag: _}| highlighted)
    }

    fn scan_buckets(path: &Path) -> BTreeMap<Tag, Bucket> {
        println!("{:?}", path);
        let directory = fs::read_dir(&path);

        match directory {
            Err(error) => {
                match error.kind() {
                    ErrorKind::NotFound => {
                        println!("There is no database yet");
                        return BTreeMap::new();
                    },
                    _ => panic!(error.to_string())
                }
            },

            Ok(directory) => {
                directory.map(|entry| {
                    let entry = entry.unwrap();

                    let bucket = Bucket::load(entry.path());
                    let tag = entry.file_name().into_string().unwrap();

                    (tag, bucket)
                }).collect()
            }
        }
    }
}