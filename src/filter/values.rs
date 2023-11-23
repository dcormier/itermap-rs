use core::iter::{DoubleEndedIterator, FusedIterator, Iterator};

use crate::iter::Iter;

/// Filters items based on the values, the second element of a two-element tuple
/// (like `(K, V)`).
///
/// See: [`IterMap::filter_values`]
///
/// [`IterMap::filter_values`]: crate::IterMap::filter_values
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct FilterValues<I, F>(Iter<I, F>);

impl<I, F> FilterValues<I, F> {
    pub(crate) fn new<K, V>(iter: I, value_op: F) -> Self
    where
        I: Iterator<Item = (K, V)>,
        F: FnMut(&V) -> bool,
    {
        Self(Iter { iter, op: value_op })
    }
}

impl<I, F, K, V> Iterator for FilterValues<I, F>
where
    I: Iterator<Item = (K, V)>,
    F: FnMut(&V) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        for (k, v) in self.0.iter.by_ref() {
            if (self.0.op)(&v) {
                return Some((k, v));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.iter.size_hint()
    }
}

impl<I, F, K, V> DoubleEndedIterator for FilterValues<I, F>
where
    I: DoubleEndedIterator<Item = (K, V)>,
    F: FnMut(&V) -> bool,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        for (k, v) in self.0.iter.by_ref().rev() {
            if (self.0.op)(&v) {
                return Some((k, v));
            }
        }

        None
    }
}

impl<I, F, K, V> FusedIterator for FilterValues<I, F>
where
    I: FusedIterator<Item = (K, V)>,
    F: FnMut(&V) -> bool,
{
}

impl<I, F, K, V> ExactSizeIterator for FilterValues<I, F>
where
    I: ExactSizeIterator<Item = (K, V)>,
    F: FnMut(&V) -> bool,
{
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::FilterValues;

    fn map() -> BTreeMap<char, char> {
        [('A', 'a'), ('B', 'b'), ('C', 'c'), ('D', 'd'), ('E', 'e')].into()
    }

    #[test]
    fn iter() {
        let iter = map().into_iter();

        let filtered: BTreeMap<_, _> = FilterValues::new(iter, |v| v != &'c').collect();

        let mut expected = map();
        expected.remove(&'C');

        assert_eq!(expected, filtered);
    }

    #[test]
    fn double_ended_iterator() {
        let iter = map().into_iter();

        let filtered: BTreeMap<_, _> = FilterValues::new(iter, |v| v != &'c').rev().collect();

        let mut expected = map();
        expected.remove(&'C');

        assert_eq!(expected, filtered);
    }
}
