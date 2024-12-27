//! Useful [`StringExt`] utilities for [`crate::str_concat`] macros

pub mod externs;
pub mod general;
pub mod number;

// Re-export the `NumStr` type for convenience.
pub use number::NumStr;

#[macro_export]
/// Fast concat [`String`] / &[`str`] / number.
///
/// For details of params accepted, please refers to [`StringT`].
///
/// # Examples
///
/// ```rust
/// # use macro_toolset::{str_concat_v2, string_v2::NumStr};
/// # fn main() {
/// // General usage
/// assert_eq!(
///     str_concat_v2!(
///         NumStr::hex_default(0xa_usize), // HexStr
///         "b", // &str
///         "c".to_string(), // String
///         1u8, // single number
///         '😀', // char
///         '�' // char
///     ), "abc1😀�"
/// );
/// // with initial string
/// let mut str_initial = "abc".to_string();
/// str_concat_v2!(str = str_initial; "1", "😀", "�");
/// assert_eq!(
///    str_initial, "abc1😀�"
/// );
///
/// // with capacity
/// assert_eq!(
///    str_concat_v2!(cap = 10; "abc", "1", "😀", "�"), "abc1😀�"
/// );
///
/// // with separator
/// assert_eq!(
///   str_concat_v2!(sep = ","; "abc", "1", "😀", "�"), "abc,1,😀,�"
/// );
/// # }
/// ```
macro_rules! str_concat_v2 {
    ($($x:expr),*) => {
        {
            use $crate::string_v2::StringExtT;

            ($($x,)*).to_string_ext()
        }
    };
    (str = $string_initial:expr; $($x:expr),*) => {
        {
            use $crate::string_v2::PushAnyT;

            $(
                $string_initial.push_any($x);
            )*
        }
    };
    (cap = $cap:expr; $($x:expr),*) => {
        {
            use $crate::string_v2::PushAnyT;

            let mut string_final = String::with_capacity($cap);

            $(
                string_final.push_any($x);
            )*

            string_final
        }
    };
    (sep = $sep:expr; $($x:expr),*) => {
        {
            use $crate::string_v2::StringExtT;

            ($($x,)*).to_string_ext_with_separator($sep)
        }
    };
}

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

/// Trait helper for push any string-like type to the string.
pub trait PushAnyT {
    /// Push any string-like type to the string.
    fn push_any<V>(&mut self, value: V)
    where
        V: StringT;

    /// Push any string-like type to the string with a separator.
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
        let inner = unsafe { self.as_mut_vec() };

        value.encode_to_buf_with_separator(inner, sep);

        // If is `None`?
        remove_separator_tailing!(inner, sep);
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
        remove_separator_tailing!(self, sep);
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
        remove_separator_tailing!(self, sep);
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
/// dyn-compatible.
///
/// This is an auto trait implemented for all `StringT` types that are `Sized`.
pub trait StringExtT: StringT + Sized {
    #[inline]
    /// With prefix.
    fn with_prefix<P: StringExtT + Copy>(self, prefix: P) -> impl StringExtT {
        general::tuple::SeplessTuple {
            inner: (prefix, self),
        }
    }

    #[inline]
    /// With suffix.
    fn with_suffix<S: StringExtT + Copy>(self, suffix: S) -> impl StringExtT {
        general::tuple::SeplessTuple {
            inner: (self, suffix),
        }
    }

    #[inline]
    /// Encode the value to the string.
    fn to_string_ext(self) -> String {
        let mut string_buf = String::with_capacity(64);

        string_buf.push_any(self);

        string_buf
    }

