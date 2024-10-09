//! useful [`StringExt`] utilities for [`crate::str_concat`] macros

use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
    rc::Rc,
    sync::Arc,
};

#[cfg(feature = "macros-string-sha256")]
use sha2::{Digest as _, Sha256};

#[cfg(feature = "macros-base64")]
use crate::b64_encode;

#[macro_export]
/// Fast concat [`String`] / &[`str`] / number.
///
/// For details of params accepted, please refers to [StringExtT].
///
/// # Examples
///
/// ```rust
/// # use macro_toolset::{str_concat, string::HexStr};
/// # fn main() {
/// /// General usage
/// assert_eq!(
///     str_concat!(
///         HexStr::new_default(0xa_usize), // HexStr
///         "b", // &str
///         "c".to_string(), // String
///         1u8, // unsigned num
///         '😀', // char
///         '�' // char
///     ), "abc1😀�"
/// );
/// /// with initial string
/// assert_eq!(
///    str_concat!(str = "abc"; "1", "😀", "�"), "abc1😀�"
/// );
///
/// /// with capacity
/// assert_eq!(
///    str_concat!(cap = 10; "abc", "1", "😀", "�"), "abc1😀�"
/// );
///
/// /// with separator
/// assert_eq!(
///   str_concat!(sep = ","; "abc", "1", "😀", "�"), "abc,1,😀,�"
/// );
/// # }
/// ```
macro_rules! str_concat {
    ($($x:expr),*) => {
        {
            let mut string_final = $crate::string::StringExt::with_capacity(512);
            $(
                string_final.push($x);
            )*
            string_final.into_string()
        }
    };
    (str = $str:expr; $($x:expr),*) => {
        {
            let mut string_final = macro_toolset::string::StringExt::from($str);
            $(
                string_final.push($x);
            )*
            string_final.into_string()
        }
    };
    (cap = $cap:expr; $($x:expr),*) => {
        {
            let mut string_final = macro_toolset::string::StringExt::with_capacity($cap);
            $(
                string_final.push($x);
            )*
            string_final.into_string()
        }
    };
    (sep = $sep:expr; $($x:expr),*) => {
        {
            let mut string_final = $crate::string::StringExt::with_capacity(512);
            $(
                string_final.push_with_separator($x, $sep);
            )*
            string_final.into_string_remove_tail($sep)
        }
    };
}

#[macro_export]
/// Fast way to convert number to hex string
///
/// See [`HexStr`] for more details.
///
/// If you want to convert [u8] to hex string, use [`const_hex`] instead.
///
/// # Example
/// ```rust
/// # use macro_toolset::hex_string;
/// # fn main() {
/// let hex_str = hex_string!(0x123abc_usize);
/// assert_eq!(hex_str, "123abc");
///
/// /// lowercase, resize to 3, no minimum
/// let hex_str = hex_string!(0x123abc_usize, false, 3, 0);
/// assert_eq!(hex_str, "abc");
///
/// /// uppercase, resize to 3, no minimum
/// let hex_str = hex_string!(0x123abc_usize, true, 3, 0);
/// assert_eq!(hex_str, "ABC");
///
/// /// lowercase, resize to 9, no minimum
/// let hex_str = hex_string!(0x123abc_usize, false, 9, 0);
/// assert_eq!(hex_str, "000123abc");
///
/// /// uppercase, resize to 9, no minimum
/// let hex_str = hex_string!(0x123abc_usize, true, 9, 0);
/// assert_eq!(hex_str, "000123ABC");
///
/// /// lowercase, no resize, minimum to 3
/// let hex_str = hex_string!(0x123abc_usize, false, 0, 3);
/// assert_eq!(hex_str, "123abc");
///
/// /// uppercase, no resize, minimum to 3
/// let hex_str = hex_string!(0x123abc_usize, true, 0, 3);
/// assert_eq!(hex_str, "123ABC");
///
/// /// lowercase, no resize, minimum to 9
/// let hex_str = hex_string!(0x123abc_usize, false, 0, 9);
/// assert_eq!(hex_str, "000123abc");
///
/// /// uppercase, no resize, minimum to 9
/// let hex_str = hex_string!(0x123abc_usize, true, 0, 9);
/// assert_eq!(hex_str, "000123ABC");
///
/// /// if you set resize along with minimum, minimum will be ignored
/// let hex_str = hex_string!(0x123abc_usize, false, 3, 9);
/// assert_eq!(hex_str, "abc");
/// # }
/// ```
macro_rules! hex_string {
    ($num:expr) => {{
        use macro_toolset::string::StringExtT;
        macro_toolset::string::HexStr::new_default($num).to_string_ext()
    }};
    ($num:expr, $u:expr, $r:expr, $m:expr) => {{
        use macro_toolset::string::StringExtT;
        macro_toolset::string::HexStr::<$u, $r, $m>::new($num).to_string_ext()
    }};
}

