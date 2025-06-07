#![allow(missing_docs)]

/*!
This module provides a `Map` abstraction that picks between HashMap and
BTreeMap depending on if `std` is available or not.
*/

use core::{borrow::Borrow, hash::Hash};

#[cfg(feature = "std")]
type Table<K, V> =
    std::collections::HashMap<K, V, foldhash::fast::RandomState>;
#[cfg(not(feature = "std"))]
type Table<K, V> = alloc::collections::BTreeMap<K, V>;

/// The `Map` type is a thin wrapper around either a `HashMap` or a `BTreeMap`
/// depending on the build configuration.
#[derive(Debug, Clone)]
pub struct Map<K, V> {
    table: Table<K, V>,
}

impl<K, V> Map<K, V>
where
    K: Eq + Ord + Hash,
{
    pub fn new() -> Self {
        Map { table: Table::default() }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.table.insert(key, value)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq + Ord + Hash,
    {
        self.table.get(key)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq + Ord + Hash,
    {
        self.table.remove(key)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq + Ord + Hash,
    {
        self.table.contains_key(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.table.iter()
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}

impl<K, V> Default for Map<K, V>
where
    K: Eq + Ord + Hash,
{
    fn default() -> Self {
        Map::new()
    }
}

impl<'m, K, V> IntoIterator for &'m Map<K, V> {
    type Item = (&'m K, &'m V);
    type IntoIter = <&'m Table<K, V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.table.iter()
    }
}

impl<K, V> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = <Table<K, V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.table.into_iter()
    }
}
