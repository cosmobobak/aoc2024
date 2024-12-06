#![allow(clippy::module_name_repetitions)]

use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
    ops::{Deref, Index},
};

use arrayvec::ArrayVec;
use fxhash::FxHashMap;

pub struct ArrayBucket<K, V, const CAP: usize, const SORT: bool> {
    keys: Vec<K>,
    vals: Vec<ArrayVec<V, CAP>>,
}

impl<K: Eq + Copy, V: Eq + Copy + Ord, const CAP: usize, const SORT: bool>
    ArrayBucket<K, V, CAP, SORT>
{
    pub const fn new() -> Self {
        Self {
            keys: Vec::new(),
            vals: Vec::new(),
        }
    }

    pub fn push(&mut self, k: K, v: V) {
        if let Some(idx) = self.keys.iter().position(|&x| x == k) {
            if SORT {
                let insert_point = self.vals[idx].partition_point(|&val| val < v);
                self.vals[idx].insert(insert_point, v);
            } else {
                self.vals[idx].push(v);
            }
        } else {
            self.keys.push(k);
            self.vals.push(ArrayVec::new());
            self.vals.last_mut().unwrap().push(v);
        }
    }

    pub fn remove(&mut self, k: K, v: V) {
        if let Some(idx) = self.keys.iter().position(|&x| x == k) {
            if let Some(sub_idx) = self.vals[idx].iter().position(|&x| x == v) {
                if SORT {
                    self.vals[idx].remove(sub_idx);
                } else {
                    self.vals[idx].swap_remove(sub_idx);
                }
            }
        }
    }

    pub fn find(&self, k: &K) -> Option<&[V]> {
        self.keys
            .iter()
            .position(|x| x == k)
            .map(|idx| &*self.vals[idx])
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &[V])> {
        self.keys.iter().zip(self.vals.iter().map(Deref::deref))
    }
}

pub struct HashBucket<K, V, const CAP: usize, const SORT: bool> {
    map: FxHashMap<K, ArrayVec<V, CAP>>,
}

impl<K: Eq + Copy + Hash, V: Eq + Copy + Ord, const CAP: usize, const SORT: bool>
    HashBucket<K, V, CAP, SORT>
{
    pub fn new() -> Self {
        Self {
            map: FxHashMap::default(),
        }
    }

    pub fn push(&mut self, k: K, v: V) {
        match self.map.entry(k) {
            Entry::Occupied(mut occupied_entry) => {
                let vec = occupied_entry.get_mut();
                if SORT {
                    let insert_point = vec.partition_point(|&val| val < v);
                    vec.insert(insert_point, v);
                } else {
                    vec.push(v);
                }
            }
            Entry::Vacant(vacant_entry) => vacant_entry.insert(ArrayVec::new()).push(v),
        }
    }

    pub fn remove(&mut self, k: K, v: V) {
        if let Entry::Occupied(mut occupied_entry) = self.map.entry(k) {
            let vec = occupied_entry.get_mut();
            if let Some(sub_idx) = vec.iter().position(|&x| x == v) {
                if SORT {
                    vec.remove(sub_idx);
                } else {
                    vec.swap_remove(sub_idx);
                }
            }
        }
    }

    pub fn find(&self, k: &K) -> Option<&[V]> {
        self.map.get(k).map(|vec| &**vec)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &[V])> {
        self.map.iter().map(|(k, vec)| (k, &**vec))
    }
}