#[macro_export]
/// Fast way to convert number to decimal string
///
/// See [`DecStr`] for more details.
///
/// # Example
/// ```rust
/// # use macro_toolset::dec_string;
/// # fn main() {
/// let dec_str = dec_string!(123456_usize);
/// assert_eq!(dec_str, "123456");
///
/// /// resize to 3, no minimum
/// let dec_str = dec_string!(123456_usize, 3, 0);
/// assert_eq!(dec_str, "456");
///
/// /// resize to 9, no minimum
/// let dec_str = dec_string!(123456_usize, 9, 0);
/// assert_eq!(dec_str, "000123456");
///
/// /// no resize, minimum to 3
/// let dec_str = dec_string!(123456_usize, 0, 3);
/// assert_eq!(dec_str, "123456");
///
/// /// no resize, minimum to 9
/// let dec_str = dec_string!(123456_usize, 0, 9);
/// assert_eq!(dec_str, "000123456");
///
/// /// if you set resize along with minimum, minimum will be ignored
/// let dec_str = dec_string!(123456_usize, 3, 9);
/// assert_eq!(dec_str, "456");
/// # }
/// ```
macro_rules! dec_string {
    ($num:expr) => {{
        use macro_toolset::string::StringExtT;
        macro_toolset::string::DecStr::new_default($num).to_string_ext()
    }};
    ($num:expr, $r:expr, $m:expr) => {{
        use macro_toolset::string::StringExtT;
        macro_toolset::string::DecStr::<$r, $m>::new($num).to_string_ext()
    }};
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
/// A string with extended utilities
///
/// In most circumstances, it can be considered as a [`String`] with more utilities.
pub struct StringExt {
    inner: Vec<u8>,
}

impl StringExt {
    #[inline]
    #[must_use]
    /// Create a new [`StringExt`] with capacity
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: Vec::with_capacity(cap),
        }
    }

    #[inline]
    #[must_use]
    /// Converts a vector of bytes to a `String` without checking that the
    /// string contains valid UTF-8.
    pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> Self {
        Self { inner: bytes }
    }

    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    /// Consume self and get the inner Vec<u8>
    ///
    /// # Safety
    ///
    /// Just mark the inner Vec<u8> as mutable, but it's not safe to modify the Vec<u8> directly
    /// and you must ensure the Vec<u8> is always valid utf8
    pub unsafe fn into_bytes(self) -> Vec<u8> {
        self.inner
    }

    #[inline]
    #[must_use]
    /// Extracts a string slice containing the entire `StringExt`.
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.inner.as_slice()) }
    }

    #[inline]
    #[must_use]
    /// Converts a `StringExt` into a mutable string slice.
    pub fn as_mut_str(&mut self) -> &mut str {
        unsafe { std::str::from_utf8_unchecked_mut(self.inner.as_mut_slice()) }
    }

    #[inline]
    /// push a value to the string, which implements [`StringExtT`]
    pub fn push(&mut self, value: impl StringExtT) {
        value.push_to_string(&mut self.inner);
    }

    #[inline]
    /// push a value to the string ends with given separator, which both implement [`StringExtT`]
    ///
    /// Note: the separator will be pushed even if the value is empty, and do `pop()` if you don't
    /// want it in the end of the string after [`StringExt::into_string`]
    pub fn push_with_separator(&mut self, value: impl StringExtT, separator: impl SeparatorT) {
        value.push_to_string_with_separator(&mut self.inner, separator);
    }

    /// Returns this `StringExt`'s capacity, in bytes.
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Reserves capacity for at least `additional` bytes more than the
    /// current length. The allocator may reserve more space to speculatively
    /// avoid frequent allocations. After calling `reserve`,
    /// capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if capacity is already sufficient.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Reserves the minimum capacity for at least `additional` bytes more than
    /// the current length. Unlike [`reserve`], this will not
    /// deliberately over-allocate to speculatively avoid frequent allocations.
    /// After calling `reserve_exact`, capacity will be greater than or equal to
    /// `self.len() + additional`. Does nothing if the capacity is already
    /// sufficient.
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    #[inline]
    /// Returns a byte slice of this `StringExt`'s contents.
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }

    #[inline]
    #[must_use]
    /// Get the length of the string
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    #[must_use]
    /// Returns `true` if this `String` has a length of zero, and `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    /// Borrow the inner Vec<u8>
    ///
    /// # Safety
    ///
    /// Just mark the inner Vec<u8> as mutable, but it's not safe to modify the Vec<u8> directly
    /// and you must ensure the Vec<u8> is always valid utf8
    pub unsafe fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }

    #[inline]
    /// Consume self and get the final String
    pub fn into_string(self) -> String {
        // Safety: the inner Vec<u8> is always valid utf8
        unsafe { String::from_utf8_unchecked(self.inner) }
    }

    #[inline]
    /// Consume self and get the final String, removing the separator from the end of the string
    ///
    /// Notice: the separator char / string will be removed, even if it's from the original string!
    pub fn into_string_remove_tail(mut self, separator: impl SeparatorT) -> String {
        separator.remove_end(&mut self.inner);
        // Safety: the inner Vec<u8> is always valid utf8
        unsafe { String::from_utf8_unchecked(self.inner) }
    }
}

impl From<String> for StringExt {
    #[inline]
    fn from(inner: String) -> Self {
        Self {
            inner: inner.into_bytes(),
        }
    }
}

impl From<&str> for StringExt {
    #[inline]
    fn from(inner: &str) -> Self {
        let mut new_inner = Vec::with_capacity(inner.len() + 128);
        new_inner.extend(inner.as_bytes());
        Self { inner: new_inner }
    }
}

impl From<&String> for StringExt {
    #[inline]
    fn from(inner: &String) -> Self {
        let mut new_inner = Vec::with_capacity(inner.len() + 128);
        new_inner.extend(inner.as_bytes());
        Self { inner: new_inner }
    }
}

impl AsRef<str> for StringExt {
    #[inline]
    fn as_ref(&self) -> &str {
        // Safety: the inner Vec<u8> is always valid utf8
        unsafe { std::str::from_utf8_unchecked(&self.inner) }
    }
}

impl AsRef<[u8]> for StringExt {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

/// Trait for pushing any value that can be converted into str to [`StringExt`]
///
/// If need you can implement this trait for your own type
pub trait StringExtT: Sized {
    /// Push the value to the string
    fn push_to_string(self, string: &mut Vec<u8>);

    #[inline]
    /// Push the value to the string with separator
    ///
    /// - separator should implement [`SeparatorT`] and be `Copy`
    fn push_to_string_with_separator(self, string: &mut Vec<u8>, separator: impl SeparatorT) {
        self.push_to_string(string);
        separator.push_to_string(string);
    }

    /// encode the value to the string.
    fn to_string_ext(self) -> String {
        let mut string = Vec::with_capacity(128);
        self.push_to_string(&mut string);
        // Safety: the inner Vec<u8> is always valid utf8
        unsafe { String::from_utf8_unchecked(string) }
    }

    #[inline]
    /// Push the value to the string with separator
    ///
    /// - separator should implement [`SeparatorT`] and be `Copy`
    fn to_string_ext_with_sep(self, separator: impl SeparatorT) -> String {
        let mut string = Vec::with_capacity(128);

        self.push_to_string_with_separator(&mut string, separator);
        separator.remove_end(&mut string);

        // Safety: the inner Vec<u8> is always valid utf8
        unsafe { String::from_utf8_unchecked(string) }
    }
}

impl StringExtT for () {
    #[inline]
    fn push_to_string(self, _: &mut Vec<u8>) {}

    fn push_to_string_with_separator(self, _string: &mut Vec<u8>, _separator: impl SeparatorT) {}

    fn to_string_ext(self) -> String {
        String::new()
    }

