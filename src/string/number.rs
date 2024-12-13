//! Number to string, fast and efficient utilities.

use std::{ops, str};

use super::StringExtT;

/// Hexadecimal characters in lower case.
static HEX_CHARS_LOWER: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

/// Hexadecimal characters in upper case.
static HEX_CHARS_UPPER: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
];

// === impls ===

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
#[repr(transparent)]
/// Number to string
///
/// # Generic
///
/// - `B`: the base of the number, should be within the range `2..=16`. Default
///   is 10.
/// - `U`: whether to use uppercase for hex. Default is lowercase (false).
///
///   For float number, `U` means whether only to reserve the integer part.
/// - `R`: the resize length of the string. The overflow part will be truncated,
///   and the insufficient part will be filled with '0'. Default is 0, or no
///   resize.
/// - `M`: the minimum length of the string, if the length of the string is less
///   than `M`, fill with '0'.  Default is 0, or no minimum.  For signed number
///   `M` will be ignored.
///
/// - `T`: the underlying type of the number. Default is `usize`.
///
/// # Panic
///
/// - Invalid base (== 0 or > 16)
pub struct NumStr<
    const B: u8 = 10,
    const U: bool = false,
    const R: usize = 0,
    const M: usize = 0,
    T = usize,
>(T);

impl<const B: u8, const U: bool, const R: usize, const M: usize, T> AsRef<T>
    for NumStr<B, U, R, M, T>
{
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<const B: u8, const U: bool, const R: usize, const M: usize, T> ops::Deref
    for NumStr<B, U, R, M, T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const B: u8, const U: bool, const R: usize, const M: usize, T> ops::DerefMut
    for NumStr<B, U, R, M, T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> NumStr<10, false, 0, 0, T> {
    #[inline(always)]
    /// # Create a new [`NumStr`] with the given number.
    ///
    /// With default settings of `B`, `U`, `R`, `M`:
    ///
    /// - `B`: `10`
    /// - `U`: `false`
    /// - `R`: `0`
    /// - `M`: `0`
    ///
    /// See [`NumStr`] for details.
    ///
    /// If you want a hexadecimal number, chains with [`NumStr::hexadecimal()`].
    ///
    /// ## Notice
    ///
    /// For negative number, `R`, `M` will not make sense
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use macro_toolset::string::NumStr;
    /// # let num =
    /// NumStr::new_default(123_i16)
    /// # ;
    /// ```
    pub fn new_default(inner: T) -> Self {
        NumStr(inner)
    }

    #[inline(always)]
    /// # Create a new [`NumStr`] with the given number.
    ///
    /// With default settings of `B`, `U`, `R`, `M`:
    ///
    /// - `B`: `16`
    /// - `U`: `false`
    /// - `R`: `0`
    /// - `M`: `0`
    ///
    /// See [`NumStr`] for details.
    ///
    /// ## Notice
    ///
    /// For negative number, `R`, `M` will not make sense
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use macro_toolset::string::NumStr;
    /// # let num =
    /// NumStr::hex_default(123_i16)
    /// # ;
    /// ```
    pub fn hex_default(inner: T) -> NumStr<16, false, 0, 0, T> {
        NumStr(inner)
    }
}

impl NumStr<10, false, 0, 0, u8> {
    #[inline(always)]
    /// # Create a new [`NumStr`] with the given number, mostly for encoding bytes to hex.
    ///
    /// With default settings of `B`, `U`, `R`, `M`:
    ///
    /// - `B`: `16`
    /// - `U`: `false`
    /// - `R`: `2`
    /// - `M`: `0`
    ///
    /// See [`NumStr`] for details.
    ///
    /// ## Notice
    ///
    /// For negative number, `R`, `M` will not make sense
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use macro_toolset::string::{NumStr, StringExtT};
    /// let nums = vec![0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x00]
    ///     .into_iter()
    ///     .map(NumStr::hex_byte_default);
    ///
    /// assert_eq!(nums.to_string_ext(), "11451419198100");
    /// ```
    pub fn hex_byte_default(inner: u8) -> NumStr<16, false, 2, 0, u8> {
        NumStr(inner)
    }
}

impl<const B: u8, const U: bool, const R: usize, const M: usize, T> NumStr<B, U, R, M, T> {
    #[inline]
    /// Create a new [`NumStr`] with the given number.
    pub fn new(inner: T) -> Self {
        NumStr(inner)
    }

    #[inline(always)]
    /// Convert to decimal representation.
    pub fn decimal(self) -> NumStr<10, U, R, M, T> {
        NumStr(self.0)
    }

    #[inline(always)]
    /// Convert to hexadecimal representation.
    pub fn hexadecimal(self) -> NumStr<16, U, R, M, T> {
        NumStr(self.0)
    }

    #[inline]
    /// Set custom base.
    ///
    /// The valid range is `2..=16`
    pub fn set_custom_base<const NB: u8>(self) -> NumStr<NB, U, R, M, T> {
        debug_assert!(NB >= 2 && NB <= 16);

        NumStr(self.0)
    }

    #[inline]
    /// Set uppercase / lowercase of the number.
    ///
    /// Default is lowercase
    ///
    /// Note: only works for base > 10
    pub fn set_uppercase<const NU: bool>(self) -> NumStr<B, NU, R, M, T> {
        NumStr(self.0)
    }

    #[inline]
    /// Set whether to resize the string to `len` length.
    ///
    /// The overflow part will be truncated, and the insufficient part will be
    /// filled with '0'
    ///
    /// Default is not resize
    ///
    /// Note: see [`Vec::resize`] for details
    pub fn set_resize_len<const NR: usize>(self) -> NumStr<B, U, NR, M, T> {
        NumStr(self.0)
    }

    #[inline]
    /// Set the minimum length of the string.
    ///
    /// The insufficient part will be filled with '0'.
    ///
    /// Default is not minimum
    ///
    /// Note: if set `Self::should_resize`, the minimum length will be ignored
    pub fn set_minimum_len<const NM: usize>(self) -> NumStr<B, U, R, NM, T> {
        NumStr(self.0)
    }

    #[inline(always)]
    fn charset() -> &'static [u8] {
        debug_assert!(B >= 2 && B <= 16, "unsupported base: {}", B);

        if U {
            &HEX_CHARS_UPPER
        } else {
            &HEX_CHARS_LOWER
        }
    }
}

