use core::fmt;

/// For mapping keys and/or values from maps (`HashMap`, `BTreeMap`, etc).
pub struct Iter<I, F> {
    pub iter: I,
    pub op: F,
}

impl<I, F> fmt::Debug for Iter<I, F>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Iter")
            .field("iter", &self.iter)
            .finish_non_exhaustive()
    }
}
