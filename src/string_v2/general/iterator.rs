//! Implementations for iterator type except `Vec`, etc.
//!
//! # Examples
//!
//! - `std::iter::Map`

use super::StringT;
use crate::{remove_separator_tailing, wrapper};

wrapper! {
    #[derive(Debug)]
    /// Iterator wrapper for `StringT`.
    ///
    /// Since the compiler will complain that *upstream crates may add a new impl
    /// of trait [`std::iter::Iterator`] for type `{I}`*, we cannot simply implement
    /// `StringT` for `T` where `T`: `{Trait}` but concrete type instead.
    ///
    /// This crate has already implemented `StringT` for [`std::iter::Map`].
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
    fn encode_to_buf_with_separator(mut self, string: &mut Vec<u8>, separator: &str) {
        let first_item = self.inner.next();

        if let Some(item) = first_item {
            item.encode_to_buf_with_separator(string, separator);
            string.extend(separator.as_bytes());

            // The lefted items.
            for item in self.inner {
                item.encode_to_buf_with_separator(string, separator);
                string.extend(separator.as_bytes());
            }

            remove_separator_tailing!(string, separator);
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
    fn encode_to_bytes_buf_with_separator(mut self, string: &mut bytes::BytesMut, separator: &str) {
        let first_item = self.inner.next();

        if let Some(item) = first_item {
            item.encode_to_bytes_buf_with_separator(string, separator);
            string.extend(separator.as_bytes());

            // The lefted items.
            for item in self.inner {
                item.encode_to_bytes_buf_with_separator(string, separator);
                string.extend(separator.as_bytes());
            }

            remove_separator_tailing!(string, separator);
        }
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
