use crate::query;
use crate::action;

use super::id::Id;
use super::tag::{Tag, HighlightedTag};

use std::path::Path;
use std::collections::{BTreeMap, HashSet, BTreeSet};

pub struct Database {
    //BTreeMap is used because keys should be sorted when retrieved
    ids_by_tag: BTreeMap<Tag, HashSet<Id>>
}

pub struct Bucket {
    pub tag: Tag,
    pub ids: HashSet<Id>
}

pub type Filter = BTreeSet<usize>;

impl Database {
    pub fn new(path: &Path) -> Self {
        let mut ids_by_tag = BTreeMap::new();

        let buckets = query::scan_buckets(&path);
        for Bucket { tag, ids } in buckets.into_iter() {
            ids_by_tag.insert(tag.clone(), ids);
        }

        Database { ids_by_tag }
    }

    pub fn insert<I>(&mut self, ids: I, tag: &Tag) -> bool
    where I: Iterator<Item = Id> {
        let ids: HashSet<Id> = ids.collect();
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
    pub fn filter<'b, I, T>(&self, ids: I, tags: T) -> Filter
    where I: Iterator<Item = Id>,
          T: Iterator<Item = &'b Tag> {
        let ids: Vec<Id> = ids.collect();

        let matches: HashSet<Id> = tags.fold(
            ids.iter().cloned().collect(),
            |acc, tag|
                    acc.intersection(&self.ids_by_tag[tag])
                        .cloned()
                        .collect());

        let mut filter = BTreeSet::new();
        for (i, id) in ids.iter().enumerate() {
            if matches.contains(id) {
                filter.insert(i);
            }
        }

        filter
    }

    pub fn sieved_tags<I>(&self, ids: I) -> impl Iterator<Item = HighlightedTag>
        where I: Iterator<Item = Id> {
        let ids: HashSet<Id> = ids.collect();

        self.ids_by_tag.iter()
            .map(move |(tag, bucket)|
                HighlightedTag {
                    highlighted: bucket.intersection(&ids).next().is_some(),
                    tag
                })
    }

    pub fn sieve<'a, I: 'a>(&'a self, ids: I) -> impl Iterator<Item = bool> + 'a
    where I: Iterator<Item = Id> {
        self.sieved_tags(ids).map(|HighlightedTag { highlighted, tag: _}| highlighted)
    }
}