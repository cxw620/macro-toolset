//! Random string.

use rand::{distributions::Slice, Rng};

use super::{NumStr, StringExtT, StringT};
use crate::random::fast_random;

#[macro_export]
/// See [`RandStr`] and [`RandHexStr`] for more information.
///
/// # Example
///
/// ```
/// use macro_toolset::{random_str, string_v2::PushAnyT};
/// let mut string = String::new();
/// string.push_any(random_str!(16, b"abcABC123"));
/// string.push_any(random_str!(HEX)); // 16 (default, max 16) * 1 (default) + 0 (default, max 16)
/// string.push_any(random_str!(HEX: 16)); // 16 * 1 (default) + 0 (default)
/// string.push_any(random_str!(HEX: 16, 3)); // 16 * 3 + 0 (default)
/// string.push_any(random_str!(HEX: 16, 3, 8)); // 16 * 3 + 8
/// ```
///
/// // If you like, just `to_string_ext` is fine.
/// ```
/// use macro_toolset::{random_str, string_v2::StringExtT};
/// let string = random_str!(16, b"abcABC123").to_string_ext();
/// ```
macro_rules! random_str {
    ($range:expr, $charset:expr) => {{
        $crate::string_v2::rand::RandStr::<$range>::with_charset($charset)
    }};
    (HEX) => {{
        $crate::string_v2::rand::RandHexStr::new_default()
    }};
    (HEX: $l:expr) => {{
        $crate::string_v2::rand::RandHexStr::<$l>::new()
    }};
    (HEX: $l:expr, $rp:expr) => {{
        $crate::string_v2::rand::RandHexStr::<$l, $rp>::new()
    }};
    (HEX: $l:expr, $rp:expr, $lp:expr) => {{
        $crate::string_v2::rand::RandHexStr::<$l, $rp, $lp>::new()
    }};
}

#[derive(Debug, Clone, Copy, Default)]
/// Randon hex-like string, with fix length.
///
/// For better performance, the underlying random number is generated by
/// xorshift algorithm then converted to hex string with [`NumStr`].
///
/// By default, the length is 16.
///
/// # Generic Parameters
///
/// - `L`: The length of the string. Max 16 (u64).
/// - `RP`: Repeat `L` for `RP` times.
/// - `LP`: Lefted length. Max 16.
///
/// For example, if you need a string with length 56, you may specify `L` as 16,
/// `RP` as 56 / 16 = 3, and `LP` as 56 % 16 = 8.
///
/// Since `#![feature(generic_const_exprs)]` is not stable, we have to make use
/// of these complex const generics.
///
/// Notice: will check if params are valid when you push this to a string, or
/// panic in debug mode, work normally but slower in release mode.
pub struct RandHexStr<const L: usize = 16, const RP: usize = 1, const LP: usize = 0>;

impl<const L: usize, const RP: usize, const LP: usize> StringT for RandHexStr<L, RP, LP> {
    #[inline]
    fn encode_to_buf(self, string: &mut Vec<u8>) {
        match L {
            1..=16 => {
                for _ in 0..RP {
                    NumStr::hex_default(fast_random())
                        .set_resize_len::<L>()
                        .encode_to_buf(string);
                }

                if LP > 0 {
                    debug_assert!(LP <= 16, "LP should be 0..=16");

                    NumStr::hex_default(fast_random())
                        .set_resize_len::<LP>()
                        .encode_to_buf(string);
                }
            }
            0 => {}
            _ => {
                #[cfg(any(debug_assertions, test))]
                unreachable!("L should be 0..=16");

                #[cfg(not(any(debug_assertions, test)))]
                // For RELEASE mode, avoid panic but still generate random string like general
                // RandStr does.
                string.extend(
                    rand::thread_rng()
                        .sample_iter(&Slice::new(b"0123456789abcdef").unwrap())
                        .take(L * RP + LP),
                );
            }
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
        match L {
            1..=16 => {
                for _ in 0..RP {
                    NumStr::hex_default(fast_random())
                        .set_resize_len::<L>()
                        .encode_to_bytes_buf(string);
                }

                if LP > 0 {
                    debug_assert!(LP <= 16, "LP should be 0..=16");

                    NumStr::hex_default(fast_random())
                        .set_resize_len::<LP>()
                        .encode_to_bytes_buf(string);
                }
            }
            0 => {}
            _ => {
                #[cfg(any(debug_assertions, test))]
                unreachable!("L should be 0..=16");

                #[cfg(not(any(debug_assertions, test)))]
                // For RELEASE mode, avoid panic but still generate random string like general
                // RandStr does.
                string.extend(
                    rand::thread_rng()
                        .sample_iter(&Slice::new(b"0123456789abcdef").unwrap())
                        .take(L * RP + LP),
                );
            }
        }
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
        self.encode_to_bytes_buf(string);
        string.extend(separator.as_bytes());
    }
}

impl<const L: usize, const RP: usize, const LP: usize> StringExtT for RandHexStr<L, RP, LP> {}

impl RandHexStr {
    #[inline]
    /// Create a new [`RandHexStr`] and generate simple random hex-like string
    /// with length 16 (default).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use macro_toolset::string::{RandHexStr, StringExtT};
    /// let random_str = RandHexStr::new_default().to_string_ext();
    /// assert_eq!(random_str.len(), 16);
    /// ```
    pub const fn new_default() -> Self {
        Self
    }
}

impl<const L: usize, const RP: usize, const LP: usize> RandHexStr<L, RP, LP> {
    #[inline]
    /// Create a new [`RandStr`] and generate random hex-like string with
    /// length setting by `L`, `RP`, `LP`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use macro_toolset::string::{RandHexStr, StringExtT};
    /// let random_str = RandHexStr::<16, 3, 8>::new().to_string_ext();
    /// assert_eq!(random_str.len(), 56);
    /// ```
    pub const fn new() -> Self {
        RandHexStr
    }

