//! useful [`StringExt`] utilities for [`crate::str_concat`] macros

#[cfg(feature = "feat-string-ext-base64")]
mod base64;
#[cfg(feature = "feat-string-ext-hex")]
mod hex;
mod number;
#[cfg(feature = "feat-string-ext-rand")]
mod rand;
mod slice_sep;

use std::{borrow::Cow, ops, rc::Rc, sync::Arc};

#[cfg(feature = "feat-string-ext-base64")]
// re-export
pub use base64::{b64_padding, Base64Str};
#[cfg(feature = "feat-string-ext-hex")]
// re-export
pub use hex::HexStr;
// re-export
pub use number::NumStr;
#[cfg(feature = "feat-string-ext-rand")]
pub use rand::{RandHexStr, RandStr};
// re-export
pub use slice_sep::SliceSep;

#[macro_export]
/// Fast concat [`String`] / &[`str`] / number.
///
/// For details of params accepted, please refers to [`StringExtT`].
///
/// # Examples
///
/// ```rust
/// # use macro_toolset::{str_concat, string::NumStr};
/// # fn main() {
/// // General usage
/// assert_eq!(
///     str_concat!(
///         NumStr::hex_default(0xa_usize), // HexStr
///         "b", // &str
///         "c".to_string(), // String
///         1u8, // single number
///         '😀', // char
///         '�' // char
///     ), "abc1😀�"
/// );
/// // with initial string
/// assert_eq!(
///    str_concat!(str = "abc"; "1", "😀", "�"), "abc1😀�"
/// );
///
/// // with capacity
/// assert_eq!(
///    str_concat!(cap = 10; "abc", "1", "😀", "�"), "abc1😀�"
/// );
///
/// // with separator
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
            let mut string_final = $crate::string::StringExt::from($str);
            $(
                string_final.push($x);
            )*
            string_final.into_string()
        }
    };
    (cap = $cap:expr; $($x:expr),*) => {
        {
            let mut string_final = $crate::string::StringExt::with_capacity($cap);
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

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
/// A string with extended utilities
///
/// In most circumstances, it can be considered as a [`String`] with more
/// utilities.
pub struct StringExt {
    inner: Vec<u8>,
}

impl StringExt {
    #[inline]
    #[must_use]
    /// Create a new [`StringExt`] with initial capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: Vec::with_capacity(cap),
        }
    }

    #[inline]
    /// Create a new [`StringExt`] with given value that implements
    /// [`StringExtT`].
    ///
    /// Most time you don't need to use this function but [`str_concat`] macro,
    /// but sometime you may need the [`StringExt`] itself and this function is
    /// what you need.
    ///
    /// # Example
    ///
    /// ```
    /// # use macro_toolset::string::StringExt;
    /// let example_string_ext = StringExt::from_value("Hello, world!");
    /// let example_header_value: http::HeaderValue = example_string_ext.try_into().unwrap();
    /// ```
    pub fn from_value(value: impl StringExtT) -> Self {
        let mut this = Self::with_capacity(64);
        this.push(value);
        this
    }

    #[inline]
    /// Create a new [`StringExt`] with given value that implements
    /// [`StringExtT`].
    ///
    /// See [`from_value`](Self::from_value) for more information.
    pub fn from_value_with_separator(value: impl StringExtT, separator: impl SeparatorT) -> Self {
        let mut this = Self::with_capacity(64);
        this.push_with_separator(value, separator);
        this
    }

    #[allow(unsafe_code)]
    #[inline]
    #[must_use]
    /// Converts a vector of bytes to a String without checking that the
    /// string contains valid UTF-8.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the bytes passed
    /// to it are valid UTF-8. If this constraint is violated, undefined
    /// behavior results.
    pub unsafe fn from_utf8_unchecked(bytes: Vec<u8>) -> Self {
        Self { inner: bytes }
    }

    #[allow(unsafe_code)]
    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    /// Consume self and get the inner Vec<u8>
    ///
    /// # Safety
    ///
    /// Just mark the inner Vec<u8> as mutable, but it's not safe to modify the
    /// Vec<u8> directly and you must ensure the Vec<u8> is always valid
    /// utf8
    pub unsafe fn into_bytes(self) -> Vec<u8> {
        self.inner
    }

    #[inline(always)]
    #[must_use]
    /// Extracts a string slice containing the entire [`StringExt`].
    pub fn as_str(&self) -> &str {
        #[allow(unsafe_code)]
        // Safety: the inner Vec<u8> is always valid utf-8
        unsafe {
            std::str::from_utf8_unchecked(self.inner.as_slice())
        }
    }

    #[inline(always)]
    #[must_use]
    /// Converts a [`StringExt`] into a mutable string slice.
    pub fn as_mut_str(&mut self) -> &mut str {
        // Safety: the inner Vec<u8> is always valid utf-8
        #[allow(unsafe_code)]
        unsafe {
            std::str::from_utf8_unchecked_mut(self.inner.as_mut_slice())
        }
    }

    #[inline(always)]
    /// Push a value to the string, which implements [`StringExtT`].
    pub fn push(&mut self, value: impl StringExtT) {
        value.push_to_string(&mut self.inner);
    }

    #[inline(always)]
    /// Push a value to the string ends with given separator, which both
    /// implement [`StringExtT`]
    ///
    /// Note: the separator will be pushed even if the value is empty.
    ///
    /// See [`StringExt::into_string_remove_tail`] for more details.
    pub fn push_with_separator(&mut self, value: impl StringExtT, separator: impl SeparatorT) {
        value.push_to_string_with_separator(&mut self.inner, separator);
    }

    /// Returns this [`StringExt`]'s capacity, in bytes.
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

    #[inline(always)]
    /// Returns a byte slice of this [`StringExt`]'s contents.
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }

    #[inline(always)]
    #[must_use]
    /// Get the length of the string
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline(always)]
    #[must_use]
    /// Returns true if this String has a length of zero, and false
    /// otherwise.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[allow(unsafe_code)]
    #[inline(always)]
    /// Borrow the inner Vec<u8>
    ///
    /// # Safety
    ///
    /// Just mark the inner Vec<u8> as mutable, but it's not safe to modify the
    /// Vec<u8> directly and you must ensure the Vec<u8> is always valid
    /// utf8
    pub unsafe fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }

    #[inline(always)]
    /// Consume self and get the final String.
    pub fn into_string(self) -> String {
        // Safety: the inner Vec<u8> is always valid utf-8
        #[allow(unsafe_code)]
        unsafe {
            String::from_utf8_unchecked(self.inner)
        }
    }

    #[inline]
    /// Consume self and get the final String, removing the separator from the
    /// end of the string
    ///
    /// Notice: the separator char / string will be removed, even if it's from
    /// the original string!
    pub fn into_string_remove_tail(mut self, separator: impl SeparatorT) -> String {
        separator.remove_end(&mut self.inner);
        self.into_string()
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
        self.as_str()
    }
}

