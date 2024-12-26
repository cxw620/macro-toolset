//! Useful [`StringExt`] utilities for [`crate::str_concat`] macros

pub mod externs;
pub mod general;
pub mod number;

// Re-export the `NumStr` type for convenience.
pub use number::NumStr;

/// Trait helper for push any string-like type to the string.
pub trait PushAnyT {
    /// Push any string-like type to the string.
    fn push_any<V>(&mut self, value: V)
    where
        V: StringT;

    /// Push any string-like type to the string with a separator.
    ///
    /// Only affects the array-or-slice-like types, since for a single element,
    /// it's meaningless.
    fn push_any_with_separator<V>(&mut self, value: V, sep: &str)
    where
        V: StringT;
}

impl PushAnyT for String {
    #[inline]
    fn push_any<V>(&mut self, value: V)
    where
        V: StringT,
    {
        #[allow(unsafe_code, reason = "safe because of the `StringT` trait")]
        value.encode_to_buf(unsafe { self.as_mut_vec() });
    }

    #[inline]
    fn push_any_with_separator<V>(&mut self, value: V, sep: &str)
    where
        V: StringT,
    {
        #[allow(unsafe_code, reason = "safe because of the `StringT` trait")]
        value.encode_to_buf_with_separator(unsafe { self.as_mut_vec() }, sep);
    }
}

impl PushAnyT for Vec<u8> {
    #[inline]
    fn push_any<V>(&mut self, value: V)
    where
        V: StringT,
    {
        value.encode_to_buf(self);
    }

    #[inline]
    fn push_any_with_separator<V>(&mut self, value: V, sep: &str)
    where
        V: StringT,
    {
        value.encode_to_buf_with_separator(self, sep);
    }
}

#[cfg(feature = "feat-string-ext-bytes")]
impl PushAnyT for bytes::BytesMut {
    #[inline]
    fn push_any<V>(&mut self, value: V)
    where
        V: StringT,
    {
        value.encode_to_bytes_buf(self);
    }

    #[inline]
    fn push_any_with_separator<V>(&mut self, value: V, sep: &str)
    where
        V: StringT,
    {
        value.encode_to_bytes_buf_with_separator(self, sep);
    }
}

/// Trait for string-like types.
pub trait StringT {
    /// Push the value to the string (the underlying `Vec<u8>`).
    fn encode_to_buf(self, string: &mut Vec<u8>);

    /// Push the value to the string (the underlying `Vec<u8>`) with a
    /// separator.
    ///
    /// Only affects the array-or-slice-like types.
    fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str);

    #[cfg(feature = "feat-string-ext-bytes")]
    /// Push the value to the string (the underlying `bytes::BytesMut`).
    fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut);

    #[cfg(feature = "feat-string-ext-bytes")]
    /// Push the value to the string (the underlying `bytes::BytesMut`) with a
    /// separator.
    ///
    /// Only affects the array-or-slice-like types.
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str);
}

#[allow(clippy::len_without_is_empty)]
/// Trait for string-like types, but extended with some methods making it not
/// dyn capable.
///
/// This is an auto trait implemented for all `StringT` types that are `Sized`.
pub trait StringExtT: StringT + Sized {
    #[inline(always)]
    /// With prefix.
    fn with_prefix<P: StringT>(self, prefix: P) -> general::tuple::SeplessTuple<(P, Self)> {
        general::tuple::SeplessTuple {
            inner: (prefix, self),
        }
    }

    #[inline(always)]
    /// With suffix.
    fn with_suffix<S: StringT>(self, suffix: S) -> general::tuple::SeplessTuple<(Self, S)> {
        general::tuple::SeplessTuple {
            inner: (self, suffix),
        }
    }

    #[inline]
    /// Encode the value to the string.
    fn to_string_ext(self) -> String {
        let mut string = String::with_capacity(64);
        #[allow(unsafe_code, reason = "safe because of the `StringT` trait")]
        self.encode_to_buf(unsafe { string.as_mut_vec() });
        string
    }

    #[inline]
    /// Encode the value(s) to the string with separator.
    ///
    /// For single-element values, this is the same as `to_string_ext`.
    fn to_string_with_separator(self, separator: &str) -> String {
        let mut string_buf = String::with_capacity(64);

        #[allow(unsafe_code, reason = "safe because of the `StringT` trait")]
        self.encode_to_buf_with_separator(unsafe { string_buf.as_mut_vec() }, separator);

        string_buf
    }
}

impl<T: StringT + Sized> StringExtT for T {}

// =============================================================================

