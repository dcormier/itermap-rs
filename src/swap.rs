use core::{fmt, iter::FusedIterator};

pub struct Swap<I> {
    iter: I,
}

impl<I> Swap<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }

    fn map_item<K, V>((k, v): (K, V)) -> (V, K) {
        (v, k)
    }
}

impl<I, K, V> Iterator for Swap<I>
where
    I: Iterator<Item = (K, V)>,
{
    type Item = (V, K);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Self::map_item)
    }
}

impl<I, K, V> DoubleEndedIterator for Swap<I>
where
    I: DoubleEndedIterator<Item = (K, V)>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Self::map_item)
    }
}

impl<I, K, V> ExactSizeIterator for Swap<I> where I: Iterator<Item = (K, V)> + ExactSizeIterator {}
impl<I, K, V> FusedIterator for Swap<I> where I: Iterator<Item = (K, V)> + FusedIterator {}

impl<I> fmt::Debug for Swap<I>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Swap").field("iter", &self.iter).finish()
    }
}

impl<I> Clone for Swap<I>
where
    I: Clone,
{
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

impl<I> Copy for Swap<I> where I: Copy {}
