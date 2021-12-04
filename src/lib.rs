#![feature(unboxed_closures)]
#![feature(fn_traits)]

mod idx;
mod idx_vec;

pub use idx::Idx;
pub use idx_vec::{Enumerated, IndexVec, IntoIdx};

#[cfg(not(feature = "serde"))]
#[macro_export]
macro_rules! newtype_index {
    ($($tt:tt)*) => {
        $crate::_newtype_index!($($tt)*);
    };
}

#[cfg(feature = "serde")]
#[macro_export]
macro_rules! newtype_index {
    ($($tt:tt)*) => {
        $crate::_newtype_index! {
            #[derive(serde::Serialize, serde::Deserialize)]
            #[serde(transparent)]
            $($tt)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _newtype_index {
    (
        $(#[$attrs:meta])*
        $vis:vis struct $type:ident
        {
            $( $const_vis:vis const $constant:ident = $value:expr; )*
        }
    ) => {
        #[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $(#[$attrs])*
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

        impl ::std::str::FromStr for $type {
            type Err = ::std::num::ParseIntError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse::<usize>().map($crate::Idx::new)
            }
        }
    };
    (
        $(#[$attrs:meta])*
        $vis:vis $type:ident
    ) => {
        $crate::_newtype_index! {
            $(#[$attrs])*
            $vis struct $type {}
        }
    };
}

#[cfg(test)]
mod tests;