    #[inline]
    /// Encode the value(s) to the string with separator.
    ///
    /// For single-element values, this is the same as `to_string_ext`.
    fn to_string_ext_with_separator(self, separator: &str) -> String {
        let mut string_buf = String::with_capacity(64);

        string_buf.push_any_with_separator(self, separator);

        string_buf
    }
}

// =============================================================================

#[doc(hidden)]
#[macro_export]
macro_rules! impl_for_shared_ref {
    (COPIED: $($ty:ty)*) => {
        $(
            impl StringT for &$ty {
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

            impl StringExtT for &$ty {}

            impl StringT for &mut $ty {
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

            impl StringExtT for &mut $ty {}

            impl StringT for &&$ty {
                #[inline]
                fn encode_to_buf(self, string: &mut Vec<u8>) {
                    (**self).encode_to_buf(string);
                }

                #[inline]
                fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                    (**self).encode_to_buf_with_separator(string, separator);
                }

                #[inline]
                #[cfg(feature = "feat-string-ext-bytes")]
                fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                    (**self).encode_to_bytes_buf(string);
                }

                #[inline]
                #[cfg(feature = "feat-string-ext-bytes")]
                fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                    (**self).encode_to_bytes_buf_with_separator(string, separator);
                }
            }

            impl StringExtT for &&$ty {}

            impl StringT for &mut &$ty {
                #[inline]
                fn encode_to_buf(self, string: &mut Vec<u8>) {
                    (**self).encode_to_buf(string);
                }

                #[inline]
                fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                    (**self).encode_to_buf_with_separator(string, separator);
                }

                #[inline]
                #[cfg(feature = "feat-string-ext-bytes")]
                fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                    (**self).encode_to_bytes_buf(string);
                }

                #[inline]
                #[cfg(feature = "feat-string-ext-bytes")]
                fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                    (**self).encode_to_bytes_buf_with_separator(string, separator);
                }
            }

            impl StringExtT for &mut &$ty {}

            impl StringT for &&mut $ty {
                #[inline]
                fn encode_to_buf(self, string: &mut Vec<u8>) {
                    (**self).encode_to_buf(string);
                }

                #[inline]
                fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                    (**self).encode_to_buf_with_separator(string, separator);
                }

                #[inline]
                #[cfg(feature = "feat-string-ext-bytes")]
                fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                    (**self).encode_to_bytes_buf(string);
                }

                #[inline]
                #[cfg(feature = "feat-string-ext-bytes")]
                fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                    (**self).encode_to_bytes_buf_with_separator(string, separator);
                }
            }

            impl StringExtT for &&mut $ty {}

            impl StringT for &&&$ty {
                #[inline]
                fn encode_to_buf(self, string: &mut Vec<u8>) {
                    (***self).encode_to_buf(string);
                }

                #[inline]
                fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                    (***self).encode_to_buf_with_separator(string, separator);
                }

                #[inline]
                #[cfg(feature = "feat-string-ext-bytes")]
                fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                    (***self).encode_to_bytes_buf(string);
                }

                #[inline]
                #[cfg(feature = "feat-string-ext-bytes")]
                fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                    (***self).encode_to_bytes_buf_with_separator(string, separator);
                }
            }

            impl StringExtT for &&&$ty {}
        )*
    };
    (REF: $($ge:ident => $ty:ty)*) => {
        $(
            impl<$ge> StringT for $ty
            where
                for <'a> &'a $ge: StringT,
            {
                #[inline]
                fn encode_to_buf(self, string: &mut Vec<u8>) {
                    (&*self).encode_to_buf(string);
                }

                #[inline]
                fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                    (&*self).encode_to_buf_with_separator(string, separator);
                }

                #[inline]
                #[cfg(feature = "feat-string-ext-bytes")]
                fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                    (&*self).encode_to_bytes_buf(string);
                }

                #[inline]
                #[cfg(feature = "feat-string-ext-bytes")]
                fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                    (&*self).encode_to_bytes_buf_with_separator(string, separator);
                }
            }

            impl<$ge> StringExtT for $ty
            where
                for <'a> &'a $ge: StringExtT,
            {}
        )*
    };
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;

    // Check dyn capable.
    type BoxedString = Box<dyn StringT>;

    #[test]
    fn test_prefix_or_suffix() {
        let exp1 = "world".with_prefix("hello");
        assert_eq!(exp1.to_string_ext(), "helloworld");

        let exp2 = str_concat_v2!(sep = " "; ("hello", "world"));
        assert_eq!(exp2, "hello world");

        // dbg!(None::<()>
        //     .with_suffix("-suffix")
        //     .with_prefix("prefix-")
        //     .to_string_ext());

        let exp3 = str_concat_v2!(
            sep = " ";
            None::<()>.with_prefix("prefix-")
        );

        assert_eq!(exp3, "");

        let exp4 = str_concat_v2!(
            sep = " ";
            None::<()>.with_prefix("prefix-"),
            None::<()>,
            None::<()>
        );

        assert_eq!(exp4, "");

        let exp5 = str_concat_v2!(
            sep = " ";
            (None::<()>.with_prefix("prefix-"), None::<()>.with_prefix("prefix-")),
            ("hello", "world"),
            "hello".with_suffix(Some("-suffix")),
            None::<()>.with_prefix("prefix-"),
            "3hello".with_suffix(None::<()>).with_prefix(None::<()>),
            None::<()>.with_prefix("prefix-").with_suffix("-suffix")
        );
        assert_eq!(exp5, "hello world hello-suffix 3hello");
    }

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
                1u8,
            ),
            ",",
        );
        assert_eq!(
            string,
            "abca,b,ca,b,c,b,ca,b,c,b,c,prefix-d,e-suffix,2prefix-f-suffix2,1"
        );
    }
}
