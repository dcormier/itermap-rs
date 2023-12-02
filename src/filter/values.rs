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
pub struct FilterValues<I, P>(Iter<I, P>);

impl<I, P> FilterValues<I, P> {
    pub(crate) fn new<K, V>(iter: I, predicate: P) -> Self
    where
        I: Iterator<Item = (K, V)>,
        P: FnMut(&V) -> bool,
    {
        Self(Iter { iter, predicate })
    }
}

impl<I, P, K, V> Iterator for FilterValues<I, P>
where
    I: Iterator<Item = (K, V)>,
    P: FnMut(&V) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .iter
            .by_ref()
            .find(|item| (self.0.predicate)(&item.1))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.iter.size_hint()
    }
}

impl<I, P, K, V> DoubleEndedIterator for FilterValues<I, P>
where
    I: DoubleEndedIterator<Item = (K, V)>,
    P: FnMut(&V) -> bool,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .iter
            .by_ref()
            .rev()
            .find(|item| (self.0.predicate)(&item.1))
    }
}

impl<I, P, K, V> FusedIterator for FilterValues<I, P>
where
    I: FusedIterator<Item = (K, V)>,
    P: FnMut(&V) -> bool,
{
}

impl<I, P, K, V> ExactSizeIterator for FilterValues<I, P>
where
    I: ExactSizeIterator<Item = (K, V)>,
    P: FnMut(&V) -> bool,
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