    #[inline]
    /// Set `L`.
    ///
    /// You may prefer [`RandHexStr::<L, RP, LP>::new`](Self::new).
    pub const fn with_l<const NL: usize>(self) -> RandHexStr<NL, RP, LP> {
        RandHexStr
    }

    #[inline]
    /// Set `RP`.
    ///
    /// You may prefer [`RandHexStr::<L, RP, LP>::new`](Self::new).
    pub const fn with_rp<const NRP: usize>(self) -> RandHexStr<L, NRP, LP> {
        RandHexStr
    }

    #[inline]
    /// Set `LP`.
    ///
    /// You may prefer [`RandHexStr::<L, RP, LP>::new`](Self::new).
    pub const fn with_lp<const NLP: usize>(self) -> RandHexStr<L, RP, NLP> {
        RandHexStr
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
/// Randon string, with fix length and given charset.
///
/// # Generic Parameters
///
/// - `L`: The length of the string. Default is 32.
///
/// Notice: must make sure each u8 within the slice is valid
/// single byte UTF-8 char.
///
/// If the charset is `0123456789abcdef`, [`RandHexStr`] is recommended and 4~6x
/// faster than this (when feature `feat-random-fast` enabled).
pub struct RandStr<'r, const L: usize = 32>(&'r [u8]);

impl<const L: usize> StringT for RandStr<'_, L> {
    #[inline]
    fn encode_to_buf(self, string: &mut Vec<u8>) {
        if self.0.is_empty() {
            return;
        }

        string.extend(
            rand::thread_rng()
                .sample_iter(Slice::new(self.0).unwrap())
                .take(L),
        );
    }

    #[inline]
    fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
        if self.0.is_empty() {
            return;
        }

        string.extend(
            rand::thread_rng()
                .sample_iter(Slice::new(self.0).unwrap())
                .take(L),
        );

        string.extend(separator.as_bytes());
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
        if self.0.is_empty() {
            return;
        }

        string.extend(
            rand::thread_rng()
                .sample_iter(Slice::new(self.0).unwrap())
                .take(L),
        );
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
        if self.0.is_empty() {
            return;
        }

        string.extend(
            rand::thread_rng()
                .sample_iter(Slice::new(self.0).unwrap())
                .take(L),
        );

        string.extend(separator.as_bytes());
    }
}

impl<const L: usize> StringExtT for RandStr<'_, L> {}

impl<'r> RandStr<'r> {
    #[inline]
    /// Create a new [`RandStr`] and generate random string with length
    /// setting by `L`.
    pub const fn with_charset_default(charset: &'r [u8]) -> Self {
        Self(charset)
    }
}

impl<'r, const L: usize> RandStr<'r, L> {
    #[inline]
    /// Create a new [`RandStr`] and generate random string with length
    /// setting by `L`.
    pub const fn with_charset(charset: &'r [u8]) -> Self {
        Self(charset)
    }

    #[inline]
    /// Set `L`.
    pub const fn with_l<const NL: usize>(self) -> RandStr<'r, NL> {
        RandStr(self.0)
    }
}
