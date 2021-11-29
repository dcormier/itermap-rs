use core::iter::{DoubleEndedIterator, FusedIterator, Iterator};

use crate::iter::Iter;

/// Maps keys (or the first element of a two-element tuple like `(K, V)`),
/// leaving other elements intact and untouched.
///
/// For use, see the documentation on [`IterMap::map_keys`].
///
/// [`IterMap::map_keys`]: crate::IterMap::map_keys
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct MapKeys<I, F>(Iter<I, F>);

impl<I, F> MapKeys<I, F> {
    pub(super) fn new<K, V, J>(iter: I, key_op: F) -> Self
    where
        I: Iterator<Item = (K, V)>,
        F: FnMut(K) -> J,
    {
        Self(Iter { iter, op: key_op })
    }
}

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

impl<I, F, K, J, V> ExactSizeIterator for MapKeys<I, F>
where
    I: ExactSizeIterator<Item = (K, V)>,
    F: FnMut(K) -> J,
{
}
