// This implementation is modified from:
// https://github.com/gifnksm/topological-sort-rs
//
// Modifications were to use fxhash and arrayvec for improved performance (because it's fun).

use arrayvec::ArrayVec;
use fxhash::{FxHashMap, FxHashSet};
use std::collections::hash_map::Entry;
use std::hash::Hash;

struct Dependency<T> {
    num_prec: usize,
    succ: FxHashSet<T>,
}

impl<T: Hash + Eq> Dependency<T> {
    fn new() -> Dependency<T> {
        Self {
            num_prec: 0,
            succ: FxHashSet::default(),
        }
    }
}

pub struct TopSort<T> {
    top: FxHashMap<T, Dependency<T>>,
}

impl<T> Default for TopSort<T> {
    fn default() -> Self {
        Self {
            top: Default::default(),
        }
    }
}

impl<T: Hash + Eq + Clone> TopSort<T> {
    pub fn new() -> TopSort<T> {
        Default::default()
    }

    /// Registers a dependency between two verticies.
    /// * `prec` The element appears before `succ`.
    /// * `succ` The element appears after `prec`.
    pub fn add_dependency(&mut self, prec: T, succ: T) {
        match self.top.entry(prec) {
            Entry::Vacant(e) => {
                let mut dep = Dependency::new();
                dep.succ.insert(succ.clone());
                e.insert(dep);
            }
            Entry::Occupied(e) => {
                if !e.into_mut().succ.insert(succ.clone()) {
                    // already registered
                    return;
                }
            }
        }

        match self.top.entry(succ) {
            Entry::Vacant(e) => {
                let mut dep = Dependency::new();
                dep.num_prec += 1;
                e.insert(dep);
            }
            Entry::Occupied(e) => e.into_mut().num_prec += 1,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.peek().cloned().inspect(|key| {
            self.remove(key);
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top
            .iter()
            .filter(|&(_, v)| v.num_prec == 0)
            .map(|(k, _)| k)
            .next()
    }

    fn remove(&mut self, prec: &T) -> Option<Dependency<T>> {
        let res = self.top.remove(prec);
        if let Some(ref p) = res {
            for s in &p.succ {
                if let Some(y) = self.top.get_mut(s) {
                    y.num_prec -= 1;
                }
            }
        }
        res
    }
}

impl<T: Hash + Eq + Clone> Iterator for TopSort<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

/// ArrayTopSort is the same as TopSort, but with a fixed capacity for verticies and edeges. This
/// allows _most_ things to stay on the stack.
pub struct ArrayTopSort<T, const CAP: usize> {
    top: FxHashMap<T, ArrayDependency<T, CAP>>,
}

impl<T, const CAP: usize> Default for ArrayTopSort<T, CAP> {
    fn default() -> Self {
        Self {
            top: Default::default(),
        }
    }
}

impl<T: Hash + Eq + Clone, const CAP: usize> ArrayTopSort<T, CAP> {
    pub fn new() -> ArrayTopSort<T, CAP> {
        Default::default()
    }

    /// Registers a dependency between two verticies.
    /// * `prec` The element appears before `succ`.
    /// * `succ` The element appears after `prec`.
    pub fn add_dependency(&mut self, prec: T, succ: T) {
        // does not handle "already registered"
        self.top
            .entry(prec)
            .or_insert(ArrayDependency::new())
            .succ
            .push(succ.clone());

        self.top
            .entry(succ)
            .or_insert(ArrayDependency::new())
            .num_prec += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.peek().cloned().inspect(|key| {
            self.remove(key);
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top
            .iter()
            .filter(|&(_, v)| v.num_prec == 0)
            .map(|(k, _)| k)
            .next()
    }

    fn remove(&mut self, prec: &T) -> Option<ArrayDependency<T, CAP>> {
        let res = self.top.remove(prec);
        if let Some(ref p) = res {
            for s in &p.succ {
                if let Some(y) = self.top.get_mut(s) {
                    y.num_prec -= 1;
                }
            }
        }
        res
    }
}

impl<T: Hash + Eq + Clone, const CAP: usize> Iterator for ArrayTopSort<T, CAP> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

struct ArrayDependency<T, const CAP: usize> {
    num_prec: usize,
    succ: ArrayVec<T, CAP>,
}

impl<T: Hash + Eq, const CAP: usize> ArrayDependency<T, CAP> {
    fn new() -> ArrayDependency<T, CAP> {
        Self {
            num_prec: 0,
            succ: ArrayVec::default(),
        }
    }
}