    fn to_string_ext_with_sep(self, _separator: impl SeparatorT) -> String {
        String::new()
    }
}

impl StringExtT for char {
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        // ! '�' != "�" in utf8 world. see [`String::push`].
        match self.len_utf8() {
            1 => string.push(self as u8),
            _ => string.extend(self.encode_utf8(&mut [0; 4]).as_bytes()),
        }
    }
}

impl StringExtT for String {
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        string.extend(self.as_bytes());
    }

    #[inline]
    fn to_string_ext(self) -> String {
        self
    }
}

macro_rules! impl_for_str {
    ($($ty:ty),*) => {
        $(
            impl StringExtT for $ty {
                #[inline]
                fn push_to_string(self, string: &mut Vec<u8>) {
                    string.extend(self.as_bytes());
                }
            }
        )*
    };
}

macro_rules! impl_for_ref_copied {
    ($($ty:ty),*) => {
        $(
            impl StringExtT for &$ty {
                #[inline]
                fn push_to_string(self, string: &mut Vec<u8>) {
                    (*self).push_to_string(string);
                }

                #[inline]
                fn push_to_string_with_separator(self, string: &mut Vec<u8>, separator: impl SeparatorT) {
                    (*self).push_to_string_with_separator(string, separator);
                }

                #[inline]
                fn to_string_ext(self) -> String {
                    (*self).to_string_ext()
                }

                #[inline]
                fn to_string_ext_with_sep(self, separator: impl SeparatorT) -> String {
                    (*self).to_string_ext_with_sep(separator)
                }
            }
        )*
    };
}

macro_rules! impl_for_num {
    (SIGNED; $($ty:ty),*) => {
        $(
            impl StringExtT for $ty {
                #[inline]
                fn push_to_string(self, string: &mut Vec<u8>) {
                    if self.is_negative() {
                        string.push(b'-');
                        DecStr::new_default(self.wrapping_neg() as usize)
                    } else {
                        DecStr::new_default(self as usize)
                    }.push_to_string(string);
                }


                #[inline]
                fn to_string_ext(self) -> String {
                    let mut string = String::with_capacity(32);
                    // Safe: number to string is always valid utf8
                    self.push_to_string(unsafe { string.as_mut_vec() });
                    string
                }
            }
        )*
    };
    (UNSIGNED; $($ty:ty),*) => {
        $(
            impl StringExtT for $ty {
                #[inline]
                fn push_to_string(self, string: &mut Vec<u8>) {
                    DecStr::new_default(self as usize).push_to_string(string);
                }

                #[inline]
                fn to_string_ext(self) -> String {
                    DecStr::new_default(self as usize).to_string_ext()
                }
            }
        )*
    };
}

macro_rules! impl_for_tuple {
    ( $( $name:ident )+ ) => {
        #[allow(non_snake_case)]
        impl<$($name: StringExtT),+> StringExtT for ($($name,)+)
        {
            #[inline]
            fn push_to_string(self, string: &mut Vec<u8>) {
                let ($($name,)+) = self;
                $($name.push_to_string(string);)+
            }

            #[inline]
            fn push_to_string_with_separator(self, string: &mut Vec<u8>, separator: impl SeparatorT) {
                let ($($name,)+) = self;
                $(
                    $name.push_to_string_with_separator(string, separator);
                )+
            }
        }
    };
}

impl_for_str!(
    Rc<str>,
    Rc<String>,
    Arc<str>,
    Arc<String>,
    &str,
    &String,
    Cow<'_, str>,
    &Cow<'_, str>
);
impl_for_ref_copied!(char, &char, &str, &&str);

impl_for_num!(UNSIGNED; u8, u16,  usize);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_for_num!(UNSIGNED; u32);
#[cfg(target_pointer_width = "64")]
impl_for_num!(UNSIGNED; u64);

impl_for_ref_copied!(u8, u16, usize);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_for_ref_copied!(u32);
#[cfg(target_pointer_width = "64")]
impl_for_ref_copied!(u64);

impl_for_num!(SIGNED; i8, i16, i32, isize);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_for_num!(SIGNED; i64);

impl_for_ref_copied!(i8, i16, i32, isize);
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_for_ref_copied!(i64);

impl_for_tuple!(T1);
impl_for_tuple!(T1 T2);
impl_for_tuple!(T1 T2 T3);
impl_for_tuple!(T1 T2 T3 T4);
impl_for_tuple!(T1 T2 T3 T4 T5);
impl_for_tuple!(T1 T2 T3 T4 T5 T6);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27);

impl<T> StringExtT for Vec<T>
where
    T: StringExtT,
{
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        for item in self {
            item.push_to_string(string);
        }
    }

    #[inline]
    fn push_to_string_with_separator(self, string: &mut Vec<u8>, separator: impl SeparatorT) {
        let is_empty = self.is_empty();

        for item in self {
            item.push_to_string_with_separator(string, separator);
        }

        if !is_empty {
            separator.remove_end(string);
            separator.push_to_string(string)
        }
    }
}

impl<T> StringExtT for &[T]
where
    T: StringExtT + Copy,
{
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        for item in self {
            item.push_to_string(string);
        }
    }

    #[inline]
    fn push_to_string_with_separator(self, string: &mut Vec<u8>, separator: impl SeparatorT) {
        let is_empty = self.is_empty();

        for item in self {
            item.push_to_string_with_separator(string, separator);
        }

        if !is_empty {
            separator.remove_end(string);
            separator.push_to_string(string)
        }
    }
}

impl<T, const N: usize> StringExtT for [T; N]
where
    T: StringExtT,
{
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        for item in self {
            item.push_to_string(string);
        }
    }

    #[inline]
    fn push_to_string_with_separator(self, string: &mut Vec<u8>, separator: impl SeparatorT) {
        for item in self {
            item.push_to_string_with_separator(string, separator);
        }
        if N != 0 {
            separator.remove_end(string);
            separator.push_to_string(string)
        }
    }
}

impl<T, const N: usize> StringExtT for &[T; N]
where
    T: StringExtT + Copy,
{
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        for item in self.iter() {
            item.push_to_string(string);
        }
    }

    #[inline]
    fn push_to_string_with_separator(self, string: &mut Vec<u8>, separator: impl SeparatorT) {
        for item in self {
            item.push_to_string_with_separator(string, separator);
        }
        if N != 0 {
            separator.remove_end(string);
            separator.push_to_string(string)
        }
    }
}

