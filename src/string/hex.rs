//! Number to Hex string
//!
//! Actually just with [`NumStr`](crate::string::NumStr) you can do so.
//! However for fixed length hex string, [`const_hex`] does better.

use super::StringExtT;

#[derive(Debug, Clone)]
/// Hex string with fixed length.
///
/// # Generic (TODO: Replace with bitflag?)
///
/// - N: Length of target buffer, the length of the final string is 2*N (+1 if with prefix).
/// - P: Prefix `0x`, default false
/// - U: Uppercase, default false
///
/// This implements [`StringExtT`](super::StringExtT).
///
/// For hex string with variable length, use [`NumStr`](super::NumStr).
pub enum HexStr<'s, const N: usize, const P: bool = false, const U: bool = false> {
    /// Owned
    Owned(Vec<u8>),

    /// Borrowed
    Borrowed(&'s [u8]),
}

impl<'s, const N: usize, const P: bool, const U: bool> HexStr<'s, N, P, U> {
    #[inline]
    /// Create a new hex string from give slice.
    pub const fn new(value: &'s [u8]) -> Self {
        Self::Borrowed(value)
    }

    #[inline]
    /// Create a new hex string from given owned slice.
    pub const fn new_owned(value: Vec<u8>) -> Self {
        Self::Owned(value)
    }

    #[inline]
    /// Set with prefix `0x`
    pub fn set_with_prefix<const NP: bool>(self) -> HexStr<'s, N, NP, U> {
        match self {
            Self::Borrowed(value) => HexStr::Borrowed(value),
            Self::Owned(value) => HexStr::Owned(value),
        }
    }

    #[inline]
    /// Set to uppercase
    pub fn set_uppercase<const NU: bool>(self) -> HexStr<'s, N, P, NU> {
        match self {
            Self::Borrowed(value) => HexStr::Borrowed(value),
            Self::Owned(value) => HexStr::Owned(value),
        }
    }

    #[inline]
    /// Encode to string
    fn encode(&self, string: &mut Vec<u8>) {
        let mut buffer = [0; N];

        for (idx, &i) in (0..N).rev().zip(
            match self {
                Self::Borrowed(value) => value,
                Self::Owned(value) => value.as_slice(),
            }
            .iter()
            .rev(),
        ) {
            buffer[idx] = i
        }

        if U {
            string.extend(
                const_hex::Buffer::<N, P>::new()
                    .const_format_upper(&buffer)
                    .as_bytes(),
            );
        } else {
            string.extend(
                const_hex::Buffer::<N, P>::new()
                    .const_format(&buffer)
                    .as_bytes(),
            );
        }
    }
}

impl<const N: usize, const P: bool, const U: bool> StringExtT for HexStr<'_, N, P, U> {
    fn push_to_string(self, string: &mut Vec<u8>) {
        self.encode(string);
    }
}

#[derive(Debug)]
/// Simple wrapper over [`const_hex::Buffer`] that implements [`StringExtT`](super::StringExtT).
///
/// Just create a [`const_hex::Buffer`] then [`Into::into`]!
pub struct RawBuffer<const N: usize, const P: bool = false>(const_hex::Buffer<N, P>);

impl<const N: usize, const P: bool> From<const_hex::Buffer<N, P>> for RawBuffer<N, P> {
    fn from(value: const_hex::Buffer<N, P>) -> Self {
        Self(value)
    }
}

impl<const N: usize, const P: bool> StringExtT for RawBuffer<N, P> {
    fn push_to_string(self, string: &mut Vec<u8>) {
        string.extend(self.0.as_bytes())
    }
}

#[cfg(test)]
mod test {
    use crate::string::{HexStr, StringExtT};

    #[test]
    fn test() {
        assert_eq!(
            HexStr::<0>::new(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]).to_string_ext(),
            ""
        );
        assert_eq!(
            HexStr::<7>::new(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]).to_string_ext(),
            "01020304050607"
        );
        assert_eq!(
            HexStr::<8>::new(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]).to_string_ext(),
            "0001020304050607"
        );
        assert_eq!(
            HexStr::<9>::new(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]).to_string_ext(),
            "000001020304050607"
        );
    }

    #[test]
    fn test_with_prefix() {
        assert_eq!(
            HexStr::<0>::new(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07])
                .set_with_prefix::<true>()
                .to_string_ext(),
            "0x"
        );
        assert_eq!(
            HexStr::<7>::new(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07])
                .set_with_prefix::<true>()
                .to_string_ext(),
            "0x01020304050607"
        );
        assert_eq!(
            HexStr::<8>::new(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07])
                .set_with_prefix::<true>()
                .to_string_ext(),
            "0x0001020304050607"
        );
        assert_eq!(
            HexStr::<9>::new(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07])
                .set_with_prefix::<true>()
                .to_string_ext(),
            "0x000001020304050607"
        );
    }

    #[test]
    fn test_uppcase() {
        assert_eq!(
            HexStr::<7>::new(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0xa7])
                .set_with_prefix::<true>()
                .set_uppercase::<true>()
                .to_string_ext(),
            "0x010203040506A7"
        );
        assert_eq!(
            HexStr::<8>::new(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0xa7])
                .set_with_prefix::<true>()
                .set_uppercase::<true>()
                .to_string_ext(),
            "0x00010203040506A7"
        );
        assert_eq!(
            HexStr::<9>::new(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0xa7])
                .set_with_prefix::<true>()
                .set_uppercase::<true>()
                .to_string_ext(),
            "0x0000010203040506A7"
        );
    }
}