#[doc(hidden)]
#[macro_export]
macro_rules! impl_for_ref_copy {
    ($($ty:ty)*) => {
        $(
            impl StringT for &$ty {
                #[inline(always)]
                fn encode_to_buf(self, string: &mut Vec<u8>) {
                    (*self).encode_to_buf(string);
                }

                #[inline(always)]
                fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                    (*self).encode_to_buf_with_separator(string, separator);
                }

                #[inline(always)]
                #[cfg(feature = "feat-string-ext-bytes")]
                /// Push the value to the string (the underlying `bytes::BytesMut`).
                fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                    (*self).encode_to_bytes_buf(string);
                }

                #[inline(always)]
                #[cfg(feature = "feat-string-ext-bytes")]
                /// Push the value to the string (the underlying `bytes::BytesMut`) with a
                /// separator.
                ///
                /// Only affects the array-or-slice-like types.
                fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                    (*self).encode_to_bytes_buf_with_separator(string, separator);
                }
            }

            impl StringT for &mut $ty {
                #[inline(always)]
                fn encode_to_buf(self, string: &mut Vec<u8>) {
                    (*self).encode_to_buf(string);
                }

                #[inline(always)]
                fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                    (*self).encode_to_buf_with_separator(string, separator);
                }

                #[inline(always)]
                #[cfg(feature = "feat-string-ext-bytes")]
                /// Push the value to the string (the underlying `bytes::BytesMut`).
                fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                    (*self).encode_to_bytes_buf(string);
                }

                #[inline(always)]
                #[cfg(feature = "feat-string-ext-bytes")]
                /// Push the value to the string (the underlying `bytes::BytesMut`) with a
                /// separator.
                ///
                /// Only affects the array-or-slice-like types.
                fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                    (*self).encode_to_bytes_buf_with_separator(string, separator);
                }
            }

            impl StringT for &&$ty {
                #[inline(always)]
                fn encode_to_buf(self, string: &mut Vec<u8>) {
                    (**self).encode_to_buf(string);
                }

                #[inline(always)]
                fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                    (**self).encode_to_buf_with_separator(string, separator);
                }

                #[inline(always)]
                #[cfg(feature = "feat-string-ext-bytes")]
                /// Push the value to the string (the underlying `bytes::BytesMut`).
                fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                    (**self).encode_to_bytes_buf(string);
                }

                #[inline(always)]
                #[cfg(feature = "feat-string-ext-bytes")]
                /// Push the value to the string (the underlying `bytes::BytesMut`) with a
                /// separator.
                ///
                /// Only affects the array-or-slice-like types.
                fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                    (**self).encode_to_bytes_buf_with_separator(string, separator);
                }
            }

            impl StringT for &mut &$ty {
                #[inline(always)]
                fn encode_to_buf(self, string: &mut Vec<u8>) {
                    (**self).encode_to_buf(string);
                }

                #[inline(always)]
                fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                    (**self).encode_to_buf_with_separator(string, separator);
                }

                #[inline(always)]
                #[cfg(feature = "feat-string-ext-bytes")]
                /// Push the value to the string (the underlying `bytes::BytesMut`).
                fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                    (**self).encode_to_bytes_buf(string);
                }

                #[inline(always)]
                #[cfg(feature = "feat-string-ext-bytes")]
                /// Push the value to the string (the underlying `bytes::BytesMut`) with a
                /// separator.
                ///
                /// Only affects the array-or-slice-like types.
                fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                    (**self).encode_to_bytes_buf_with_separator(string, separator);
                }
            }

            impl StringT for &&mut $ty {
                #[inline(always)]
                fn encode_to_buf(self, string: &mut Vec<u8>) {
                    (**self).encode_to_buf(string);
                }

                #[inline(always)]
                fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                    (**self).encode_to_buf_with_separator(string, separator);
                }

                #[inline(always)]
                #[cfg(feature = "feat-string-ext-bytes")]
                /// Push the value to the string (the underlying `bytes::BytesMut`).
                fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                    (**self).encode_to_bytes_buf(string);
                }

                #[inline(always)]
                #[cfg(feature = "feat-string-ext-bytes")]
                /// Push the value to the string (the underlying `bytes::BytesMut`) with a
                /// separator.
                ///
                /// Only affects the array-or-slice-like types.
                fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                    (**self).encode_to_bytes_buf_with_separator(string, separator);
                }
            }

            impl StringT for &&&$ty {
                #[inline(always)]
                fn encode_to_buf(self, string: &mut Vec<u8>) {
                    (***self).encode_to_buf(string);
                }

                #[inline(always)]
                fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                    (***self).encode_to_buf_with_separator(string, separator);
                }

                #[inline(always)]
                #[cfg(feature = "feat-string-ext-bytes")]
                /// Push the value to the string (the underlying `bytes::BytesMut`).
                fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                    (***self).encode_to_bytes_buf(string);
                }

                #[inline(always)]
                #[cfg(feature = "feat-string-ext-bytes")]
                /// Push the value to the string (the underlying `bytes::BytesMut`) with a
                /// separator.
                ///
                /// Only affects the array-or-slice-like types.
                fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                    (***self).encode_to_bytes_buf_with_separator(string, separator);
                }
            }
        )*
    };
}

#[doc(hidden)]
#[macro_export]
/// Remove the trailing separator from the string.
///
/// Notice: will not check if the separator exists or not!
macro_rules! remove_separator_tailing {
    ($string:expr, $separator:expr) => {
        let current_len = $string.len();
        if let Some(target_len) = current_len.checked_sub($separator.len()) {
            $string.truncate(target_len);
        }
    };
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;

    // Check dyn capable.
    type BoxedString = Box<dyn StringT>;

    #[test]
    fn test_separator() {
        let mut string = String::new();

        string.push_any_with_separator(["b", "c"], ",");
        assert_eq!(string, "b,c");
        string.clear();

        string.push_any(("a", "b", "c"));
        assert_eq!(string, "abc");

        string.push_any_with_separator(("a", "b", "c"), ",");
        assert_eq!(string, "abca,b,c");

        string.push_any_with_separator(("a", "b", "c", ("b", "c")), ",");
        assert_eq!(string, "abca,b,ca,b,c,b,c");

        string.push_any_with_separator(
            (
                &&&"a",
                vec!["b"],
                ("c"),
                ["b", "c"],
                "d".with_prefix("prefix-"),
                "e".with_suffix("-suffix"),
                "f".with_prefix("2prefix-").with_suffix("-suffix2"),
                1u8
            ),
            ",",
        );
        assert_eq!(
            string,
            "abca,b,ca,b,c,b,ca,b,c,b,c,prefix-d,e-suffix,2prefix-f-suffix2,1"
        );
    }
}