// ! cannot implement for Iterator directly, since compiler
// ! would complain about conflicting implementations

impl<T, I, F> StringExtT for std::iter::Map<I, F>
where
    T: StringExtT,
    I: Iterator,
    F: FnMut(I::Item) -> T,
{
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        self.into_iter()
            .for_each(|item| item.push_to_string(string));
    }

    #[inline]
    fn push_to_string_with_separator(self, string: &mut Vec<u8>, separator: impl SeparatorT) {
        let mut is_empty = true;
        self.into_iter().for_each(|item| {
            is_empty = false;
            item.push_to_string_with_separator(string, separator);
        });
        if !is_empty {
            separator.remove_end(string);
            separator.push_to_string(string)
        }
    }
}

impl<T> StringExtT for Option<T>
where
    T: StringExtT,
{
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        if let Some(item) = self {
            item.push_to_string(string);
        }
    }

    #[inline]
    fn push_to_string_with_separator(self, string: &mut Vec<u8>, separator: impl SeparatorT) {
        if let Some(item) = self {
            item.push_to_string_with_separator(string, separator);
        }
    }
}

impl<T: StringExtT> StringExtT for Box<T> {
    fn push_to_string_with_separator(self, string: &mut Vec<u8>, separator: impl SeparatorT) {
        (*self).push_to_string_with_separator(string, separator)
    }

    fn to_string_ext(self) -> String {
        (*self).to_string_ext()
    }

    fn push_to_string(self, string: &mut Vec<u8>) {
        (*self).push_to_string(string);
    }

    fn to_string_ext_with_sep(self, separator: impl SeparatorT) -> String {
        (*self).to_string_ext_with_sep(separator)
    }
}

// === extend utilities ===

#[allow(clippy::len_without_is_empty)]
/// Trait for removing the separator from the end of the string
pub trait SeparatorT: StringExtT + Copy {
    /// Remove the separator from the end of the string
    fn remove_end(&self, string: &mut Vec<u8>);
}

impl SeparatorT for char {
    #[inline]
    fn remove_end(&self, string: &mut Vec<u8>) {
        let len = self.len_utf8();

        match len {
            1 => {
                if string.last() == Some(&(*self as u8)) {
                    string.pop();
                }
            }
            _ => {
                let buf = &mut [0; 4];
                let buf = self.encode_utf8(buf);
                if string.get(string.len() - len..) == Some(buf.as_bytes()) {
                    string.truncate(string.len() - 4);
                }
            }
        }
    }
}

impl SeparatorT for &str {
    #[inline]
    fn remove_end(&self, string: &mut Vec<u8>) {
        string.truncate(string.len() - self.len());
    }
}

#[cfg(feature = "macros-base64")]
#[derive(Debug, Clone, Default)]
/// Encode binary data to base64 string
pub struct BinaryStr(bytes::BytesMut);

#[cfg(feature = "macros-base64")]
impl std::ops::Deref for BinaryStr {
    type Target = bytes::BytesMut;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "macros-base64")]
impl std::ops::DerefMut for BinaryStr {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "macros-base64")]
impl StringExtT for BinaryStr {
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        string.extend(b64_encode!(self.0).as_bytes());
    }
}

#[cfg(feature = "macros-base64")]
impl BinaryStr {
    /// Create a new [`BinaryStr`] with capacity
    pub fn with_capacity(cap: usize) -> Self {
        Self(bytes::BytesMut::with_capacity(cap))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(transparent)]
/// uint to string
///
/// # Generic
///
/// - `B`: the base of the number, only support 10 and 16
/// - `U`: whether to use uppercase for hex, default is lowercase
/// - `R`: the resize length of the string. The overflow part will be truncated, and the insufficient
///   part will be filled with '0'
/// - `M`: the minimum length of the string, if the length of the string is less than `M`, fill with '0'.
///
/// # Panic
///
/// - invalid base (== 0 or > 16)
pub struct NumStr<const B: usize, const U: bool, const R: usize, const M: usize>(usize);

impl Display for NumStr<10, false, 0, 0> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const B: usize, const U: bool, const R: usize, const M: usize> AsRef<usize>
    for NumStr<B, U, R, M>
{
    #[inline]
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl<const B: usize, const U: bool, const R: usize, const M: usize> NumStr<B, U, R, M> {
    const HEX_CHARS_LOWER: [u8; 16] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e',
        b'f',
    ];
    const HEX_CHARS_UPPER: [u8; 16] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E',
        b'F',
    ];

    #[inline]
    /// Create a new [`NumStr`] with the given number
    ///
    /// Concrete settings of `B`, `U`, `R`, `M` can be inferred from the context or manually specified.
    ///
    /// See [`NumStr`] for details
    pub fn new(inner: impl Into<usize>) -> Self {
        Self(inner.into())
    }

    #[inline]
    fn charset() -> &'static [u8] {
        debug_assert!(B <= 16 && B != 0, "unsupported base: {}", B);
        if U {
            &Self::HEX_CHARS_UPPER
        } else {
            &Self::HEX_CHARS_LOWER
        }
    }

    #[inline]
    /// Set whether to resize the string to `len` length.
    ///
    /// The overflow part will be truncated, and the insufficient part will be filled with '0'
    ///
    /// Default is not resize
    ///
    /// Note: see [`Vec::resize`] for details
    pub fn set_resize_len<const NR: usize>(self) -> NumStr<B, U, NR, M> {
        NumStr::<B, U, NR, M>(self.0)
    }

    #[inline]
    /// Set the minimum length of the string.
    ///
    /// The insufficient part will be filled with '0'.
    ///
    /// Default is not minimum
    ///
    /// Note: if set `Self::should_resize`, the minimum length will be ignored
    pub fn set_minimum_len<const NM: usize>(self) -> NumStr<B, U, R, NM> {
        NumStr::<B, U, R, NM>(self.0)
    }

    #[inline]
    /// Set uppercase / lowercase of the number.
    ///
    /// Default is lowercase
    ///
    /// Note: only works for base > 10
    pub fn set_uppercase<const NU: bool>(self) -> NumStr<B, NU, R, M> {
        NumStr::<B, NU, R, M>(self.0)
    }

    #[inline]
    /// Encode the number to the str
    pub fn encode(self) -> Vec<u8> {
        let mut num_div = Vec::with_capacity(32);

        if self.0 == 0 {
            num_div.push(b'0');
            return num_div;
        }

        let charset = Self::charset();
        let mut num = self.0;
        while num > 0 {
            num_div.push(charset[num % B]);
            num /= B;
        }

        if R != 0 {
            num_div.resize(R, b'0');
        } else if M != 0 && num_div.len() < M {
            num_div.resize(M, b'0');
        }

        num_div.reverse();
        num_div
    }

    #[inline]
    /// Encode the number to the str
    pub fn encode_bigint(num: u128) -> Vec<u8> {
        let mut num_div = Vec::with_capacity(32);

        if num == 0 {
            num_div.push(b'0');
            return num_div;
        }

        let charset = Self::charset();
        let mut num = num;
        while num > 0 {
            num_div.push(charset[(num % (B as u128)) as usize]);
            num /= B as u128;
        }

        if R != 0 {
            num_div.resize(R, b'0');
        } else if M != 0 && num_div.len() < M {
            num_div.resize(M, b'0');
        }

        num_div.reverse();
        num_div
    }
}