impl<const B: u8, const U: bool, const R: usize, const M: usize> NumStr<B, U, R, M, f32> {
    #[inline]
    /// Set integer only mode.
    ///
    /// Default disable.
    pub fn set_integer_only<const NU: bool>(self) -> NumStr<B, NU, R, M, f32> {
        NumStr(self.0)
    }
}

impl<const B: u8, const U: bool, const R: usize, const M: usize> NumStr<B, U, R, M, f64> {
    #[inline]
    /// Set integer only mode.
    ///
    /// Default disable.
    pub fn set_integer_only<const NU: bool>(self) -> NumStr<B, NU, R, M, f64> {
        NumStr(self.0)
    }
}

macro_rules! impl_num_str {
    (UNSIGNED: $($ty:ty) +) => {
        $(
            impl<const B: u8, const U: bool, const R: usize, const M: usize> NumStr<B, U, R, M, $ty> {
                #[inline]
                /// Encode the number to the str
                pub fn encode(self, string: &mut Vec<u8>) {
                    let current_ptr = string.len();

                    if R > 0 {
                        string.resize(current_ptr + R, b'0');

                        let (mut num, charset) = if self.0 == 0 {
                            return
                        } else {
                            (self.0, Self::charset())
                        };

                        let string = &mut string[current_ptr..current_ptr + R];

                        let mut count = 0;

                        while let Some(s) = string.get_mut(count) {
                            *s = charset[(num % B as $ty) as usize];
                            num /= B as $ty;
                            count += 1;

                            if num <= 0 {
                                break
                            }
                        }

                        string
                    } else {
                        let (mut num, charset) = if self.0 == 0 {
                            string.push(b'0');
                            return
                        } else {
                            (self.0, Self::charset())
                        };

                        let mut count = 0;

                        while num > 0 {
                            count += 1;
                            string.push(charset[(num % B as $ty) as usize]);
                            num /= B as $ty;
                        }

                        // Minimal length
                        while count < M {
                            count += 1;
                            string.push(b'0');
                        }

                        let final_ptr = string.len();
                        &mut string[current_ptr..final_ptr]
                    }.reverse();
                }
            }

            impl<const B: u8, const U: bool, const R: usize, const M: usize> StringExtT
                for NumStr<B, U, R, M, $ty>
            {
                #[inline]
                fn push_to_string(self, string: &mut Vec<u8>) {
                    self.encode(string)
                }
            }

            impl StringExtT for $ty {
                #[inline]
                fn push_to_string(self, string: &mut Vec<u8>) {
                    NumStr::new_default(self).push_to_string(string)
                }
            }
        )+
    };
    (SIGNED: $($ty:ty as $uty:ty);+) => {
        $(
            impl<const B: u8, const U: bool, const R: usize, const M: usize> NumStr<B, U, R, M, $ty> {
                #[inline]
                /// Encode the number to the str
                pub fn encode(self, string: &mut Vec<u8>) {
                    if self.is_negative() {
                        string.push(b'-');
                        // No resize or minimum length for signed numbers!
                    }

                    NumStr::<B, U, 0, 0, _>::new(self.0.unsigned_abs()).encode(string);
                }
            }

            impl<const B: u8, const U: bool, const R: usize, const M: usize> StringExtT
                for NumStr<B, U, R, M, $ty>
            {
                #[inline]
                fn push_to_string(self, string: &mut Vec<u8>) {
                    self.encode(string)
                }
            }

            impl StringExtT for $ty {
                #[inline]
                fn push_to_string(self, string: &mut Vec<u8>) {
                    NumStr::new_default(self).push_to_string(string)
                }
            }
        )+
    };
    (FLOAT: $($ty:ty) +) => {
        $(
            impl<const B: u8, const U: bool, const R: usize, const M: usize> StringExtT
                for NumStr<B, U, R, M, $ty>
            {
                #[inline]
                fn push_to_string(mut self, string: &mut Vec<u8>) {
                    if U {
                        self.0 = self.0.trunc();
                    }

                    let original_len = string.len();

                    #[cfg(not(feature = "feat-string-ext-ryu"))]
                    string.extend(format!("{}", self.0).as_bytes());

                    #[cfg(feature = "feat-string-ext-ryu")]
                    string.extend(ryu::Buffer::new().format(self.0).as_bytes());

                    #[allow(unsafe_code, reason = "must be valid utf8")]
                    match unsafe { str::from_utf8_unchecked(string) }.rfind('.') {
                        Some(dot_pos) if self.0.is_finite() => {
                            if U {
                                string.truncate(dot_pos);
                            } else if R > 0 {
                                string.resize(dot_pos + R + 1, b'0');
                            } else if dot_pos - original_len < M {
                                string.resize(dot_pos + M + 1, b'0');
                            } else {
                                // do nothing
                            }
                        },
                        Some(_) => {
                            // is NOT finite, do nothing
                        },
                        None if (U || !self.0.is_finite()) => {
                            // is not finite, or integer only, do nothing
                        },
                        None => {
                            string.push(b'.');
                            if R > 0 {
                                string.resize(original_len + R + 1, b'0');
                            } else if M > 0{
                                string.resize(original_len + M + 1, b'0');
                            } else {
                                string.push(b'0');
                            }
                        }
                    }
                }
            }

            impl StringExtT for $ty {
                #[inline]
                fn push_to_string(self, string: &mut Vec<u8>) {
                    NumStr::new_default(self).push_to_string(string)
                }
            }
        )*
    }
}

