// Module for CommentMapStruct

use fnv::FnvHashMap;
use std::borrow::Borrow;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct FallbackHashMap<K: Hash + Eq, V> {
    map: FnvHashMap<K, V>,
    fallback_value: V,
}

impl<K: Hash + Eq, V> FallbackHashMap<K, V> {
    pub fn new(fallback_value: V) -> FallbackHashMap<K, V> {
        FallbackHashMap {
            map: FnvHashMap::default(),
            fallback_value,
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.map.insert(k, v)
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> &V
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get(k).unwrap_or(&self.fallback_value)
    }
}