impl<const B: usize, const U: bool, const R: usize, const M: usize> StringExtT
    for NumStr<B, U, R, M>
{
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        string.extend(self.encode());
    }

    #[inline]
    fn to_string_ext(self) -> String {
        // Safety: the inner Vec<u8> is always valid utf8
        unsafe { String::from_utf8_unchecked(self.encode()) }
    }
}

/// Just a [`NumStr`] with base 10
pub type DecStr<const R: usize = 0, const M: usize = 0> = NumStr<10, false, R, M>;

impl From<usize> for DecStr {
    #[inline]
    fn from(inner: usize) -> Self {
        Self(inner)
    }
}

impl DecStr {
    #[inline]
    /// Create a new [`DecStr`] with default settings: no resize, no minimum
    pub fn new_default(inner: impl Into<usize>) -> Self {
        NumStr(inner.into())
    }

    /// Create a new [`DecStr`]<R, M> .
    ///
    /// - `R`: the resize length of the string. The overflow part will be truncated, and the insufficient
    ///   part will be filled with '0'
    /// - `M`: the minimum length of the string, if the length of the string is less than `M`, fill with '0'.
    pub fn dec<const R: usize, const M: usize>(inner: impl Into<usize>) -> DecStr<R, M> {
        NumStr(inner.into())
    }

    #[inline]
    /// Map a slice of numbers to a slice of [`DecStr`].
    ///
    /// The return type is actually an iterator and implements [`StringExtT`].
    ///
    /// - `U`: whether to use uppercase for hex
    /// - `R`: the resize length of the string. The overflow part will be truncated, and the insufficient
    ///  part will be filled with '0'
    /// - `M`: the minimum length of the string, if the length of the string is less than `M`, fill with '0'.
    ///
    /// You may use macro [`dec_str`] to create a slice of [`DecStr`]:
    ///
    /// ```rust
    /// # use macro_toolset::{dec_str, string::StringExtT};
    /// # let dec_str =
    /// dec_str!(SLICE: &[123456789_usize, 987654321_usize]; R => 0; M => 0)
    /// # .to_string_ext_with_sep(',');
    /// # assert_eq!(dec_str, "123456789,987654321")
    /// ```
    ///
    /// You do not need to specify `R`, `M` at the same time like the example does.
    ///
    /// This is offen used to create [`DecStr`]s from a [Vec] or a slice, then [to_string_ext](StringExtT::to_string_ext),
    /// or [to_string_ext_with_sep](StringExtT::to_string_ext_with_sep) to get the final string.
    pub fn map_dec_slice<const R: usize, const M: usize>(
        inner: &[impl Into<usize> + Copy],
    ) -> impl StringExtT + '_ {
        inner.iter().map(|n| DecStr::dec::<R, M>(*n))
    }
}

#[macro_export]
/// Create a [`DecStr`] for a single number
///
/// # Exmaples
///
/// ```rust
/// use macro_toolset::{dec_str, string::StringExtT};
///
/// // default
/// # assert_eq!(
/// dec_str!(123456789_usize) // 123456789
/// # .to_string_ext(), "123456789");
///
/// // resize
/// # assert_eq!(
/// dec_str!(123456789_usize; R => 10) // +[0] 123456789
/// # .to_string_ext(), "0123456789");
/// # assert_eq!(
/// dec_str!(123456789_usize; R => 5) // 56789
/// # .to_string_ext(), "56789");
///
/// // minimum
/// # assert_eq!(
/// dec_str!(123456789_usize; M => 8) // 123456789
/// # .to_string_ext(), "123456789");
/// # assert_eq!(
/// dec_str!(123456789_usize; M => 16) // +[0000000] 123456789
/// # .to_string_ext(), "0000000123456789");
///
/// // mix R or M, R is always first considered so do not do so
///
/// // Create a temp slice
/// # let _ =
/// dec_str!(SLICE: 123456789_usize, 987654321_usize)
/// # ;
///
/// ```
///
/// Notice: `R`, `M` in the macro are optional but should be in order.
macro_rules! dec_str {
    ($dig:expr $(;R => $r:expr)? $(;M => $m:expr)?) => {
        $crate::string::DecStr::new_default($dig)
            $(.set_resize_len::<$r>())?
            $(.set_minimum_len::<$m>())?
    };
    (SLICE: $dig_f:expr, $($dig:expr),+ $(;R => $r:expr)? $(;M => $m:expr)?) => {
        &[
            $crate::dec_str!($dig_f $(;R => $r)? $(;M => $m)?),
            $(
                $crate::string::NumStr::new($dig)
            )+
        ]
    };
    (SLICE: $vec:expr; R => $r:expr) => {{
        $crate::string::DecStr::map_dec_slice::<$r, 0>($vec)
    }};
    (SLICE: $vec:expr; M => $m:expr) => {{
        $crate::string::DecStr::map_dec_slice::<0, $m>($vec)
    }};
    (SLICE: $vec:expr; R => $r:expr; M => $m:expr) => {{
        $crate::string::DecStr::map_dec_slice::<$r, $m>($vec)
    }};
}

/// Just a [`NumStr`] with base 16
pub type HexStr<const U: bool = false, const R: usize = 0, const M: usize = 0> =
    NumStr<16, U, R, M>;

impl From<usize> for HexStr {
    #[inline]
    fn from(inner: usize) -> Self {
        Self(inner)
    }
}

