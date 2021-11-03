#![feature(unboxed_closures)]
#![feature(fn_traits)]

mod idx;
mod idx_vec;

pub use idx::Idx;
pub use idx_vec::{Enumerated, IndexVec, IntoIdx};

// Serde is not a feature for now as `#[cfg_attr(feature = "serde", derive(..)]`
// doesn't seem to work as intended.
pub use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! newtype_index {
    (
        $(#[$attrs:meta])*
        $vis:vis struct $type:ident {
            $( $const_vis:vis const $constant:ident = $value:expr; )*
        }
    ) => {
        $(#[$attrs])*
        #[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[derive($crate::Serialize, $crate::Deserialize)]
        $vis struct $type {
            idx: usize,
        }

        impl $type {
            $( $const_vis const $constant: $type = Self::const_new($value); )*
        }

        impl $type {
            #[allow(unused)]
            pub const fn const_new(idx: usize) -> Self {
                Self { idx }
            }
        }

        impl Clone for $type {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl $crate::Idx for $type {
            #[inline]
            fn new(idx: usize) -> Self {
                Self { idx }
            }

            #[inline]
            fn index(self) -> usize {
                self.idx
            }
        }
    };
    ($vis:vis $type:ident) => {
        $crate::newtype_index!($vis struct $type {});
    };
}

#[cfg(test)]
mod tests;
