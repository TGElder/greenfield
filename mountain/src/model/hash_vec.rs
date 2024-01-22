use std::collections::HashSet;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HashVec<T>
where
    T: Copy + Eq + Hash,
{
    waiting: HashSet<T>,
    queue: Vec<T>,
}

impl<T> HashVec<T>
where
    T: Copy + Eq + Hash,
{
    pub fn new() -> HashVec<T> {
        HashVec {
            waiting: HashSet::new(),
            queue: Vec::new(),
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.waiting.contains(value)
    }

    pub fn push(&mut self, value: T) {
        self.waiting.insert(value);
        self.queue.push(value);
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.queue.retain(|value| {
            let out = f(value);
            if !out {
                self.waiting.remove(value);
            }
            out
        })
    }
}