impl HexStr {
    #[inline]
    /// Create a new [`HexStr`] with default settings: lowercase, no resize, no minimum
    ///
    /// You may use macro [`hex_str`] to create a [`HexStr`]:
    ///
    /// ```rust
    /// # use macro_toolset::{hex_str, string::StringExtT};
    /// # let hex_str =
    /// hex_str!(0x123456789abc_usize) // 123456789abc
    /// # ;
    ///
    /// # assert_eq!(hex_str.to_string_ext(), "123456789abc");
    /// ```
    ///
    /// For more details, see [`hex_str`].
    pub fn new_default(inner: impl Into<usize>) -> HexStr {
        NumStr(inner.into())
    }

    #[inline]
    /// Create a new [`HexStr`]
    ///
    /// - `U`: whether to use uppercase for hex
    /// - `R`: the resize length of the string. The overflow part will be truncated, and the insufficient
    ///  part will be filled with '0'
    /// - `M`: the minimum length of the string, if the length of the string is less than `M`, fill with '0'.
    ///
    /// You may use macro [`hex_str`] to create a [`HexStr`]. See [`hex_str`] for details.
    pub fn hex<const U: bool, const R: usize, const M: usize>(
        inner: impl Into<usize>,
    ) -> HexStr<U, R, M> {
        NumStr(inner.into())
    }

    #[inline]
    /// Map a slice of numbers to a slice of [`HexStr`].
    ///
    /// The return type is actually an iterator and implements [`StringExtT`].
    ///
    /// - `U`: whether to use uppercase for hex
    /// - `R`: the resize length of the string. The overflow part will be truncated, and the insufficient
    ///  part will be filled with '0'
    /// - `M`: the minimum length of the string, if the length of the string is less than `M`, fill with '0'.
    ///
    /// You may use macro [`hex_str`] to create a slice of [`HexStr`]:
    ///
    /// ```rust
    /// # use macro_toolset::{hex_str, string::StringExtT};
    /// # let hex_str =
    /// hex_str!(SLICE: &[0x123456789abc_usize, 0xcba987654321_usize]; U => true; R => 0; M => 0)
    /// # .to_string_ext_with_sep(',');
    /// # assert_eq!(hex_str, "123456789ABC,CBA987654321")
    /// ```
    ///
    /// You do not need to specify `U`, `R`, `M` at the same time like the example does.
    ///
    /// This is offen used to create [`HexStr`]s from a [Vec] or a slice, then [to_string_ext](StringExtT::to_string_ext),
    /// or [to_string_ext_with_sep](StringExtT::to_string_ext_with_sep) to get the final string.
    ///
    /// See [`hex_str`] for more details
    pub fn map_hex_slice<const U: bool, const R: usize, const M: usize>(
        inner: &[impl Into<usize> + Copy],
    ) -> impl StringExtT + '_ {
        inner.iter().map(|n| HexStr::hex::<U, R, M>(*n))
    }
}

#[macro_export]
/// Create a [`HexStr`] for a single number or a slice of numbers
///
/// # Exmaples
///
/// ```rust
/// use macro_toolset::{hex_str, string::StringExtT};
///
/// // default lowercase
/// # assert_eq!(
/// hex_str!(0x123456789abc_usize) // 123456789abc
/// # .to_string_ext(), "123456789abc");
///
/// // uppercase
/// # assert_eq!(
/// hex_str!(0x123456789abc_usize; U => true) // 123456789ABC
/// # .to_string_ext(), "123456789ABC");
///
/// // resize
/// # assert_eq!(
/// hex_str!(0x123456789abc_usize; R => 10) // -[12] 3456789abc
/// # .to_string_ext(), "3456789abc");
/// # assert_eq!(
/// hex_str!(0x123456789abc_usize; R => 20) // +[00000000] 123456789abc
/// # .to_string_ext(), "00000000123456789abc");
///
/// // minimum
/// # assert_eq!(
/// hex_str!(0x123456789abc_usize; M => 8) // 123456789abc
/// # .to_string_ext(), "123456789abc");
/// # assert_eq!(
/// hex_str!(0x123456789abc_usize; M => 20) // +[00000000] 123456789abc
/// # .to_string_ext(), "00000000123456789abc");
///
/// // mix then!
/// # assert_eq!(
/// hex_str!(0x123456789abc_usize; U => true; M => 16) // +[0000] 123456789ABC
/// # .to_string_ext(), "0000123456789ABC");
///
/// // Create a temp slice
/// # let _ =
/// hex_str!(SLICE: 0x123456789abc_usize, 0xcba987654321_usize; U => true; R => 0; M => 0)
/// # ;
///
/// ```
///
/// Notice: `U`, `R`, `M` in the macro are optional but should be in order.
macro_rules! hex_str {
    ($dig:expr $(;U => $u:expr)? $(;R => $r:expr)? $(;M => $m:expr)?) => {
        $crate::string::HexStr::new_default($dig)
            $(.set_uppercase::<$u>())?
            $(.set_resize_len::<$r>())?
            $(.set_minimum_len::<$m>())?
    };
    (SLICE: $dig_f:expr, $($dig:expr),+ $(;U => $u:expr)? $(;R => $r:expr)? $(;M => $m:expr)?) => {
        &[
            $crate::hex_str!($dig_f $(;U => $u)? $(;R => $r)? $(;M => $m)?),
            $(
                $crate::string::NumStr::new($dig)
            )+
        ]
    };
    (SLICE: $vec:expr; U => $u:expr; R => $r:expr; M => $m:expr) => {{
        $crate::string::HexStr::map_hex_slice::<$u, $r, $m>($vec)
    }};
    (SLICE: $vec:expr; U => $u:expr; R => $r:expr) => {{
        $crate::string::HexStr::map_hex_slice::<$u, $r, 0>($vec)
    }};
    (SLICE: $vec:expr; U => $u:expr; M => $m:expr) => {{
        $crate::string::HexStr::map_hex_slice::<$u, 0, $m>($vec)
    }};
    (SLICE: $vec:expr; R => $r:expr; M => $m:expr) => {{
        $crate::string::HexStr::map_hex_slice::<false, $r, $m>($vec)
    }};
    (SLICE: $vec:expr; U => $u:expr) => {{
        $crate::string::HexStr::map_hex_slice::<$u, 0, 0>($vec)
    }};
    (SLICE: $vec:expr; R => $r:expr) => {{
        $crate::string::HexStr::map_hex_slice::<false, $r, 0>($vec)
    }};
    (SLICE: $vec:expr; M => $m:expr) => {{
        $crate::string::HexStr::map_hex_slice::<false, 0, $m>($vec)
    }};
}

