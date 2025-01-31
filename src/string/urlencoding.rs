//! URL Encoded string

use super::{StringExtT, StringT};
use crate::wrapper;

#[macro_export]
/// See [`Encode`] or [`Decode`] for more information.
///
/// # Example
///
/// ```
/// # use macro_toolset::{urlencoding_str, string::StringExtT};
/// let data = urlencoding_str!(E: "你好, 世界").to_string_ext();
/// assert_eq!(data, "%E4%BD%A0%E5%A5%BD%2C%20%E4%B8%96%E7%95%8C");
/// let data = urlencoding_str!(D: "%E4%BD%A0%E5%A5%BD%2C%20%E4%B8%96%E7%95%8C").to_string_ext();
/// assert_eq!(data, "你好, 世界");
/// ```
macro_rules! urlencoding_str {
    (E: $data:expr) => {
        $crate::string::urlencoding::Encode { inner: $data }
    };
    (D: $data:expr) => {
        $crate::string::urlencoding::Decode { inner: $data }
    };
}

wrapper! {
    #[derive(Debug)]
    /// `string` which is to be encoded.
    pub Encode<T>(pub T)
}

impl<T> StringT for Encode<T>
where
    T: StringT,
{
    #[inline]
    fn encode_to_buf(self, string: &mut Vec<u8>) {
        let mut buf = Vec::with_capacity(64);
        self.inner.encode_to_buf(&mut buf);

        string.reserve_exact(buf.len() | 15);
        buf.into_iter().for_each(|byte| {
            if matches!(byte, b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' |  b'-' | b'.' | b'_' | b'~')
            {
                string.push(byte);
            } else {
                string.extend([b'%', to_hex_digit(byte >> 4), to_hex_digit(byte & 15)]);
            }
        });
    }

    #[inline]
    fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
        self.encode_to_buf(string);
        string.extend(separator.as_bytes());
    }

    #[inline]
    fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
        let mut buf = Vec::with_capacity(64);
        self.inner.encode_to_buf(&mut buf);

        string.reserve(buf.len() | 15);
        buf.into_iter().for_each(|byte| {
            if matches!(byte, b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' |  b'-' | b'.' | b'_' | b'~')
            {
                string.extend(Some(byte));
            } else {
                string.extend([b'%', to_hex_digit(byte >> 4), to_hex_digit(byte & 15)]);
            }
        });
    }

    #[inline]
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
        self.encode_to_bytes_buf(string);
        string.extend(separator.as_bytes());
    }
}

impl<T> StringExtT for Encode<T> where T: StringT {}

#[inline(always)]
const fn to_hex_digit(digit: u8) -> u8 {
    match digit {
        0..=9 => b'0' + digit,
        10..=255 => b'A' - 10 + digit,
    }
}

wrapper! {
    #[derive(Debug)]
    /// `string` which is to be decoded.
    pub Decode<T>(pub T)
}

impl<T> StringT for Decode<T>
where
    T: AsRef<str>,
{
    #[inline]
    fn encode_to_buf(self, string: &mut Vec<u8>) {
        let encoded_bytes = self.inner.as_ref().as_bytes();

        let mut encoded_bytes_iter = encoded_bytes.split(|&c| c == b'%');
        if let Some(non_escaped_part) = encoded_bytes_iter.next() {
            string.extend(non_escaped_part);
        } else {
            return;
        }
        for escaped_part in encoded_bytes_iter {
            let decoded = escaped_part.get(0..=1).and_then(|escaped_part| {
                Some((
                    from_hex_digit(escaped_part[0])?,
                    from_hex_digit(escaped_part[1])?,
                ))
            });
            if let Some(decoded) = decoded {
                string.push((decoded.0 << 4) | decoded.1);
                if let Some(non_escaped_part) = escaped_part.get(2..) {
                    string.extend(non_escaped_part);
                }
            } else {
                // Error, keep it.
                string.extend(b"%");
                string.extend(escaped_part);
            }
        }
    }

    #[inline]
    fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
        self.encode_to_buf(string);
        string.extend(separator.as_bytes());
    }

    #[inline]
    fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
        let encoded_bytes = self.inner.as_ref().as_bytes();

        let mut encoded_bytes_iter = encoded_bytes.split(|&c| c == b'%');
        if let Some(non_escaped_part) = encoded_bytes_iter.next() {
            string.extend(non_escaped_part);
        } else {
            return;
        }
        for escaped_part in encoded_bytes_iter {
            let decoded = escaped_part.get(0..=1).and_then(|escaped_part| {
                Some((
                    from_hex_digit(escaped_part[0])?,
                    from_hex_digit(escaped_part[1])?,
                ))
            });
            if let Some(decoded) = decoded {
                string.extend(Some((decoded.0 << 4) | decoded.1));
                if let Some(non_escaped_part) = escaped_part.get(2..) {
                    string.extend(non_escaped_part);
                }
            } else {
                // Error, keep it.
                string.extend(b"%");
                string.extend(escaped_part);
            }
        }
    }

    #[inline]
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
        self.encode_to_bytes_buf(string);
        string.extend(separator.as_bytes());
    }
}

impl<T> StringExtT for Decode<T> where T: AsRef<str> {}

#[inline(always)]
const fn from_hex_digit(digit: u8) -> Option<u8> {
    match digit {
        b'0'..=b'9' => Some(digit - b'0'),
        b'A'..=b'F' => Some(digit - b'A' + 10),
        b'a'..=b'f' => Some(digit - b'a' + 10),
        _ => None,
    }
}
