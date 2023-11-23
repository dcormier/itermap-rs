use core::iter::{DoubleEndedIterator, FusedIterator, Iterator};

use crate::iter::Iter;

/// Maps values, or the second element of a two-element tuple (like `(K, V)`)
///
/// For use, see the documentation on [`IterMap::filter_values`].
///
/// [`IterMap::filter_values`]: crate::IterMap::filter_values
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct FilterValues<I, F>(Iter<I, F>);

impl<I, F> FilterValues<I, F> {
    pub(super) fn new<K, V>(iter: I, value_op: F) -> Self
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
        self.0.iter.next().filter(|(_k, v)| (self.0.op)(v))
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
        self.0.iter.next_back().filter(|(_k, v)| (self.0.op)(v))
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
