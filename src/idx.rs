use std::fmt::Debug;
use std::hash::Hash;

pub trait Idx: Copy + Ord + Debug + Hash + Eq + 'static {
    fn new(idx: usize) -> Self;
    fn index(self) -> usize;
}

impl Idx for usize {
    #[inline]
    fn new(idx: usize) -> Self {
        idx
    }

    #[inline]
    fn index(self) -> usize {
        self
    }
}
