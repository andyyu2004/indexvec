use crate::Idx;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut, Range, RangeBounds};
use std::{iter, slice, vec};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IndexVec<I: Idx, T> {
    pub raw: Vec<T>,
    _marker: PhantomData<I>,
}

impl<I: Idx, T> Default for IndexVec<I, T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Idx, T> IndexVec<I, T> {
    #[inline]
    pub fn new() -> Self {
        IndexVec {
            raw: Vec::new(),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn from_raw(raw: Vec<T>) -> Self {
        IndexVec {
            raw,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        IndexVec {
            raw: Vec::with_capacity(capacity),
            _marker: PhantomData,
        }
    }

    /// Create an `IndexVec` with `n` elements, where the value of each
    /// element is the result of `func(i)`. (The underlying vector will
    /// be allocated only once, with a capacity of at least `n`.)
    #[inline]
    pub fn from_fn_n(func: impl FnMut(I) -> T, n: usize) -> Self {
        let indices = (0..n).map(I::new);
        Self::from_raw(indices.map(func).collect())
    }

    #[inline]
    pub fn push(&mut self, d: T) -> I {
        let idx = I::new(self.len());
        self.raw.push(d);
        idx
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.raw.pop()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    /// Gives the next index that will be assigned when `push` is
    /// called.
    #[inline]
    pub fn next_index(&self) -> I {
        I::new(self.len())
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    #[inline]
    pub fn into_iter(self) -> vec::IntoIter<T> {
        self.raw.into_iter()
    }

    #[inline]
    pub fn into_iter_enumerated(self) -> Enumerated<I, vec::IntoIter<T>> {
        self.raw.into_iter().enumerate().map(IntoIdx {
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.raw.iter()
    }

    #[inline]
    pub fn iter_enumerated(&self) -> Enumerated<I, slice::Iter<'_, T>> {
        self.raw.iter().enumerate().map(IntoIdx {
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn indices(&self) -> iter::Map<Range<usize>, IntoIdx<I>> {
        (0..self.len()).map(IntoIdx {
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.raw.iter_mut()
    }

    #[inline]
    pub fn iter_enumerated_mut(&mut self) -> Enumerated<I, slice::IterMut<'_, T>> {
        self.raw.iter_mut().enumerate().map(IntoIdx {
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn drain<'a, R: RangeBounds<usize>>(
        &'a mut self,
        range: R,
    ) -> impl Iterator<Item = T> + 'a {
        self.raw.drain(range)
    }

    #[inline]
    pub fn drain_enumerated<'a, R: RangeBounds<usize>>(
        &'a mut self,
        range: R,
    ) -> impl Iterator<Item = (I, T)> + 'a {
        self.raw.drain(range).enumerate().map(IntoIdx {
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn last(&self) -> Option<I> {
        self.len().checked_sub(1).map(I::new)
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.raw.shrink_to_fit()
    }

    #[inline]
    pub fn swap(&mut self, a: I, b: I) {
        self.raw.swap(a.index(), b.index())
    }

    #[inline]
    pub fn truncate(&mut self, a: usize) {
        self.raw.truncate(a)
    }

    #[inline]
    pub fn get(&self, index: I) -> Option<&T> {
        self.raw.get(index.index())
    }

    #[inline]
    pub fn get_mut(&mut self, index: I) -> Option<&mut T> {
        self.raw.get_mut(index.index())
    }

    pub fn convert_index_type<Ix: Idx>(self) -> IndexVec<Ix, T> {
        IndexVec {
            raw: self.raw,
            _marker: PhantomData,
        }
    }

    /// Grows the index vector so that it contains an entry for
    /// `elem`; if that is already true, then has no
    /// effect. Otherwise, inserts new values as needed by invoking
    /// `fill_value`.
    #[inline]
    pub fn ensure_contains_elem(&mut self, elem: I, fill_value: impl FnMut() -> T) {
        let min_new_len = elem.index() + 1;
        if self.len() < min_new_len {
            self.raw.resize_with(min_new_len, fill_value);
        }
    }

    #[inline]
    pub fn resize_to_elem(&mut self, elem: I, fill_value: impl FnMut() -> T) {
        let min_new_len = elem.index() + 1;
        self.raw.resize_with(min_new_len, fill_value);
    }
}

impl<I: Idx, T> Index<I> for IndexVec<I, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &T {
        &self.raw[index.index()]
    }
}

impl<I: Idx, T> IndexMut<I> for IndexVec<I, T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut T {
        &mut self.raw[index.index()]
    }
}

pub type Enumerated<I, J> = iter::Map<iter::Enumerate<J>, IntoIdx<I>>;

pub struct IntoIdx<I: Idx> {
    _marker: PhantomData<fn(I)>,
}

impl<I: Idx, T> FnOnce<((usize, T),)> for IntoIdx<I> {
    type Output = (I, T);

    extern "rust-call" fn call_once(self, ((n, t),): ((usize, T),)) -> Self::Output {
        (I::new(n), t)
    }
}

impl<I: Idx, T> FnMut<((usize, T),)> for IntoIdx<I> {
    extern "rust-call" fn call_mut(&mut self, ((n, t),): ((usize, T),)) -> Self::Output {
        (I::new(n), t)
    }
}

impl<I: Idx> FnOnce<(usize,)> for IntoIdx<I> {
    type Output = I;

    extern "rust-call" fn call_once(self, (n,): (usize,)) -> Self::Output {
        I::new(n)
    }
}

impl<I: Idx> FnMut<(usize,)> for IntoIdx<I> {
    extern "rust-call" fn call_mut(&mut self, (n,): (usize,)) -> Self::Output {
        I::new(n)
    }
}
