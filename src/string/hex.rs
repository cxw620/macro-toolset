//! Number to Hex string
//!
//! Actually just with [`NumStr`](crate::string::NumStr) you can do so.
//! However for fixed length hex string, [`const_hex`] does better.

use super::{StringExtT, StringT};

#[derive(Debug, Clone)]
/// Hex string with fixed length.
///
/// # Generic
///
/// - N: Length of target buffer, the length of the final string is 2*N (+1 if
///   with prefix).
/// - P: Prefix `0x`, default false
/// - U: Uppercase, default false
///
/// For hex string with non-fixed length, use [`NumStr`](super::NumStr).
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
    fn encode<T>(&self, string: &mut T)
    where
        T: for<'a> Extend<&'a u8>,
    {
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

impl<const N: usize, const P: bool, const U: bool> StringT for HexStr<'_, N, P, U> {
    #[inline]
    fn encode_to_buf(self, string: &mut Vec<u8>) {
        self.encode(string);
    }

    #[inline]
    fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
        self.encode(string);
        string.extend(separator.as_bytes());
    }

    #[inline]
    fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
        self.encode(string);
    }

    #[inline]
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
        self.encode(string);
        string.extend(separator.as_bytes());
    }
}

impl<const N: usize, const P: bool, const U: bool> StringExtT for HexStr<'_, N, P, U> {}

impl<const N: usize, const P: bool> StringT for const_hex::Buffer<N, P> {
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
    fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
        string.extend(self.as_bytes());
    }

    #[inline]
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
        string.extend(self.as_bytes());
        string.extend(separator.as_bytes());
    }
}

impl<const N: usize, const P: bool> StringExtT for const_hex::Buffer<N, P> {}

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
