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

mod iter;
mod map_keys;
mod map_values;
#[cfg(any(test, doctest))]
mod tests;

use core::iter::Iterator;

pub use self::{map_keys::MapKeys, map_values::MapValues};

/// Adds additional methods for `Iterator`s over maps (e.g., `HashMap`,
/// `BTreeMap`, etc.) and other two-element tuples (like `(K, V)`).
pub trait IterMap<I, K, V>: Sized {
    /// Maps map keys (or the first element of a two-element tuple like
    /// `(K, V)`), leaving other elements intact and untouched.
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

    /// Maps map values (or the second element of a two-element tuple like
    /// `(K, V)`), leaving other elements intact and untouched.
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

impl<I, K, V> IterMap<I, K, V> for I
where
    I: Iterator<Item = (K, V)>,
{
    fn map_keys<Fk, J>(self, key_op: Fk) -> MapKeys<I, Fk>
    where
        Fk: FnMut(K) -> J,
    {
        MapKeys::new(self, key_op)
    }

    fn map_values<Fv, U>(self, value_op: Fv) -> MapValues<I, Fv>
    where
        Fv: FnMut(V) -> U,
    {
        MapValues::new(self, value_op)
    }
}
