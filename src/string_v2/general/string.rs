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

use std::{borrow::Cow, ops::Deref, rc::Rc, sync::Arc};

use super::{StringExtT, StringT};
use crate::{impl_for_shared_ref, impl_for_wrapper, wrapper};

macro_rules! impl_for_string {
    ($($ty:ty),*) => {
        $(
            // So ugly, but it works.
            impl_for_string!(@INTERNAL $ty);
            impl_for_string!(@INTERNAL &$ty);
            impl_for_string!(@INTERNAL &mut $ty);
            impl_for_string!(@INTERNAL &&$ty);
            impl_for_string!(@INTERNAL &&mut $ty);
            impl_for_string!(@INTERNAL &mut &$ty);
            impl_for_string!(@INTERNAL &mut &mut $ty);
            impl_for_string!(@INTERNAL &&&$ty);
        )*
    };

    (@INTERNAL $ty:ty) => {
        impl StringT for $ty {
            #[inline]
            fn encode_to_buf(self, string: &mut Vec<u8>) {
                string.extend(self.as_bytes());
            }

            #[inline]
            fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                string.extend(self.as_bytes());
                string.extend(separator.as_bytes());
            }

            #[inline]
            #[cfg(feature = "feat-string-ext-bytes")]
            fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                string.extend(self.as_bytes());
            }

            #[inline]
            #[cfg(feature = "feat-string-ext-bytes")]
            fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                string.extend(self.as_bytes());
                string.extend(separator.as_bytes());
            }
        }

        impl StringExtT for $ty {}
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
    fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
        self.encode_to_buf(string);
        string.extend(separator.as_bytes());
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
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
        self.encode_to_bytes_buf(string);
        string.extend(separator.as_bytes());
    }
}

impl StringExtT for char {}

impl_for_shared_ref!(COPIED: char);

#[macro_export]
/// See [`StrWrapper`] and [`StringWrapper`].
///
/// # Example
///
/// ```
/// # use std::sync::Arc;
/// # use macro_toolset::{str_wrapper, string_v2::StringExtT};
/// let general_str = str_wrapper!(str = "hello");
/// # assume_string_ext_t(general_str);
/// let smart_pointer_str = str_wrapper!(str = Arc::<str>::from("hello"));
/// # assume_string_ext_t(smart_pointer_str);
/// let general_string = str_wrapper!(str = "hello".to_string());
/// # assume_string_ext_t(general_string);
/// let smart_pointer_string = str_wrapper!(string = Arc::<String>::from("hello".to_string()));
/// # assume_string_ext_t(smart_pointer_string);
/// # fn assume_string_ext_t<T: StringExtT>(_: T) {}
/// ```
macro_rules! str_wrapper {
    (str = $data:expr) => {
        $crate::string_v2::general::string::StrWrapper { inner: $data }
    };
    (string = $data:expr) => {
        $crate::string_v2::general::string::StringWrapper { inner: $data }
    };
}

wrapper! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    /// Since the compiler will complain that *upstream crates may add a new impl
    /// of trait *** for type `{I}`*, we have to wrap it with newtype.
    ///
    /// Just `StrWrapper::from`.
    ///
    /// For `Arc<String>`, etc, use [`StringWrapper`] since it just implements `Deref<Target = String>` but not `Deref<Target = str>`.
    pub StrWrapper<T>(pub T)
}

wrapper! {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    /// Since the compiler will complain that *upstream crates may add a new impl
    /// of trait *** for type `{I}`*, we have to wrap it with newtype.
    ///
    /// Just `StrWrapper::from`.
    ///
    /// For `Arc<str>`, etc, use [`StrWrapper`] since it just implements `Deref<Target = str>` but not `Deref<Target = String>`.
    pub StringWrapper<T>(pub T)
}

impl_for_wrapper!(STRING_T: impl<T> StringT for StrWrapper<T> where T: Deref<Target = str>);
impl_for_wrapper!(STRING_T: impl<T> StringT for StringWrapper<T> where T: Deref<Target = String>);
impl_for_wrapper!(STRING_EXT_T: impl<T> StringExtT for StrWrapper<T> where T: Deref<Target = str>);
impl_for_wrapper!(STRING_EXT_T: impl<T> StringExtT for StringWrapper<T> where T: Deref<Target = String>);
