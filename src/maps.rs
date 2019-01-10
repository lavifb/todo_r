// Module for CommentMapStruct

use fnv::FnvHashMap;
use regex::Regex;
use std::borrow::Borrow;
use std::hash::Hash;

use crate::comments::CommentTypes;
use crate::parser::build_parser_regexs;

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
    /// Creates new FallbackHashMap
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

    /// Gets the value for key `k`. Returns `None` instead of a fallback if the key is not found
    pub fn get_without_fallback<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get(k)
    }
}

/// Hashmap that does not need to copy values for two keys to have same value.
/// Also, it converts `CommentTypes` into `Vec<Regex>` and caches the value to avoid
/// repeated conversions.
///
/// Note that CommentRegexMultiMap is not designed to remove items or repeatedly change values for
/// a given key.
/// This is because values are not reference counted and thus cannot be discarded once added.
/// This lets it not have to hash twice and thus be a little more performant.
#[derive(Debug, Clone)]
pub struct CommentRegexMultiMap<K: Hash + Eq> {
    map: FallbackHashMap<K, usize>,
    comment_types: Vec<CommentTypes>,
    regexs: Vec<Option<Vec<Regex>>>,
}

impl<K: Hash + Eq> CommentRegexMultiMap<K> {
    /// Creates new CommentRegexMultiMap
    pub fn new(fallback_value: CommentTypes) -> CommentRegexMultiMap<K> {
        let mut comment_types = Vec::new();
        comment_types.push(fallback_value);

        let mut regexs = Vec::new();
        regexs.push(None);

        CommentRegexMultiMap {
            map: FallbackHashMap::new(0),
            comment_types,
            regexs,
        }
    }

    /// Inserts value `v` with key `k`
    // MAYB?: reference count to delete unused CommentTypes if inserted over.
    #[allow(dead_code)]
    pub fn insert(&mut self, k: K, v: CommentTypes) {
        let i = self.comment_types.len();
        self.map.insert(k, i);
        self.comment_types.push(v);
        self.regexs.push(None);
    }

    /// Inserts value `v` for all keys in `ks`
    pub fn insert_keys(&mut self, ks: impl IntoIterator<Item = K>, v: CommentTypes) {
        let i = self.comment_types.len();
        for k in ks {
            self.map.insert(k, i);
        }
        self.comment_types.push(v);
        self.regexs.push(None);
    }

    /// Gets the the Vec<Regex> built from the inserted CommentTypes for key `k`.
    /// The Vec<Regex> is cached so the regexs do not need to be rebuilt.
    /// If `k` has not been inserted, fallback value is returned
    pub fn get<Q: ?Sized>(&mut self, k: &Q, tags: &[String]) -> &Vec<Regex>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let v_i = *self.map.get(k);
        let comment_types = &self.comment_types;
        self.regexs[v_i].get_or_insert_with(|| build_parser_regexs(&comment_types[v_i], tags))
    }

    /// Same as `get()` except it does not fallback if the key is not found.
    #[allow(dead_code)]
    pub fn get_without_fallback<Q: ?Sized>(&mut self, k: &Q, tags: &[String]) -> Option<&Vec<Regex>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.map.get_without_fallback(k) {
            Some(pv_i) => {
                let v_i = *pv_i;
                let comment_types = &self.comment_types;
                Some(
                    self.regexs[v_i]
                        .get_or_insert_with(|| build_parser_regexs(&comment_types[v_i], tags)),
                )
            }
            None => None,
        }
    }

    /// Resets the fallback value
    #[allow(dead_code)]
    pub fn reset_fallback_value(&mut self, new_fallback_value: CommentTypes) {
        self.comment_types[0] = new_fallback_value;
        self.regexs[0] = None;
    }

    /// Resets the fallback value to the one given by `new_fallback_key`.
    /// Returns the new fallback `Some(CommentTypes)` if succeeded and `None` otherwise.
    pub fn reset_fallback_key<Q: ?Sized>(&mut self, new_fallback_key: &Q) -> Option<&CommentTypes>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.map.get_without_fallback(new_fallback_key) {
            Some(pv_i) => {
                let v_i = *pv_i;
                if v_i != 0 {
                    self.comment_types[0] = self.comment_types[v_i].clone();
                    self.regexs[0] = None;
                }
                Some(&self.comment_types[0])
            }
            None => None,
        }
    }
}
