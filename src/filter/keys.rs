use core::iter::{DoubleEndedIterator, FusedIterator, Iterator};

use crate::iter::Iter;

/// Filters items based on the keys, the first element of a two-element tuple
/// (like `(K, V)`).
///
/// See: [`IterMap::filter_keys`]
///
/// [`IterMap::filter_keys`]: crate::IterMap::filter_keys
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct FilterKeys<I, F>(Iter<I, F>);

impl<I, F> FilterKeys<I, F> {
    pub(crate) fn new<K, V>(iter: I, key_op: F) -> Self
    where
        I: Iterator<Item = (K, V)>,
        F: FnMut(&K) -> bool,
    {
        Self(Iter { iter, op: key_op })
    }
}

impl<I, F, K, V> Iterator for FilterKeys<I, F>
where
    I: Iterator<Item = (K, V)>,
    F: FnMut(&K) -> bool,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        for (k, v) in self.0.iter.by_ref() {
            if (self.0.op)(&k) {
                return Some((k, v));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.iter.size_hint()
    }
}

impl<I, F, K, V> DoubleEndedIterator for FilterKeys<I, F>
where
    I: DoubleEndedIterator<Item = (K, V)>,
    F: FnMut(&K) -> bool,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        for (k, v) in self.0.iter.by_ref().rev() {
            if (self.0.op)(&k) {
                return Some((k, v));
            }
        }

        None
    }
}

impl<I, F, K, V> FusedIterator for FilterKeys<I, F>
where
    I: FusedIterator<Item = (K, V)>,
    F: FnMut(&K) -> bool,
{
}

impl<I, F, K, V> ExactSizeIterator for FilterKeys<I, F>
where
    I: ExactSizeIterator<Item = (K, V)>,
    F: FnMut(&K) -> bool,
{
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::FilterKeys;

    fn map() -> BTreeMap<char, char> {
        [('A', 'a'), ('B', 'b'), ('C', 'c'), ('D', 'd'), ('E', 'e')].into()
    }

    #[test]
    fn iter() {
        let iter = map().into_iter();

        let filtered: BTreeMap<_, _> = FilterKeys::new(iter, |k| k != &'C').collect();

        let mut expected = map();
        expected.remove(&'C');

        assert_eq!(expected, filtered);
    }

    #[test]
    fn double_ended_iterator() {
        let iter = map().into_iter();

        let filtered: BTreeMap<_, _> = FilterKeys::new(iter, |k| k != &'C').rev().collect();

        let mut expected = map();
        expected.remove(&'C');

        assert_eq!(expected, filtered);
    }
}