impl_num_str!(UNSIGNED: u8 u16 u32 u64 u128 usize);
impl_num_str!(SIGNED: i8 as u8; i16 as u16; i32 as u32; i64 as u64; i128 as u128; isize as usize);
impl_num_str!(FLOAT: f32 f64);

#[cfg(test)]
#[allow(clippy::cognitive_complexity)]
mod test {
    use crate::string::{NumStr, StringExtT};

    #[test]
    fn test_num_basic() {
        // unsigned number
        assert_eq!("0", (0_u8).to_string_ext());
        assert_eq!("1", (1_u8).to_string_ext());
        assert_eq!("123", (123_u8).to_string_ext());
        assert_eq!(u8::MAX.to_string(), (u8::MAX).to_string_ext());

        assert_eq!("0", (0_u16).to_string_ext());
        assert_eq!("1", (1_u16).to_string_ext());
        assert_eq!("123", (123_u16).to_string_ext());
        assert_eq!(u16::MAX.to_string(), (u16::MAX).to_string_ext());

        assert_eq!("0", (0_u32).to_string_ext());
        assert_eq!("1", (1_u32).to_string_ext());
        assert_eq!("123", (123_u32).to_string_ext());
        assert_eq!(u32::MAX.to_string(), (u32::MAX).to_string_ext());

        assert_eq!("0", (0_u64).to_string_ext());
        assert_eq!("1", (1_u64).to_string_ext());
        assert_eq!("123", (123_u64).to_string_ext());
        assert_eq!(u64::MAX.to_string(), (u64::MAX).to_string_ext());

        assert_eq!("0", (0_u128).to_string_ext());
        assert_eq!("1", (1_u128).to_string_ext());
        assert_eq!("123", (123_u128).to_string_ext());
        assert_eq!(u128::MAX.to_string(), (u128::MAX).to_string_ext());

        assert_eq!("0", (0_usize).to_string_ext());
        assert_eq!("1", (1_usize).to_string_ext());
        assert_eq!("123", (123_usize).to_string_ext());
        assert_eq!(usize::MAX.to_string(), (usize::MAX).to_string_ext());

        // signed number
        assert_eq!("-123", (-123_i8).to_string_ext());
        assert_eq!("-1", (-1_i8).to_string_ext());
        assert_eq!("0", (0_i8).to_string_ext());
        assert_eq!("1", (1_i8).to_string_ext());
        assert_eq!("123", (123_i8).to_string_ext());
        assert_eq!(i8::MAX.to_string(), (i8::MAX).to_string_ext());
        assert_eq!(i8::MIN.to_string(), (i8::MIN).to_string_ext());

        assert_eq!("-123", (-123_i16).to_string_ext());
        assert_eq!("-1", (-1_i16).to_string_ext());
        assert_eq!("0", (0_i16).to_string_ext());
        assert_eq!("1", (1_i16).to_string_ext());
        assert_eq!("123", (123_i16).to_string_ext());
        assert_eq!(i16::MAX.to_string(), (i16::MAX).to_string_ext());
        assert_eq!(i16::MIN.to_string(), (i16::MIN).to_string_ext());

        assert_eq!("-123", (-123_i32).to_string_ext());
        assert_eq!("-1", (-1_i32).to_string_ext());
        assert_eq!("0", (0_i32).to_string_ext());
        assert_eq!("1", (1_i32).to_string_ext());
        assert_eq!("123", (123_i32).to_string_ext());
        assert_eq!(i32::MAX.to_string(), (i32::MAX).to_string_ext());
        assert_eq!(i32::MIN.to_string(), (i32::MIN).to_string_ext());

        assert_eq!("-123", (-123_i64).to_string_ext());
        assert_eq!("-1", (-1_i64).to_string_ext());
        assert_eq!("0", (0_i64).to_string_ext());
        assert_eq!("1", (1_i64).to_string_ext());
        assert_eq!("123", (123_i64).to_string_ext());
        assert_eq!(i64::MAX.to_string(), (i64::MAX).to_string_ext());
        assert_eq!(i64::MIN.to_string(), (i64::MIN).to_string_ext());

        assert_eq!("-123", (-123_i128).to_string_ext());
        assert_eq!("-1", (-1_i128).to_string_ext());
        assert_eq!("0", (0_i128).to_string_ext());
        assert_eq!("1", (1_i128).to_string_ext());
        assert_eq!("123", (123_i128).to_string_ext());
        assert_eq!(i128::MAX.to_string(), (i128::MAX).to_string_ext());
        assert_eq!(i128::MIN.to_string(), (i128::MIN).to_string_ext());

        assert_eq!("-123", (-123_isize).to_string_ext());
        assert_eq!("-1", (-1_isize).to_string_ext());
        assert_eq!("0", (0_isize).to_string_ext());
        assert_eq!("1", (1_isize).to_string_ext());
        assert_eq!("123", (123_isize).to_string_ext());
        assert_eq!(isize::MAX.to_string(), (isize::MAX).to_string_ext());
        assert_eq!(isize::MIN.to_string(), (isize::MIN).to_string_ext());

        assert_eq!("-inf", f32::NEG_INFINITY.to_string_ext());
        assert_eq!("-inf", f64::NEG_INFINITY.to_string_ext());
        assert_eq!("-1.0", (-1.0_f32).to_string_ext());
        assert_eq!("-1.0", (-1.0_f64).to_string_ext());
        #[cfg(feature = "feat-string-ext-ryu")]
        assert_eq!(
            "-1.23e-40",
            (-0.000000000000000000000000000000000000000123_f32).to_string_ext()
        );
        #[cfg(not(feature = "feat-string-ext-ryu"))]
        assert_eq!(
            "-0.000000000000000000000000000000000000000123",
            (-0.000000000000000000000000000000000000000123_f32).to_string_ext()
        );
        #[cfg(feature = "feat-string-ext-ryu")]
        assert_eq!(
            "-1.23e-40",
            (-0.000000000000000000000000000000000000000123_f64).to_string_ext()
        );
        #[cfg(not(feature = "feat-string-ext-ryu"))]
        assert_eq!(
            "-0.000000000000000000000000000000000000000123",
            (-0.000000000000000000000000000000000000000123_f64).to_string_ext()
        );
        assert_eq!("-4.242", (-4.242_f32).to_string_ext());
        assert_eq!("-4.242", (-4.242_f64).to_string_ext());
        assert_eq!("0.0", (0.0_f32).to_string_ext());
        assert_eq!("0.0", (0.0_f64).to_string_ext());
        assert_eq!("1.0", (1.0_f32).to_string_ext());
        assert_eq!("1.0", (1.0_f64).to_string_ext());
        assert_eq!("4.242", (4.242_f32).to_string_ext());
        assert_eq!("4.242", (4.242_f64).to_string_ext());
        assert_eq!("inf", f32::INFINITY.to_string_ext());
        assert_eq!("inf", f64::INFINITY.to_string_ext());
    }

