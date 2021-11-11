//! Tools for [`Iterator`]s over maps ([`HashMap`], [`BTreeMap`], etc),
//! or any two-element tuple.
//!
//! Just import [`IterMap`] to get extra methods on iterators.
//!
//! # Examples
//!
//! Mapping keys:
//! ```
//! # use std::collections::HashMap;
//! #
//! use itermap::IterMap;
//!
//! let mut hash: HashMap<&str, &str> = HashMap::new();
//! hash.insert("a", "A");
//! hash.insert("b", "B");
//!
//! let hash: HashMap<String, &str> = hash
//!     .into_iter()
//!     .map_keys(String::from)
//!     .collect();
//! ```
//!
//! Mapping values:
//! ```
//! # use std::collections::HashMap;
//! #
//! use itermap::IterMap;
//!
//! let mut hash: HashMap<&str, &str> = HashMap::new();
//! hash.insert("a", "A");
//! hash.insert("b", "B");
//!
//! let hash: HashMap<&str, String> = hash
//!     .into_iter()
//!     .map_values(String::from)
//!     .collect();
//! ```
//!
//! You can, of course, chain both adaptors to map both keys and values:
//! ```
//! # use std::collections::HashMap;
//! #
//! use itermap::IterMap;
//!
//! let mut hash: HashMap<&str, &str> = HashMap::new();
//! hash.insert("a", "A");
//! hash.insert("b", "B");
//!
//! let hash: HashMap<String, char> = hash
//!     .into_iter()
//!     .map_keys(String::from)
//!     .map_values(|v| v.parse().unwrap())
//!     .collect();
//! ```
//!
//! It works with any iterator over a two-element tuple:
//! ```
//! use itermap::IterMap;
//!
//! let mut values: Vec<(&str, &str)> = Vec::new();
//! values.push(("a", "A"));
//! values.push(("b", "B"));
//!
//! let values: Vec<(String, char)> = values
//!     .into_iter()
//!     .map_keys(String::from)
//!     .map_values(|v| v.parse().unwrap())
//!     .collect();
//! ```
//!
//! [`HashMap`]: std::collections::HashMap
//! [`BTreeMap`]: std::collections::BTreeMap

#[cfg(any(test, doctest))]
mod tests;

use core::{
    fmt,
    iter::{DoubleEndedIterator, FusedIterator, Iterator},
};

/// For mapping keys and/or values from maps (`HashMap`, `BTreeMap`, etc).
struct Iter<I, F> {
    iter: I,
    op: F,
}

impl<I, F> fmt::Debug for Iter<I, F>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("itermap")
            .field("iter", &self.iter)
            .finish_non_exhaustive()
    }
}

/// Maps keys (or the first element of a two-element tuple like `(K, V)`),
/// leaving other elements intact and untouched.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct MapKeys<I, F>(Iter<I, F>);

impl<I, F, K, J, V> Iterator for MapKeys<I, F>
where
    I: Iterator<Item = (K, V)>,
    F: FnMut(K) -> J,
{
    type Item = (J, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter.next().map(|(k, v)| ((self.0.op)(k), v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.iter.size_hint()
    }
}

impl<I, F, K, J, V> DoubleEndedIterator for MapKeys<I, F>
where
    I: DoubleEndedIterator<Item = (K, V)>,
    F: FnMut(K) -> J,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.iter.next_back().map(|(k, v)| ((self.0.op)(k), v))
    }
}

impl<I, F, K, J, V> FusedIterator for MapKeys<I, F>
where
    I: FusedIterator<Item = (K, V)>,
    F: FnMut(K) -> J,
{
}

/// Maps values (or the second element of a two-element tuple like `(K, V)`),
/// leaving other elements intact and untouched.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct MapValues<I, F>(Iter<I, F>);

impl<I, F, K, V, W> Iterator for MapValues<I, F>
where
    I: Iterator<Item = (K, V)>,
    F: FnMut(V) -> W,
{
    type Item = (K, W);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter.next().map(|(k, v)| (k, (self.0.op)(v)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.iter.size_hint()
    }
}

impl<I, F, K, V, W> DoubleEndedIterator for MapValues<I, F>
where
    I: DoubleEndedIterator<Item = (K, V)>,
    F: FnMut(V) -> W,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.iter.next_back().map(|(k, v)| (k, (self.0.op)(v)))
    }
}

impl<I, F, K, V, W> FusedIterator for MapValues<I, F>
where
    I: FusedIterator<Item = (K, V)>,
    F: FnMut(V) -> W,
{
}

/// Adds additional methods for `Iterator`s over maps (e.g., `HashMap`,
/// `BTreeMap`, etc.) and other two-element tuples (like `(K, V)`).
pub trait IterMap<I, K, V>: Sized {
    /// Maps keys (or the first element of a two-element tuple like `(K, V)`),
    /// leaving other elements intact and untouched.
    ///
    /// # Example
    /// ```
    /// # use std::collections::HashMap;
    /// #
    /// use itermap::IterMap;
    ///
    /// let mut hash: HashMap<&str, &str> = HashMap::new();
    /// hash.insert("a", "A");
    /// hash.insert("b", "B");
    ///
    /// let hash: HashMap<String, &str> = hash
    ///     .into_iter()
    ///     .map_keys(String::from)
    ///     .collect();
    /// ```
    fn map_keys<Fk, J>(self, f: Fk) -> MapKeys<I, Fk>
    where
        Fk: FnMut(K) -> J;

    /// Maps values (or the second element of a two-element tuple like `(K, V)`),
    /// leaving other elements intact and untouched.
    ///
    /// # Example
    /// ```
    /// # use std::collections::HashMap;
    /// #
    /// use itermap::IterMap;
    ///
    /// let mut hash: HashMap<&str, &str> = HashMap::new();
    /// hash.insert("a", "A");
    /// hash.insert("b", "B");
    ///
    /// let hash: HashMap<&str, String> = hash
    ///     .into_iter()
    ///     .map_values(String::from)
    ///     .collect();
    /// ```
    fn map_values<Fv, U>(self, f: Fv) -> MapValues<I, Fv>
    where
        Fv: FnMut(V) -> U;
}

fn map_keys<I, K, V, Fk, J>(iter: I, key_op: Fk) -> MapKeys<I, Fk>
where
    I: Iterator<Item = (K, V)>,
    Fk: FnMut(K) -> J,
{
    MapKeys(Iter { iter, op: key_op })
}

fn map_values<I, K, V, Fv, U>(iter: I, value_op: Fv) -> MapValues<I, Fv>
where
    I: Iterator<Item = (K, V)>,
    Fv: FnMut(V) -> U,
{
    MapValues(Iter { iter, op: value_op })
}

impl<I, K, V> IterMap<I, K, V> for I
where
    I: Iterator<Item = (K, V)>,
{
    fn map_keys<Fk, J>(self, key_op: Fk) -> MapKeys<I, Fk>
    where
        Fk: FnMut(K) -> J,
    {
        map_keys(self, key_op)
    }

    fn map_values<Fv, U>(self, value_op: Fv) -> MapValues<I, Fv>
    where
        Fv: FnMut(V) -> U,
    {
        map_values(self, value_op)
    }
}
