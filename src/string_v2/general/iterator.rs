//! Implementations for any `Iterator`.
//!
//! # Concepts
//!
//! Items of the iterator must implement `StringT` and are all considered
//! **independent** (like what we do for tuple, hmmm, tuple with fixed length
//! and with items that are all with the same type?).
//!
//! Since the compiler will complain that *upstream crates may add a new impl
//! of trait [`std::iter::Iterator`] for type `{I}`*, we cannot simply implement
//! `StringT` for `T` where `T`: `{Trait}` but concrete one instead.
//!
//! This crate has already implemented `StringT` for the following types:
//!
//! - [`std::iter::Map`]
//!
//! For array-or-slice-like types:
//!
//! - `Vec<T>`
//! - `[T; N]`
//! - `&[T]` or `&[T; N]`
//!
//!   Only when &T implements `StringT`.
//!   We have implemented `StringT` for most `&T` where T: Copy, though best
//!   effort.

use super::{StringExtT, StringT};
use crate::wrapper;

wrapper! {
    #[derive(Debug)]
    /// Wrapper for any iterator with items implementing `StringT`.

    pub IterWrapper<I>(pub I)
}

#[macro_export]
/// See [`IterWrapper`].
macro_rules! str_iter_wrapper {
    ($inner:expr) => {
        $crate::string_v2::general::iterator::IterWrapper { inner: $inner }
    };
}

impl<I> StringT for IterWrapper<I>
where
    I: Iterator,
    I::Item: StringT,
{
    #[inline]
    fn encode_to_buf(self, string: &mut Vec<u8>) {
        for item in self.inner {
            item.encode_to_buf(string);
        }
    }

    #[inline]
    fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
        for item in self.inner {
            item.encode_to_buf_with_separator(string, separator);
        }
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
        for item in self.inner {
            item.encode_to_bytes_buf(string);
        }
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
        for item in self.inner {
            item.encode_to_bytes_buf_with_separator(string, separator);
        }
    }
}

impl<I> StringExtT for IterWrapper<I>
where
    I: Iterator,
    I::Item: StringT,
{
    #[inline]
    fn with_prefix<P: StringExtT + Copy>(self, prefix: P) -> impl StringExtT {
        self.inner.map(move |item| super::tuple::SeplessTuple {
            inner: (prefix, item),
        })
    }

    #[inline]
    fn with_suffix<S: StringExtT + Copy>(self, suffix: S) -> impl StringExtT {
        self.inner.map(move |item| super::tuple::SeplessTuple {
            inner: (item, suffix),
        })
    }
}

impl<T, I, F> StringT for std::iter::Map<I, F>
where
    T: StringT,
    I: Iterator,
    F: FnMut(I::Item) -> T,
{
    #[inline]
    fn encode_to_buf(self, string: &mut Vec<u8>) {
        str_iter_wrapper!(self.into_iter()).encode_to_buf(string);
    }

    #[inline]
    fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
        str_iter_wrapper!(self.into_iter()).encode_to_buf_with_separator(string, separator);
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
        str_iter_wrapper!(self.into_iter()).encode_to_bytes_buf(string);
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
        str_iter_wrapper!(self.into_iter()).encode_to_bytes_buf_with_separator(string, separator);
    }
}

impl<T, I, F> StringExtT for std::iter::Map<I, F>
where
    T: StringT,
    I: Iterator,
    F: FnMut(I::Item) -> T,
{
    #[inline]
    fn with_prefix<P: StringExtT + Copy>(self, prefix: P) -> impl StringExtT {
        self.into_iter()
            .map(move |item| super::tuple::SeplessTuple {
                inner: (prefix, item),
            })
    }

    #[inline]
    fn with_suffix<S: StringExtT + Copy>(self, suffix: S) -> impl StringExtT {
        self.into_iter()
            .map(move |item| super::tuple::SeplessTuple {
                inner: (item, suffix),
            })
    }
}

// ==============================================================================================

macro_rules! impl_for_array_or_slice_like {
    (STRING_T: $($tt:tt)*) => {
        $($tt)* {
            #[inline]
            fn encode_to_buf(self, string: &mut Vec<u8>) {
                str_iter_wrapper!(self.into_iter()).encode_to_buf(string);
            }

            #[inline]
            fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                str_iter_wrapper!(self.into_iter()).encode_to_buf_with_separator(string, separator);
            }

            #[inline]
            #[cfg(feature = "feat-string-ext-bytes")]
            fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                str_iter_wrapper!(self.into_iter()).encode_to_bytes_buf(string);
            }

            #[inline]
            #[cfg(feature = "feat-string-ext-bytes")]
            fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                str_iter_wrapper!(self.into_iter()).encode_to_bytes_buf_with_separator(string, separator);
            }
        }
    };
    (STRING_EXT_T: $($tt:tt)*) => {
        $($tt)* {
            #[inline]
            fn with_prefix<P: StringExtT + Copy>(self, prefix: P) -> impl StringExtT {
                self.into_iter()
                    .map(move |item| super::tuple::SeplessTuple {
                        inner: (prefix, item),
                    })
            }

            #[inline]
            fn with_suffix<S: StringExtT + Copy>(self, suffix: S) -> impl StringExtT {
                self.into_iter()
                    .map(move |item| super::tuple::SeplessTuple {
                        inner: (item, suffix),
                    })
            }
        }
    };
}

impl_for_array_or_slice_like!(STRING_T: impl<T, const N: usize> StringT for [T; N] where T: StringT);
impl_for_array_or_slice_like!(STRING_EXT_T: impl<T, const N: usize> StringExtT for [T; N] where T: StringT);

impl_for_array_or_slice_like!(STRING_T: impl<T, const N: usize> StringT for &[T; N] where for<'a> &'a T: StringT);
impl_for_array_or_slice_like!(STRING_EXT_T: impl<T, const N: usize> StringExtT for &[T; N] where for<'a> &'a T: StringT);

impl_for_array_or_slice_like!(STRING_T: impl<T> StringT for &[T] where for<'a> &'a T: StringT);
impl_for_array_or_slice_like!(STRING_EXT_T: impl<T> StringExtT for &[T] where for<'a> &'a T: StringT);

impl_for_array_or_slice_like!(STRING_T: impl<T> StringT for Vec<T> where T: StringT);
impl_for_array_or_slice_like!(STRING_EXT_T: impl<T> StringExtT for Vec<T> where T: StringT);
