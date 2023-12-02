use core::fmt;

/// For mapping keys and/or values from maps (`HashMap`, `BTreeMap`, etc).
pub struct Iter<I, P> {
    pub iter: I,
    pub predicate: P,
}

impl<I, P> fmt::Debug for Iter<I, P>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Iter")
            .field("iter", &self.iter)
            .finish_non_exhaustive()
    }
}
