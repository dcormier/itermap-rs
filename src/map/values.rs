use core::iter::{DoubleEndedIterator, FusedIterator, Iterator};

use crate::iter::Iter;

/// Maps values, or the second element of a two-element tuple (like `(K, V)`),
/// leaving the other element intact and untouched.
///
/// For use, see the documentation on [`IterMap::map_values`].
///
/// [`IterMap::map_values`]: crate::IterMap::map_values
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct MapValues<I, P>(Iter<I, P>);

impl<I, P> MapValues<I, P> {
    pub(crate) fn new<K, V, W>(iter: I, predicate: P) -> Self
    where
        I: Iterator<Item = (K, V)>,
        P: FnMut(V) -> W,
    {
        Self(Iter { iter, predicate })
    }
}

impl<I, P, K, V, W> Iterator for MapValues<I, P>
where
    I: Iterator<Item = (K, V)>,
    P: FnMut(V) -> W,
{
    type Item = (K, W);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter.next().map(|(k, v)| (k, (self.0.predicate)(v)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.iter.size_hint()
    }
}

impl<I, P, K, V, W> DoubleEndedIterator for MapValues<I, P>
where
    I: DoubleEndedIterator<Item = (K, V)>,
    P: FnMut(V) -> W,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .iter
            .next_back()
            .map(|(k, v)| (k, (self.0.predicate)(v)))
    }
}

impl<I, P, K, V, W> FusedIterator for MapValues<I, P>
where
    I: FusedIterator<Item = (K, V)>,
    P: FnMut(V) -> W,
{
}

impl<I, P, K, V, W> ExactSizeIterator for MapValues<I, P>
where
    I: ExactSizeIterator<Item = (K, V)>,
    P: FnMut(V) -> W,
{
}
