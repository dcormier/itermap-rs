use core::iter::{DoubleEndedIterator, FusedIterator, Iterator};

use crate::iter::Iter;

/// Filters keys, or the first element of a two-element tuple (like `(K, V)`).
///
/// For use, see the documentation on [`IterMap::filter_keys`].
///
/// [`IterMap::filter_keys`]: crate::IterMap::filter_keys
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct FilterKeys<I, F>(Iter<I, F>);

impl<I, F> FilterKeys<I, F> {
    pub(super) fn new<K, V, L>(iter: I, key_op: F) -> Self
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
        self.0.iter.next().filter(|(k, _v)| (self.0.op)(k))
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
        self.0.iter.next_back().filter(|(k, _v)| (self.0.op)(k))
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