#[cfg(feature = "macros-string-sha256")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(transparent)]
/// A sha256 hash to string
///
/// # Generic
///
/// - `R`: the resize length of the string, see [`Vec::resize`] for details. 0 means no resize
///   `R` < 32 just means truncate the string to `R` length
/// - `M`: the minimum length of the string, if the length of the string is less than `M`, fill with '0'.
///   `M` <= 32 is not recommended since sha256 hash is always 256 bits, or 32 bytes.
///   0 means no minimum
pub struct Sha256Hash<T: AsRef<str>, const R: usize, const M: usize>(T);

#[cfg(feature = "macros-string-sha256")]
#[macro_export]
/// Create a sha256 hash string
///
/// # Examples
///
/// ```rust
/// # fn main() {
/// # use macro_toolset::sha256_hash;
/// #
/// assert_eq!(sha256_hash!("hello"), "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
/// assert_eq!(sha256_hash!("hello", 12), "2cf24dba5fb0");
/// # }
/// ```
macro_rules! sha256_hash {
    ($h:expr) => {{
        use macro_toolset::string::StringExtT;
        macro_toolset::string::Sha256Hash::<_, 0, 0>::from($h).to_string_ext()
    }};
    ($h:expr, $r:expr) => {{
        use macro_toolset::string::StringExtT;
        macro_toolset::string::Sha256Hash::<_, $r, 0>::from($h).to_string_ext()
    }};
}

#[cfg(feature = "macros-string-sha256")]
impl<T: AsRef<str>, const R: usize, const M: usize> From<T> for Sha256Hash<T, R, M> {
    #[inline]
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

#[cfg(feature = "macros-string-sha256")]
impl<T: AsRef<str>, const R: usize, const M: usize> Sha256Hash<T, R, M> {
    #[inline]
    fn resize(vec: &mut Vec<u8>) {
        if R != 0 {
            vec.resize(R, b'0');
        } else if M != 0 && vec.len() < M {
            vec.resize(M, b'0');
        }
    }

    #[inline]
    fn encode(self) -> Vec<u8> {
        let str = self.0.as_ref();

        if str.is_empty() {
            let mut vec = Vec::with_capacity(128);
            Self::resize(&mut vec);
            vec
        } else {
            let mut sha256 = const_hex::encode(Sha256::digest(str)).into_bytes();
            Self::resize(&mut sha256);
            sha256
        }
    }
}

#[cfg(feature = "macros-string-sha256")]
impl<T: AsRef<str>, const R: usize, const M: usize> StringExtT for Sha256Hash<T, R, M> {
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        string.extend(self.encode())
    }