    #[test]
    fn test_num_hex() {
        // unsigned number
        assert_eq!(
            "0",
            NumStr::new_default(0x0_u8).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "1",
            NumStr::new_default(0x1_u8).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "42",
            NumStr::new_default(0x42_u8).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "ff",
            NumStr::new_default(u8::MAX).hexadecimal().to_string_ext()
        );

        assert_eq!(
            "0",
            NumStr::new_default(0x0_u16).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "1",
            NumStr::new_default(0x1_u16).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "123",
            NumStr::new_default(0x123_u16).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "ffff",
            NumStr::new_default(u16::MAX).hexadecimal().to_string_ext()
        );

        assert_eq!(
            "0",
            NumStr::new_default(0x0_u32).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "1",
            NumStr::new_default(0x1_u32).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "123",
            NumStr::new_default(0x123_u32).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "ffffffff",
            NumStr::new_default(u32::MAX).hexadecimal().to_string_ext()
        );

        assert_eq!(
            "0",
            NumStr::new_default(0x0_u64).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "1",
            NumStr::new_default(0x1_u64).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "123",
            NumStr::new_default(0x123_u64).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "ffffffffffffffff",
            NumStr::new_default(u64::MAX).hexadecimal().to_string_ext()
        );

        assert_eq!(
            "0",
            NumStr::new_default(0x0_u128).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "1",
            NumStr::new_default(0x1_u128).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "123",
            NumStr::new_default(0x123_u128)
                .hexadecimal()
                .to_string_ext()
        );
        assert_eq!(
            "ffffffffffffffffffffffffffffffff",
            NumStr::new_default(u128::MAX).hexadecimal().to_string_ext()
        );

        assert_eq!(
            "0",
            NumStr::new_default(0x0_usize).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "1",
            NumStr::new_default(0x1_usize).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "123",
            NumStr::new_default(0x123_usize)
                .hexadecimal()
                .to_string_ext()
        );
        assert_eq!(
            format!("{:x}", usize::MAX),
            NumStr::new_default(usize::MAX)
                .hexadecimal()
                .to_string_ext()
        );

        // signed number
        assert_eq!(
            "-42",
            NumStr::new_default(-0x42_i8).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "-1",
            NumStr::new_default(-0x1_i8).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "0",
            NumStr::new_default(0x0_i8).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "1",
            NumStr::new_default(0x1_i8).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "42",
            NumStr::new_default(0x42_i8).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "7f",
            NumStr::new_default(i8::MAX).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "-80",
            NumStr::new_default(i8::MIN).hexadecimal().to_string_ext()
        );

        assert_eq!(
            "-123",
            NumStr::new_default(-0x123_i16)
                .hexadecimal()
                .to_string_ext()
        );
        assert_eq!(
            "-1",
            NumStr::new_default(-0x1_i16).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "0",
            NumStr::new_default(0x0_i16).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "1",
            NumStr::new_default(0x1_i16).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "123",
            NumStr::new_default(0x123_i16).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "7fff",
            NumStr::new_default(i16::MAX).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "-8000",
            NumStr::new_default(i16::MIN).hexadecimal().to_string_ext()
        );

        assert_eq!(
            "-123",
            NumStr::new_default(-0x123_i32)
                .hexadecimal()
                .to_string_ext()
        );
        assert_eq!(
            "-1",
            NumStr::new_default(-0x1_i32).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "0",
            NumStr::new_default(0x0_i32).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "1",
            NumStr::new_default(0x1_i32).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "123",
            NumStr::new_default(0x123_i32).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "7fffffff",
            NumStr::new_default(i32::MAX).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "-80000000",
            NumStr::new_default(i32::MIN).hexadecimal().to_string_ext()
        );

        assert_eq!(
            "-123",
            NumStr::new_default(-0x123_i64)
                .hexadecimal()
                .to_string_ext()
        );
        assert_eq!(
            "-1",
            NumStr::new_default(-0x1_i64).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "0",
            NumStr::new_default(0x0_i64).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "1",
            NumStr::new_default(0x1_i64).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "123",
            NumStr::new_default(0x123_i64).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "7fffffffffffffff",
            NumStr::new_default(i64::MAX).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "-8000000000000000",
            NumStr::new_default(i64::MIN).hexadecimal().to_string_ext()
        );

        assert_eq!(
            "-123",
            NumStr::new_default(-0x123_i128)
                .hexadecimal()
                .to_string_ext()
        );
        assert_eq!(
            "-1",
            NumStr::new_default(-0x1_i128).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "0",
            NumStr::new_default(0x0_i128).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "1",
            NumStr::new_default(0x1_i128).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "123",
            NumStr::new_default(0x123_i128)
                .hexadecimal()
                .to_string_ext()
        );
        assert_eq!(
            "7fffffffffffffffffffffffffffffff",
            NumStr::new_default(i128::MAX).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "-80000000000000000000000000000000",
            NumStr::new_default(i128::MIN).hexadecimal().to_string_ext()
        );

        assert_eq!(
            "-123",
            NumStr::new_default(-0x123_isize)
                .hexadecimal()
                .to_string_ext()
        );
        assert_eq!(
            "-1",
            NumStr::new_default(-0x1_isize)
                .hexadecimal()
                .to_string_ext()
        );
        assert_eq!(
            "0",
            NumStr::new_default(0x0_isize).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "1",
            NumStr::new_default(0x1_isize).hexadecimal().to_string_ext()
        );
        assert_eq!(
            "123",
            NumStr::new_default(0x123_isize)
                .hexadecimal()
                .to_string_ext()
        );
        assert_eq!(
            format!("{:x}", isize::MAX),
            NumStr::new_default(isize::MAX)
                .hexadecimal()
                .to_string_ext()
        );
        assert_eq!(
            format!("-{:x}", isize::MIN),
            NumStr::new_default(isize::MIN)
                .hexadecimal()
                .to_string_ext()
        );
    }