impl AsRef<[u8]> for StringExt {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl ops::Deref for StringExt {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(feature = "feat-string-ext-axum")]
impl axum_core::response::IntoResponse for StringExt {
    #[inline]
    fn into_response(mut self) -> axum_core::response::Response {
        // Avoid an extra allocation if possible.
        self.inner.truncate(self.inner.len());

        axum_core::response::Response::new(bytes::Bytes::from(self.inner).into())
    }
}

#[cfg(feature = "feat-string-ext-http")]
impl TryInto<http::HeaderName> for StringExt {
    type Error = http::header::InvalidHeaderName;

    fn try_into(self) -> Result<http::HeaderName, Self::Error> {
        http::HeaderName::from_bytes(&self.inner)
    }
}

#[cfg(feature = "feat-string-ext-http")]
impl TryInto<http::HeaderValue> for StringExt {
    type Error = http::header::InvalidHeaderValue;

    fn try_into(mut self) -> Result<http::HeaderValue, Self::Error> {
        // Avoid an extra allocation if possible.
        self.inner.truncate(self.inner.len());

        http::HeaderValue::from_maybe_shared(bytes::Bytes::from(self.inner))
    }
}

// === Core Impls ===

/// Trait for pushing any value that can be converted into str to [`StringExt`]
///
/// If needed, you can implement this trait for your own type.
///
/// The type we support currently:
///
/// - [`()`]: will be a no-op
/// - [`bool`]: string `true` or `false`.
/// - [`char`]
/// - Any (smart) pointer that implements [`ops::Deref`] with target `str`.
///
///   Since the compiler complains with MAYBE IMPLEMENTED BY UPSTREAM, we have
///   to do so manually.
///
///   - &[`str`]
///   - [`String`]
///   - [`Rc<str>`]
///   - [`Rc<String>`]
///   - [`Arc<str>`]
///   - [`Arc<String>`]
///   - [`Box<String>`] // Actually meanless
///   - [`Cow<str>`]
/// - Numbers, see [`NumStr`]
/// - Hex string, see [`HexStr`], including [`const_hex::Buffer`]
/// - Slice of any type that implements [`StringExtT`], including &[\[T\]] or
///   [\[T; N\]],
/// - [`Vec`] of any type that implements [`StringExtT`]
/// - Iterator with item that implements [`StringExtT`]
///
///   Since the compiler complains with MAYBE IMPLEMENTED BY UPSTREAM, we have
///   to do so manually.
///
///   - [`std::iter::Map`]
/// - [`Box`] of any type that implements [`StringExtT`]
/// - [`Option`] of any type that implements [`StringExtT`]
/// - [`Result<T, E>`], `T` implements [`StringExtT`]
/// - Tuple of any type that implements [`StringExtT`]
/// - Any type that implements [`Copy`], just copy it.
///
///   Since the compiler complains with MAYBE IMPLEMENTED BY UPSTREAM, we have
///   to do so manually.
///
///   - [`bool`]
///   - [`char`]
///   - &[`str`]
///   - Numbers, including
///     - [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], [`usize`],
///     - [`i8`], [`i16`], [`i32`], [`i64`], [`i128`], [`isize`]
pub trait StringExtT: Sized {
    /// Push the value to the string.
    fn push_to_string(self, string: &mut Vec<u8>);

