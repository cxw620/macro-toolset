//! Implementations for string.
//!
//! - &str
//! - &mut str
//! - &&str
//! - &mut &str
//! - &&mut str
//! - &mut &mut str
//! - Arc<str>
//! - Rc<str>
//! - Box<str>
//! - Cow<str>
//! - String
//! - &String
//! - &mut String
//! - &&String
//! - &mut &String

use std::{borrow::Cow, rc::Rc, sync::Arc};

use super::StringT;
use crate::impl_for_ref_copy;

macro_rules! impl_for_string {
    ($($ty:ty),*) => {
        $(
            // So ugly, but it works.
            impl_for_string!(INTERNAL $ty);
            impl_for_string!(INTERNAL &$ty);
            impl_for_string!(INTERNAL &mut $ty);
            impl_for_string!(INTERNAL &&$ty);
            impl_for_string!(INTERNAL &&mut $ty);
            impl_for_string!(INTERNAL &mut &$ty);
            impl_for_string!(INTERNAL &mut &mut $ty);
            impl_for_string!(INTERNAL &&&$ty);
        )*
    };

    (INTERNAL $ty:ty) => {
        impl StringT for $ty {
            #[inline]
            fn encode_to_buf(self, string: &mut Vec<u8>) {
                string.extend(self.as_bytes());
            }

            #[inline]
            fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, _separator: &str) {
                string.extend(self.as_bytes());
            }

            #[inline]
            #[cfg(feature = "feat-string-ext-bytes")]
            fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                string.extend(self.as_bytes());
            }

            #[inline]
            #[cfg(feature = "feat-string-ext-bytes")]
            fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, _separator: &str) {
                string.extend(self.as_bytes());
            }
        }
    }
}

impl_for_string! {
    &str,
    Arc<str>,
    Rc<str>,
    Box<str>,
    Cow<'_, str>,
    String
}

impl StringT for char {
    #[inline]
    fn encode_to_buf(self, string: &mut Vec<u8>) {
        // ! '�' != "�" in utf8 world. see [`String::push`].
        match self.len_utf8() {
            1 => string.push(self as u8),
            _ => string.extend(self.encode_utf8(&mut [0; 4]).as_bytes()),
        }
    }

    #[inline]
    fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, _separator: &str) {
        self.encode_to_buf(string);
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
        // ! '�' != "�" in utf8 world. see [`String::push`].
        match self.len_utf8() {
            1 => string.extend([self as u8]),
            _ => string.extend(self.encode_utf8(&mut [0; 4]).as_bytes()),
        }
    }

    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, _separator: &str) {
        self.encode_to_bytes_buf(string);
    }
}

impl_for_ref_copy!(char);
