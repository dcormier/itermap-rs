//! Tools for [`Iterator`]s over maps ([`HashMap`], [`BTreeMap`], etc),
//! or any two-element tuple (like `(K, V)`).
//!
//! Just import [`IterMap`] to get extra methods on iterators. See that trait
//! for more documentation and examples.
//!
//! [`HashMap`]: std::collections::HashMap
//! [`BTreeMap`]: std::collections::BTreeMap

mod filter;
mod iter;
mod map_keys;
mod map_values;
#[cfg(any(test, doctest))]
mod tests;

use core::iter::Iterator;

#[doc(hidden)]
pub use self::filter::{FilterKeys, FilterValues};
#[doc(hidden)]
pub use self::{map_keys::MapKeys, map_values::MapValues};

/// Adds additional methods for `Iterator`s over maps (e.g., `HashMap`,
/// `BTreeMap`, etc.) and other two-element tuples (like `(K, V)`).
pub trait IterMap<I, K, V>: Sized {
    /// Maps map keys, or the first element of a two-element tuple (like
    /// `(K, V)`), leaving the other element intact and untouched.
    ///
    /// # Examples
    /// ```
    /// # use std::collections::HashMap;
    /// #
    /// use itermap::IterMap;
    ///
    /// let map = HashMap::<&str, &str>::from([
    ///     ("a", "A"),
    ///     ("b", "B"),
    /// ]);
    ///
    /// let map: HashMap<String, &str> = map
    ///     .into_iter()
    ///     .map_keys(String::from)
    ///     .collect();
    /// ```
    ///
    /// Any iterator of two-element tuples will work.
    /// ```
    /// use itermap::IterMap;
    ///
    /// let items: Vec<(&str, &str)> = vec![
    ///     ("a", "A"),
    ///     ("b", "B"),
    /// ];
    ///
    /// let items: Vec<(String, &str)> = items
    ///     .into_iter()
    ///     .map_keys(String::from)
    ///     .collect();
    /// ```
    fn map_keys<Fk, L>(self, f: Fk) -> MapKeys<I, Fk>
    where
        Fk: FnMut(K) -> L;

    /// Maps map values (or the second element of a two-element tuple like
    /// `(K, V)`), leaving the other element intact and untouched.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// #
    /// use itermap::IterMap;
    ///
    /// let map = HashMap::<&str, &str>::from([
    ///     ("a", "A"),
    ///     ("b", "B"),
    /// ]);
    ///
    /// let map: HashMap<&str, String> = map
    ///     .into_iter()
    ///     .map_values(String::from)
    ///     .collect();
    /// ```
    ///
    /// Any iterator of two-element tuples will work.
    /// ```
    /// use itermap::IterMap;
    ///
    /// let items: Vec<(&str, &str)> = vec![
    ///     ("a", "A"),
    ///     ("b", "B"),
    /// ];
    ///
    /// let items: Vec<(&str, String)> = items
    ///     .into_iter()
    ///     .map_values(String::from)
    ///     .collect();
    /// ```
    fn map_values<Fv, W>(self, f: Fv) -> MapValues<I, Fv>
    where
        Fv: FnMut(V) -> W;

    /// Allows filtering based on map keys (or the first element of a
    /// two-element tuple like `(K, V)`).
    ///
    /// # Example
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// #
    /// use itermap::IterMap;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let mut map = HashMap::from([
    ///     ("a", "A"),
    ///     ("b", "B"),
    ///     ("c", "C"),
    ///     ("d", "D"),
    ///     ("e", "E"),
    /// ]);
    ///
    /// let filtered: HashMap<_, _> = map
    ///     .clone()
    ///     .into_iter()
    ///     .filter_keys(|v| v != &"c")
    ///     .collect();
    ///
    /// map.remove("c");
    ///
    /// assert_eq!(map, filtered);
    /// ```
    ///
    /// Any iterator of two-element tuples will work.
    /// ```
    /// use itermap::IterMap;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let mut items = vec![
    ///     ("a", "A"),
    ///     ("b", "B"),
    ///     ("c", "C"),
    ///     ("d", "D"),
    ///     ("e", "E"),
    /// ];
    ///
    /// let filtered: Vec<(_, _)> = items
    ///     .clone()
    ///     .into_iter()
    ///     .filter_keys(|v| v != &"c")
    ///     .collect();
    ///
    /// items.remove(2);
    ///
    /// assert_eq!(items, filtered);
    /// ```
    fn filter_keys<Fk>(self, key_op: Fk) -> FilterKeys<I, Fk>
    where
        Fk: FnMut(&K) -> bool;

    /// Allows filtering based on map values (or the second element of a
    /// two-element tuple like `(K, V)`).
    ///
    /// # Example
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// #
    /// use itermap::IterMap;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let mut map = HashMap::from([
    ///     ("a", "A"),
    ///     ("b", "B"),
    ///     ("c", "C"),
    ///     ("d", "D"),
    ///     ("e", "E"),
    /// ]);
    ///
    /// let filtered: HashMap<_, _> = map
    ///     .clone()
    ///     .into_iter()
    ///     .filter_values(|v| v != &"C")
    ///     .collect();
    ///
    /// map.remove("c");
    ///
    /// assert_eq!(map, filtered);
    /// ```
    ///
    /// Any iterator of two-element tuples will work.
    /// ```
    /// use itermap::IterMap;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let mut items = vec![
    ///     ("a", "A"),
    ///     ("b", "B"),
    ///     ("c", "C"),
    ///     ("d", "D"),
    ///     ("e", "E"),
    /// ];
    ///
    /// let filtered: Vec<(_, _)> = items
    ///     .clone()
    ///     .into_iter()
    ///     .filter_values(|v| v != &"C")
    ///     .collect();
    ///
    /// items.remove(2);
    ///
    /// assert_eq!(items, filtered);
    /// ```
    fn filter_values<Fv>(self, value_op: Fv) -> FilterValues<I, Fv>
    where
        Fv: FnMut(&V) -> bool;
}

impl<I, K, V> IterMap<I, K, V> for I
where
    I: Iterator<Item = (K, V)>,
{
    fn map_keys<Fk, L>(self, key_op: Fk) -> MapKeys<I, Fk>
    where
        Fk: FnMut(K) -> L,
    {
        MapKeys::new(self, key_op)
    }

    fn map_values<Fv, W>(self, value_op: Fv) -> MapValues<I, Fv>
    where
        Fv: FnMut(V) -> W,
    {
        MapValues::new(self, value_op)
    }

    fn filter_keys<Fk>(self, key_op: Fk) -> FilterKeys<I, Fk>
    where
        Fk: FnMut(&K) -> bool,
    {
        FilterKeys::new(self, key_op)
    }

    fn filter_values<Fv>(self, value_op: Fv) -> FilterValues<I, Fv>
    where
        Fv: FnMut(&V) -> bool,
    {
        FilterValues::new(self, value_op)
    }
}