    #[inline]
    /// Push the value to the string with a separator.
    ///
    /// ! Separator should implement [`SeparatorT`]
    fn push_to_string_with_separator(self, string: &mut Vec<u8>, separator: impl SeparatorT) {
        self.push_to_string(string);
        separator.push_to_string(string);
    }

    #[inline]
    /// Encode the value to the string.
    fn to_string_ext(self) -> String {
        let mut string = StringExt::with_capacity(128);
        string.push(self);
        string.into_string()
    }

    #[inline]
    /// Push the value to the string with separator
    ///
    /// ! Separator should implement [`SeparatorT`].
    fn to_string_ext_with_sep(self, separator: impl SeparatorT) -> String {
        let mut string = StringExt::with_capacity(128);
        string.push_with_separator(self, separator);
        string.into_string_remove_tail(separator)
    }

    #[inline]
    /// With prefix.
    ///
    /// This is different from simple tuple, since this will not add a separator
    /// between the prefix and the value.
    fn with_prefix<P: StringExtT>(self, prefix: P) -> impl StringExtT {
        SeplessTuple((prefix, self))
    }

    #[inline]
    /// With suffix.
    ///
    /// This is different from simple tuple, since this will not add a separator
    /// between the suffix and the value.
    fn with_suffix<S: StringExtT>(self, suffix: S) -> impl StringExtT {
        SeplessTuple((self, suffix))
    }
}

impl StringExtT for () {
    #[inline]
    fn push_to_string(self, _: &mut Vec<u8>) {}

    #[inline]
    fn push_to_string_with_separator(self, _string: &mut Vec<u8>, _separator: impl SeparatorT) {}

    #[inline]
    fn to_string_ext(self) -> String {
        String::new()
    }

