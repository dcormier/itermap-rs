use core::iter::{DoubleEndedIterator, FusedIterator, Iterator};

use crate::iter::Iter;

/// Maps keys, or the first element of a two-element tuple (like `(K, V)`),
/// leaving the other element intact and untouched.
///
/// For use, see the documentation on [`IterMap::map_keys`].
///
/// [`IterMap::map_keys`]: crate::IterMap::map_keys
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct MapKeys<I, P>(Iter<I, P>);

impl<I, P> MapKeys<I, P> {
    pub(crate) fn new<K, V, L>(iter: I, predicate: P) -> Self
    where
        I: Iterator<Item = (K, V)>,
        P: FnMut(K) -> L,
    {
        Self(Iter { iter, predicate })
    }
}

impl<I, P, K, L, V> Iterator for MapKeys<I, P>
where
    I: Iterator<Item = (K, V)>,
    P: FnMut(K) -> L,
{
    type Item = (L, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter.next().map(|(k, v)| ((self.0.predicate)(k), v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.iter.size_hint()
    }
}

impl<I, P, K, L, V> DoubleEndedIterator for MapKeys<I, P>
where
    I: DoubleEndedIterator<Item = (K, V)>,
    P: FnMut(K) -> L,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .iter
            .next_back()
            .map(|(k, v)| ((self.0.predicate)(k), v))
    }
}

impl<I, P, K, L, V> FusedIterator for MapKeys<I, P>
where
    I: FusedIterator<Item = (K, V)>,
    P: FnMut(K) -> L,
{
}

impl<I, P, K, L, V> ExactSizeIterator for MapKeys<I, P>
where
    I: ExactSizeIterator<Item = (K, V)>,
    P: FnMut(K) -> L,
{
}
