use crate::query;
use crate::action;

use super::id::Id;
use super::tag::{Tag, Tags};

use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet, BTreeSet};

pub struct Database {
    pub ids_by_tag: HashMap<Tag, HashSet<Id>>,
}

pub struct Bucket {
    pub tag: Tag,
    pub ids: HashSet<Id>
}

pub type Filter = BTreeSet<usize>;

impl Database {
    pub fn new(path: &Path) -> Self {
        let mut ids_by_tag = HashMap::new();

        let buckets = query::scan_buckets(&path);
        for Bucket { tag, ids } in buckets.into_iter() {
            ids_by_tag.insert(tag.clone(), ids);
        }

        Database { ids_by_tag }
    }

    pub fn insert(&mut self, ids: HashSet<Id>, tag: &Tag) -> bool {
        let mut new_tag = false;

        let bucket: HashSet<Id> = self.ids_by_tag.get(tag)
            .map(|ids| ids.into_iter().cloned().collect())
            .unwrap_or_else(|| {
                new_tag = true;
                HashSet::new()
            });

        for id in ids.iter() {
            action::label(&id, &tag);
        }

        let bucket: HashSet<Id> = bucket.union(&ids).cloned().collect();
        self.ids_by_tag.insert(tag.clone(), bucket);
        new_tag
    }

    //todo: implement union filters and combinations of unions/intersections
    pub fn filter<I: Iterator<Item = Id>>(&self, ids: I, tags: Tags) -> Filter {
        let ids: Vec<Id> = ids.collect();

        let matches: HashSet<Id> = tags.into_iter()
            .fold(ids.iter().cloned().collect(), |acc, tag|
                acc.intersection(&self.ids_by_tag[&tag]).cloned().collect());

        let mut filter = BTreeSet::new();
        for (i, id) in ids.iter().enumerate() {
            if matches.contains(id) {
                filter.insert(i);
            }
        }

        filter
    }
}