use std::ops::{Deref, Index};

use arrayvec::ArrayVec;

pub struct Bucket<K, V, const CAP: usize> {
    keys: Vec<K>,
    vals: Vec<ArrayVec<V, CAP>>,
}

impl<K: Eq + Copy, V: Eq + Copy, const CAP: usize> Bucket<K, V, CAP> {
    pub const fn new() -> Self {
        Self {
            keys: Vec::new(),
            vals: Vec::new(),
        }
    }

    pub fn push(&mut self, k: K, v: V) {
        if let Some(idx) = self.keys.iter().position(|&x| x == k) {
            self.vals[idx].push(v);
        } else {
            self.keys.push(k);
            self.vals.push(ArrayVec::new());
            self.vals.last_mut().unwrap().push(v);
        }
    }

    pub fn remove(&mut self, k: K, v: V) {
        if let Some(idx) = self.keys.iter().position(|&x| x == k) {
            if let Some(sub_idx) = self.vals[idx].iter().position(|&x| x == v) {
                self.vals[idx].swap_remove(sub_idx);
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
