use core::iter::{DoubleEndedIterator, FusedIterator, Iterator};

use crate::iter::Iter;

/// Maps values (or the second element of a two-element tuple like `(K, V)`),
/// leaving other elements intact and untouched.
///
/// For use, see the documentation on [`IterMap::map_values`].
///
/// [`IterMap::map_values`]: crate::IterMap::map_values
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct MapValues<I, F>(Iter<I, F>);

impl<I, F> MapValues<I, F> {
    pub(super) fn new<K, V, W>(iter: I, value_op: F) -> Self
    where
        I: Iterator<Item = (K, V)>,
        F: FnMut(V) -> W,
    {
        Self(Iter { iter, op: value_op })
    }
}

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

impl<I, F, K, V, W> ExactSizeIterator for MapValues<I, F>
where
    I: ExactSizeIterator<Item = (K, V)>,
    F: FnMut(V) -> W,
{
}
