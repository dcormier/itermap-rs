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
pub struct FilterKeys<I, P>(Iter<I, P>);

impl<I, P> FilterKeys<I, P> {
    pub(crate) fn new<K, V>(iter: I, predicate: P) -> Self
    where
        I: Iterator<Item = (K, V)>,
        P: FnMut(&K) -> bool,
    {
        Self(Iter { iter, predicate })
    }
}

impl<I, P, K, V> Iterator for FilterKeys<I, P>
where
    I: Iterator<Item = (K, V)>,
    P: FnMut(&K) -> bool,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .iter
            .by_ref()
            .find(|item| (self.0.predicate)(&item.0))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.iter.size_hint()
    }
}

impl<I, P, K, V> DoubleEndedIterator for FilterKeys<I, P>
where
    I: DoubleEndedIterator<Item = (K, V)>,
    P: FnMut(&K) -> bool,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .iter
            .by_ref()
            .rev()
            .find(|item| (self.0.predicate)(&item.0))
    }
}

impl<I, P, K, V> FusedIterator for FilterKeys<I, P>
where
    I: FusedIterator<Item = (K, V)>,
    P: FnMut(&K) -> bool,
{
}

impl<I, P, K, V> ExactSizeIterator for FilterKeys<I, P>
where
    I: ExactSizeIterator<Item = (K, V)>,
    P: FnMut(&K) -> bool,
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