    #[test]
    fn test_r_m() {
        let data = NumStr::new_default(123_456_789_usize);

        assert_eq!(data.set_resize_len::<12>().to_string_ext(), "000123456789");
        assert_eq!(data.set_minimum_len::<12>().to_string_ext(), "000123456789");
        assert_eq!(data.set_resize_len::<9>().to_string_ext(), "123456789");
        assert_eq!(data.set_minimum_len::<9>().to_string_ext(), "123456789");
        assert_eq!(data.set_resize_len::<6>().to_string_ext(), "456789");
        assert_eq!(data.set_minimum_len::<6>().to_string_ext(), "123456789");

        let data = NumStr::new_default(0x123_456_789_usize).hexadecimal();
        assert_eq!(data.set_resize_len::<12>().to_string_ext(), "000123456789");
        assert_eq!(data.set_minimum_len::<12>().to_string_ext(), "000123456789");
        assert_eq!(data.set_resize_len::<9>().to_string_ext(), "123456789");
        assert_eq!(data.set_minimum_len::<9>().to_string_ext(), "123456789");
        assert_eq!(data.set_resize_len::<6>().to_string_ext(), "456789");
        assert_eq!(data.set_minimum_len::<6>().to_string_ext(), "123456789");

        let data = NumStr::new_default(123456789.87654321_f64);
        assert_eq!(
            data.set_resize_len::<12>().to_string_ext(),
            "123456789.876543210000"
        );
        assert_eq!(
            data.set_minimum_len::<12>().to_string_ext(),
            "123456789.876543210000"
        );
        assert_eq!(
            data.set_resize_len::<8>().to_string_ext(),
            "123456789.87654321"
        );
        assert_eq!(
            data.set_minimum_len::<8>().to_string_ext(),
            "123456789.87654321"
        );
        assert_eq!(
            data.set_resize_len::<7>().to_string_ext(),
            "123456789.8765432"
        );
        assert_eq!(
            data.set_minimum_len::<7>().to_string_ext(),
            "123456789.87654321"
        );
        assert_eq!(data.set_resize_len::<1>().to_string_ext(), "123456789.8");
        assert_eq!(
            data.set_minimum_len::<1>().to_string_ext(),
            "123456789.87654321"
        );
        assert_eq!(data.set_integer_only::<true>().to_string_ext(), "123456789");
    }

    #[test]
    fn test_hex_uppercase() {
        let data = NumStr::new_default(0x1_234_567_890_abc_usize).hexadecimal();
        assert_eq!(
            data.set_uppercase::<true>().to_string_ext(),
            "1234567890ABC"
        );
        assert_eq!(
            data.set_uppercase::<false>().to_string_ext(),
            "1234567890abc"
        );
    }
}