    #[inline]
    fn to_string_ext_with_sep(self, _separator: impl SeparatorT) -> String {
        String::new()
    }
}

impl StringExtT for bool {
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        string.extend(if self { &b"true"[..] } else { &b"false"[..] });
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

/// Any (smart) pointer that implements [`ops::Deref`] with target `str`.
///
/// Since the compiler complains with MAYBE IMPLEMENTED BY UPSTREAM, we have to
/// do so manually.
///
///   - &[`str`]
/// - Smart pointer types:
///   - [`String`]
///   - [`Rc<str>`]
///   - [`Rc<String>`]
///   - [`Arc<str>`]
///   - [`Arc<String>`]
///   - [`Cow<str>`]
macro_rules! impl_for_string {
    ($($ty:ty),*) => {
        $(
            impl_for_string!(INTERNAL IMPL $ty);
            impl_for_string!(INTERNAL IMPL &$ty);
            impl_for_string!(INTERNAL IMPL &mut $ty);
            impl_for_string!(INTERNAL IMPL &&$ty);
            impl_for_string!(INTERNAL IMPL &&mut $ty);
            impl_for_string!(INTERNAL IMPL &mut &$ty); // make no sense?
            impl_for_string!(INTERNAL IMPL &mut &mut $ty);
        )*
    };
    (INTERNAL IMPL $ty:ty) => {
        impl StringExtT for $ty {
            #[inline]
            fn push_to_string(self, string: &mut Vec<u8>) {
                string.extend(self.as_bytes());
            }

            #[inline]
            fn to_string_ext(self) -> String {
                // optimize for String
                self.to_string()
            }

            #[inline]
            fn to_string_ext_with_sep(self, separator: impl SeparatorT) -> String {
                let mut string = StringExt::with_capacity(self.len() + 4);
                string.push_with_separator(self, separator);
                string.into_string_remove_tail(separator)
            }
        }
    }
}

impl_for_string!(
    &str,
    String,
    Rc<str>,
    Rc<String>,
    Arc<str>,
    Arc<String>,
    Cow<'_, str>
);

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

    #[inline]
    fn with_prefix<P: StringExtT>(self, prefix: P) -> impl StringExtT {
        self.map(|item| SeplessTuple((prefix, item)))
    }

    #[inline]
    fn with_suffix<S: StringExtT>(self, suffix: S) -> impl StringExtT {
        self.map(|item| SeplessTuple((item, suffix)))
    }
}

impl<T, E> StringExtT for Result<T, E>
where
    T: StringExtT,
{
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        if let Ok(item) = self {
            item.push_to_string(string);
        }
    }

    #[inline]
    fn push_to_string_with_separator(self, string: &mut Vec<u8>, separator: impl SeparatorT) {
        if let Ok(item) = self {
            item.push_to_string_with_separator(string, separator);
        }
    }

    #[inline]
    fn with_prefix<P: StringExtT>(self, prefix: P) -> impl StringExtT {
        self.map(|item| SeplessTuple((prefix, item)))
    }

    #[inline]
    fn with_suffix<S: StringExtT>(self, suffix: S) -> impl StringExtT {
        self.map(|item| SeplessTuple((item, suffix)))
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
/// A tuple with no separator inserted between elements.
pub struct SeplessTuple<T: StringExtT>(pub T);

/// - Tuple of any type that implements [`StringExtT`]
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

        #[allow(non_snake_case)]
        impl<$($name: StringExtT),+> StringExtT for SeplessTuple<($($name,)+)>
        {
            #[inline]
            fn push_to_string(self, string: &mut Vec<u8>) {
                let ($($name,)+) = self.0;
                $($name.push_to_string(string);)+
            }
        }
    };
}

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

