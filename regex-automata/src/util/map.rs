#![allow(missing_docs)]

/*!
This module provides a `Map` abstraction that picks between HashMap and
BTreeMap depending on if `std` is available or not.
*/

use core::{borrow::Borrow, hash::Hash};

#[cfg(feature = "std")]
mod hash {
    use core::hash::BuildHasher;

    #[derive(Debug, Clone, Copy)]
    pub struct RandomState(foldhash::fast::FixedState);

    impl BuildHasher for RandomState {
        type Hasher = foldhash::fast::FoldHasher;

        #[inline]
        fn build_hasher(&self) -> Self::Hasher {
            self.0.build_hasher()
        }
    }

    impl Default for RandomState {
        #[inline]
        fn default() -> Self {
            RandomState(foldhash::fast::FixedState::default())
        }
    }
}

#[cfg(feature = "std")]
type Table<K, V> = std::collections::HashMap<K, V, hash::RandomState>;
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
    #[inline]
    pub fn new() -> Self {
        Map { table: Table::default() }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Map {
            #[cfg(feature = "std")]
            table: Table::with_capacity_and_hasher(
                capacity,
                hash::RandomState::default(),
            ),
            #[cfg(not(feature = "std"))]
            table: Table::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.table.insert(key, value)
    }

    #[inline]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq + Ord + Hash,
    {
        self.table.get(key)
    }

    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq + Ord + Hash,
    {
        self.table.remove(key)
    }

    #[inline]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Eq + Ord + Hash,
    {
        self.table.contains_key(key)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.table.iter()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.table.clear();
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.table.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}

impl<K, V> Default for Map<K, V>
where
    K: Eq + Ord + Hash,
{
    #[inline]
    fn default() -> Self {
        Map::new()
    }
}

impl<'m, K, V> IntoIterator for &'m Map<K, V> {
    type Item = (&'m K, &'m V);
    type IntoIter = <&'m Table<K, V> as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.table.iter()
    }
}

impl<K, V> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = <Table<K, V> as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.table.into_iter()
    }
}
