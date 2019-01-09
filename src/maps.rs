// Module for CommentMapStruct

use fnv::FnvHashMap;
use std::borrow::Borrow;
use std::hash::Hash;

/// FallbackHashMap is a Hashmap that yields a fallback value for `get(k)` if `k` has not been
/// inserted.
///
/// FallbackHashMap uses `fnv::FnvHashMap` as its hasher.
#[derive(Debug, Clone)]
pub struct FallbackHashMap<K: Hash + Eq, V> {
    map: FnvHashMap<K, V>,
    fallback_value: V,
}

impl<K: Hash + Eq, V> FallbackHashMap<K, V> {
    /// creates new FallbackHashMap
    pub fn new(fallback_value: V) -> FallbackHashMap<K, V> {
        FallbackHashMap {
            map: FnvHashMap::default(),
            fallback_value,
        }
    }

    /// Inserts value `v` into FallbackHashMap with key `k`
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.map.insert(k, v)
    }

    /// Gets the value for key `k`. If `k` has not been inserted, fallback value is returned
    pub fn get<Q: ?Sized>(&self, k: &Q) -> &V
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get(k).unwrap_or(&self.fallback_value)
    }
}
