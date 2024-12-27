//! All implementations of [`StringExtT`] are here

use std::{rc::Rc, sync::Arc};

use crate::impl_for_ref;

use super::StringT;

pub mod array;
pub mod iterator;
pub mod string;
pub mod tuple;

impl StringT for () {
    #[inline]
    fn encode_to_buf(self, _string: &mut Vec<u8>) {}

    #[inline]
    fn encode_to_buf_with_separator(self, _string: &mut Vec<u8>, _separator: &str) {}

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf(self, _string: &mut bytes::BytesMut) {}

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf_with_separator(self, _string: &mut bytes::BytesMut, _separator: &str) {}
}

impl<T: StringT> StringT for Box<T> {
    #[inline]
    fn encode_to_buf(self, string: &mut Vec<u8>) {
        (*self).encode_to_buf(string);
    }

    #[inline]
    fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
        (*self).encode_to_buf_with_separator(string, separator);
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
        (*self).encode_to_bytes_buf(string);
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
        (*self).encode_to_bytes_buf_with_separator(string, separator);
    }
}

impl<T: StringT> StringT for Option<T> {
    #[inline]
    fn encode_to_buf(self, string: &mut Vec<u8>) {
        if let Some(inner) = self {
            inner.encode_to_buf(string);
        }
    }

    #[inline]
    fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
        if let Some(inner) = self {
            inner.encode_to_buf(string);
            string.extend_from_slice(separator.as_bytes());
        }
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
        if let Some(inner) = self {
            inner.encode_to_bytes_buf(string);
        }
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
        if let Some(inner) = self {
            inner.encode_to_bytes_buf_with_separator(string, separator);
            string.extend_from_slice(separator.as_bytes());
        }
    }
}

impl<T: StringT, E> StringT for Result<T, E> {
    #[inline]
    fn encode_to_buf(self, string: &mut Vec<u8>) {
        if let Ok(inner) = self {
            inner.encode_to_buf(string);
        }
    }

    #[inline]
    fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
        if let Ok(inner) = self {
            inner.encode_to_buf(string);
            string.extend_from_slice(separator.as_bytes());
        }
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
        if let Ok(inner) = self {
            inner.encode_to_bytes_buf(string);
        }
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
        if let Ok(inner) = self {
            inner.encode_to_bytes_buf_with_separator(string, separator);
            string.extend_from_slice(separator.as_bytes());
        }
    }
}

impl_for_ref!(REF: T => Arc<T> T => Rc<T>);