/// - Any type that implements [`Copy`], just copy it.
///
///   Since the compiler complains with MAYBE IMPLEMENTED BY UPSTREAM, we have
///   to do so manually.
///
///   - [`bool`]
///   - [`char`]
///   - &[`str`]
///   - Numbers, including
///     - [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], [`usize`],
///     - [`i8`], [`i16`], [`i32`], [`i64`], [`i128`], [`isize`]
macro_rules! impl_ref_deref {
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

            impl StringExtT for &mut $ty {
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
    ($($($ge:ident),* => $ty:ty),*) => {
        $(
            impl<$($ge: StringExtT)*> StringExtT for $ty {
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

impl_ref_deref!(bool, &bool, char, &char);
impl_ref_deref!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
impl_ref_deref!(
    &u8, &u16, &u32, &u64, &u128, &usize, &i8, &i16, &i32, &i64, &i128, &isize, &f32, &f64
);
impl_ref_deref!(T => Box<T>);

// === impl for other types

#[cfg(feature = "feat-string-ext-ammonia")]
impl StringExtT for ammonia::Document {
    fn push_to_string(self, string: &mut Vec<u8>) {
        self.write_to(string)
            .expect("Writing to a string should not fail (except on OOM)");
    }
}

#[cfg(feature = "feat-string-ext-chrono")]
impl<'a, I: Iterator<Item = B> + Clone, B: std::borrow::Borrow<chrono::format::Item<'a>>> StringExtT
    for chrono::format::DelayedFormat<I>
{
    fn push_to_string(self, string: &mut Vec<u8>) {
        // TODO: Avoid allocation here, though chrono doesn't provide a way to do so.
        string.extend(self.to_string().as_bytes());
    }
}

#[cfg(feature = "feat-string-ext-http")]
impl StringExtT for http::HeaderName {
    fn push_to_string(self, string: &mut Vec<u8>) {
        string.extend(self.as_str().as_bytes());
    }
}

#[cfg(feature = "feat-string-ext-http")]
impl StringExtT for http::Method {
    fn push_to_string(self, string: &mut Vec<u8>) {
        string.extend(self.as_str().as_bytes());
    }
}

#[cfg(feature = "feat-string-ext-http")]
impl StringExtT for http::StatusCode {
    fn push_to_string(self, string: &mut Vec<u8>) {
        string.extend(self.as_str().as_bytes());
    }
}

#[cfg(feature = "feat-string-ext-http")]
impl StringExtT for http::Uri {
    fn push_to_string(self, string: &mut Vec<u8>) {
        if let Some(scheme) = self.scheme() {
            string.extend(scheme.as_str().as_bytes());
            string.extend(b"://");
        }

        if let Some(authority) = self.authority() {
            string.extend(authority.as_str().as_bytes());
        }

        string.extend(self.path().as_bytes());

        if let Some(query) = self.query() {
            string.push(b'?');
            string.extend(query.as_bytes());
        }
    }
}

#[cfg(feature = "feat-string-ext-http")]
impl StringExtT for http::Version {
    fn push_to_string(self, string: &mut Vec<u8>) {
        let str_byte = match self {
            http::Version::HTTP_09 => &b"HTTP/0.9"[..],
            http::Version::HTTP_10 => &b"HTTP/1.0"[..],
            http::Version::HTTP_11 => &b"HTTP/1.1"[..],
            http::Version::HTTP_2 => &b"HTTP/2.0"[..],
            http::Version::HTTP_3 => &b"HTTP/3.0"[..],
            _ => {
                string.extend(format!("{self:?}").as_bytes());
                return;
            }
        };

        string.extend(str_byte);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_or_suffix() {
        let exp1 = "world".with_prefix("hello");
        assert_eq!(exp1.to_string_ext(), "helloworld");

        let exp2 = str_concat!(sep = ' '; ("hello", "world"));
        assert_eq!(exp2, "hello world");

        let exp3 = str_concat!(
            sep = ' ';
            ("hello", "world"),
            "world".with_prefix("prefix-"),
            "2world".with_prefix("2prefix-"),
            "hello".with_suffix(Some("-suffix")),
            "3hello".with_suffix(None::<()>),
            None::<()>.with_suffix("-suffix").with_prefix("prefix-")
        );
        assert_eq!(
            exp3,
            "hello world prefix-world 2prefix-2world hello-suffix 3hello"
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
        s.push(']');

        // &str
        s.push(vec!["[Hello World � 😀]", "[你好 � 😀]"]);

        // String
        s.push_with_separator("[Hello World � 😀".to_owned(), ']');
        s.push(Some("[你好 � 😀]".to_owned()));

        // Cow<str>
        s.push(Cow::Borrowed("[Hello World � 😀]"));
        s.push(Cow::Owned("[你好 � 😀]".to_string()));

        // number
        s.push(0u8);
        s.push(123usize);
        s.push(45u8);
        s.push(6789u16);
        s.push(123456789u32);
        s.push(123456789i32);
        s.push(Box::new(-123456789i32));

        // HexStr
        s.push(NumStr::hex_default(0xabcdefusize));
        s.push(NumStr::hex_default(0xabcdefusize).set_uppercase::<true>());

        assert_eq!(
            s.as_str(),
            "[�😀Aa1][Hello World � 😀][你好 � 😀][Hello World � 😀][你好 � 😀][Hello World � \
             😀][你好 � 😀]0123456789123456789123456789-123456789abcdefABCDEF"
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