    #[inline]
    fn to_string_ext(self) -> String {
        unsafe { String::from_utf8_unchecked(self.encode()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_str() {
        // dec / hex without resize or minimum
        assert_eq!(DecStr::new_default(0usize).to_string_ext(), "0");
        assert_eq!(HexStr::hex::<false, 0, 0>(0usize).to_string_ext(), "0");
        assert_eq!(HexStr::hex::<false, 0, 0>(0usize).to_string_ext(), "0");
        assert_eq!(HexStr::hex::<true, 0, 0>(0usize).to_string_ext(), "0");
        assert_eq!(
            DecStr::new_default(usize::MAX).to_string_ext(),
            format!("{}", usize::MAX)
        );
        assert_eq!(
            HexStr::hex::<false, 0, 0>(usize::MAX).to_string_ext(),
            format!("{:x}", usize::MAX)
        );
        assert_eq!(
            HexStr::hex::<false, 0, 0>(usize::MAX).to_string_ext(),
            format!("{:x}", usize::MAX)
        );
        assert_eq!(
            HexStr::hex::<true, 0, 0>(usize::MAX).to_string_ext(),
            format!("{:X}", usize::MAX)
        );

        assert_eq!(DecStr::new_default(123456usize).to_string_ext(), "123456");
        assert_eq!(
            HexStr::hex::<false, 0, 0>(0x123456usize).to_string_ext(),
            "123456"
        );
        assert_eq!(
            HexStr::hex::<false, 0, 0>(0xabcdeusize).to_string_ext(),
            "abcde"
        );
        assert_eq!(
            HexStr::hex::<true, 0, 0>(0xabcdeusize).to_string_ext(),
            "ABCDE"
        );

        // dec / hex with resize
        assert_eq!(DecStr::dec::<5, 0>(123456usize).to_string_ext(), "23456");
        assert_eq!(DecStr::dec::<6, 0>(123456usize).to_string_ext(), "123456");
        assert_eq!(DecStr::dec::<7, 0>(123456usize).to_string_ext(), "0123456");
        assert_eq!(
            HexStr::hex::<false, 5, 0>(0x123456usize).to_string_ext(),
            "23456"
        );
        assert_eq!(
            HexStr::hex::<false, 6, 0>(0x123456usize).to_string_ext(),
            "123456"
        );
        assert_eq!(
            HexStr::hex::<false, 7, 0>(0x123456usize).to_string_ext(),
            "0123456"
        );
        assert_eq!(
            HexStr::hex::<false, 5, 0>(0xabcdefusize).to_string_ext(),
            "bcdef"
        );
        assert_eq!(
            HexStr::hex::<false, 6, 0>(0xabcdefusize).to_string_ext(),
            "abcdef"
        );
        assert_eq!(
            HexStr::hex::<false, 7, 0>(0xabcdefusize).to_string_ext(),
            "0abcdef"
        );
        assert_eq!(
            HexStr::hex::<true, 5, 0>(0xabcdefusize).to_string_ext(),
            "BCDEF"
        );
        assert_eq!(
            HexStr::hex::<true, 6, 0>(0xabcdefusize).to_string_ext(),
            "ABCDEF"
        );
        assert_eq!(
            HexStr::hex::<true, 7, 0>(0xabcdefusize).to_string_ext(),
            "0ABCDEF"
        );

        // dec / hex with minimum
        assert_eq!(DecStr::dec::<0, 5>(123456usize).to_string_ext(), "123456");
        assert_eq!(DecStr::dec::<0, 6>(123456usize).to_string_ext(), "123456");
        assert_eq!(DecStr::dec::<0, 7>(123456usize).to_string_ext(), "0123456");
        assert_eq!(
            HexStr::hex::<false, 0, 5>(0x123456usize).to_string_ext(),
            "123456"
        );
        assert_eq!(
            HexStr::hex::<false, 0, 6>(0x123456usize).to_string_ext(),
            "123456"
        );
        assert_eq!(
            HexStr::hex::<false, 0, 7>(0x123456usize).to_string_ext(),
            "0123456"
        );
        assert_eq!(
            HexStr::hex::<false, 0, 5>(0xabcdefusize).to_string_ext(),
            "abcdef"
        );
        assert_eq!(
            HexStr::hex::<false, 0, 6>(0xabcdefusize).to_string_ext(),
            "abcdef"
        );
        assert_eq!(
            HexStr::hex::<false, 0, 7>(0xabcdefusize).to_string_ext(),
            "0abcdef"
        );
        assert_eq!(
            HexStr::hex::<true, 0, 5>(0xabcdefusize).to_string_ext(),
            "ABCDEF"
        );
        assert_eq!(
            HexStr::hex::<true, 0, 6>(0xabcdefusize).to_string_ext(),
            "ABCDEF"
        );
        assert_eq!(
            HexStr::hex::<true, 0, 7>(0xabcdefusize).to_string_ext(),
            "0ABCDEF"
        );

        // dec / hex with resize as well as minimum
        assert_eq!(DecStr::dec::<5, 4>(123456usize).to_string_ext(), "23456");
        assert_eq!(DecStr::dec::<5, 5>(123456usize).to_string_ext(), "23456");
        assert_eq!(DecStr::dec::<5, 6>(123456usize).to_string_ext(), "23456");
        assert_eq!(DecStr::dec::<6, 7>(123456usize).to_string_ext(), "123456");

        assert_eq!(
            HexStr::hex::<false, 5, 4>(0x123456usize).to_string_ext(),
            "23456"
        );
        assert_eq!(
            HexStr::hex::<false, 5, 5>(0x123456usize).to_string_ext(),
            "23456"
        );
        assert_eq!(
            HexStr::hex::<false, 5, 6>(0x123456usize).to_string_ext(),
            "23456"
        );
        assert_eq!(
            HexStr::hex::<false, 6, 7>(0x123456usize).to_string_ext(),
            "123456"
        );

        assert_eq!(
            HexStr::hex::<false, 5, 4>(0xabcdefusize).to_string_ext(),
            "bcdef"
        );
        assert_eq!(
            HexStr::hex::<false, 5, 5>(0xabcdefusize).to_string_ext(),
            "bcdef"
        );
        assert_eq!(
            HexStr::hex::<false, 5, 6>(0xabcdefusize).to_string_ext(),
            "bcdef"
        );
        assert_eq!(
            HexStr::hex::<false, 6, 7>(0xabcdefusize).to_string_ext(),
            "abcdef"
        );

        assert_eq!(
            HexStr::hex::<true, 5, 4>(0xabcdefusize).to_string_ext(),
            "BCDEF"
        );
        assert_eq!(
            HexStr::hex::<true, 5, 5>(0xabcdefusize).to_string_ext(),
            "BCDEF"
        );
        assert_eq!(
            HexStr::hex::<true, 5, 6>(0xabcdefusize).to_string_ext(),
            "BCDEF"
        );
        assert_eq!(
            HexStr::hex::<true, 6, 7>(0xabcdefusize).to_string_ext(),
            "ABCDEF"
        );
    }

    #[cfg(feature = "macros-string-sha256")]
    #[test]
    fn test_sha256() {
        fn hash12(s: impl AsRef<str>) -> String {
            use sha2::{Digest as _, Sha256};

            let s = s.as_ref();
            if s.is_empty() {
                "000000000000".to_owned()
            } else {
                let mut sha256 = const_hex::encode(Sha256::digest(s));
                // SAFE: sha256 hex string must be longer than 12
                sha256.truncate(12);
                sha256
            }
        }

        assert_eq!(Sha256Hash::<_, 12, 0>::from("").to_string_ext(), hash12(""));
        assert_eq!(
            Sha256Hash::<_, 12, 0>::from("0123456789").to_string_ext(),
            hash12("0123456789")
        );
    }

    #[test]
    fn test_string_ext() {
        let mut s = StringExt::with_capacity(32);

        // char
        s.push('[');
        s.push('�');
        s.push('😀');
        s.push(['A']);
        s.push(&['a', '1'][..]);
        s.push(&']');

        // &str
        s.push(vec!["[Hello World � 😀]", "[你好 � 😀]"]);

        // String
        s.push_with_separator("[Hello World � 😀".to_owned(), ']');
        s.push(Some("[你好 � 😀]".to_owned()));

        // Cow<str>
        s.push(Cow::Borrowed("[Hello World � 😀]"));
        s.push(Cow::Owned("[你好 � 😀]".to_owned()));

        // number
        s.push(0u8);
        s.push(123usize);
        s.push(45u8);
        s.push(6789u16);
        s.push(123456789u32);
        s.push(123456789i32);
        s.push(Box::new(-123456789i32));

        // HexStr
        s.push(HexStr::hex::<false, 0, 0>(0xabcdefusize));
        s.push(HexStr::hex::<true, 0, 0>(0xabcdefusize));

        assert_eq!(
            s.as_str(),
            "[�😀Aa1][Hello World � 😀][你好 � 😀][Hello World � 😀][你好 � 😀][Hello World � 😀][你好 � 😀]0123456789123456789123456789-123456789abcdefABCDEF"
        );
    }

    #[test]
    fn test_separator() {
        let mut s = StringExt::with_capacity(128);

        s.push_with_separator("Hello", ',');
        s.push_with_separator("World", ',');
        s.push_with_separator(vec!["你好", "世界"], ',');
        s.push_with_separator(Some(vec![vec!["你好"], vec!["世界"]]), ',');
        s.push_with_separator(Some(&["你好", "世界"]), ',');
        s.push_with_separator(
            (
                "你好",
                "世界",
                vec!["你好", "世界"],
                Some(&["你好", "世界"]),
                None::<&str>,
            ),
            ',',
        );

        assert_eq!(
            s.into_string_remove_tail(','),
            "Hello,World,你好,世界,你好,世界,你好,世界,你好,世界,你好,世界,你好,世界"
        );
    }
}
