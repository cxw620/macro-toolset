//! Bytes to Base64 string

use std::{marker::PhantomData, ops};

use super::StringExtT;

pub mod b64_padding {
    //! Base64 padding
    //!
    //! The `base64` crate has ugly APIs and we here create some ZSTs
    //! to represent the padding, convenient to use and performance improvement.

    use super::{Base64Str, Decode, DecodeToAny, DecodeToHex, Encode, PhantomData, StringExtT};
    use crate::string::NumStr;

    macro_rules! enum_padding {
        ($($name:ident) *) => {
            $(
                #[derive(Debug)]
                #[allow(non_camel_case_types)]
                #[doc = "Base64 Padding: "]
                #[doc = stringify!($name) ]
                pub struct $name;

                impl $name {
                    #[inline]
                    /// Create a new [`Base64Str`], and finally encode it to a Base64 string.
                    pub fn encode<T: AsRef<[u8]>>(inner: T) -> Base64Str<T, $name, Encode> {
                        Base64Str {
                            inner,
                            padding: PhantomData,
                            command: PhantomData,
                        }
                    }

                    #[inline]
                    /// Create a new [`Base64Str`], and finally decode the inner Base64 string.
                    ///
                    /// Notice: will do nothing if the decoded string is not valid UTF-8 encoded.
                    /// If that is acceptable, use [`decode_to_any`](Self::decode_to_any).
                    pub fn decode<T: AsRef<[u8]>>(inner: T) -> Base64Str<T, $name, Decode> {
                        Base64Str {
                            inner,
                            padding: PhantomData,
                            command: PhantomData,
                        }
                    }

                    #[allow(unsafe_code, reason = "Calling this means the decoded string can be invalid UTF-8")]
                    #[inline]
                    /// Create a new [`Base64Str`], and finally decode the inner Base64 string.
                    ///
                    /// # Safety
                    ///
                    /// Calling this means the decoded string can be invalid UTF-8.
                    pub unsafe fn decode_to_any<T: AsRef<[u8]>>(inner: T) -> Base64Str<T, $name, DecodeToAny> {
                        Base64Str {
                            inner,
                            padding: PhantomData,
                            command: PhantomData,
                        }
                    }

                    #[inline]
                    /// Create a new [`Base64Str`], and finally decode the inner Base64 string.
                    ///
                    /// Notice: will do nothing if the inner string is not a valid Base64 string.
                    pub fn decode_to_hex<T: AsRef<[u8]>>(inner: T) -> Base64Str<T, $name, DecodeToHex> {
                        Base64Str {
                            inner,
                            padding: PhantomData,
                            command: PhantomData,
                        }
                    }
                }

                impl<T: AsRef<[u8]>> StringExtT for Base64Str<T, $name, Encode> {
                    fn push_to_string(self, string: &mut Vec<u8>) {
                        let inner = self.inner.as_ref();

                        let current_len = string.len();
                        let base64_len = inner.len() * 4 / 3 + 4;
                        let target_len = current_len + base64_len;

                        string.reserve_exact(base64_len);
                        #[allow(unsafe_code)]
                        // Safety: have reserved and allocate enough space
                        unsafe {
                            string.set_len(target_len);
                        }

                        let bytes_written = base64::Engine::encode_slice(
                            &base64::engine::general_purpose::$name,
                            self.inner,
                            &mut string[current_len..target_len],
                        )
                        .unwrap_or(0);

                        string.truncate(current_len + bytes_written);
                    }
                }

                impl<T: AsRef<[u8]>> StringExtT for Base64Str<T, $name, Decode> {
                    fn push_to_string(self, string: &mut Vec<u8>) {
                        use base64::Engine;

                        let data = base64::engine::general_purpose::$name
                            .decode(self.inner.as_ref())
                            .unwrap_or_default();

                        if std::str::from_utf8(&data).is_ok() {
                            string.extend(data)
                        }
                    }
                }

                impl<T: AsRef<[u8]>> StringExtT for Base64Str<T, $name, DecodeToAny> {
                    fn push_to_string(self, string: &mut Vec<u8>) {
                        use base64::Engine;

                        let _ = base64::engine::general_purpose::$name
                            .decode_vec(self.inner.as_ref(), string);
                    }
                }

                impl<T: AsRef<[u8]>> StringExtT for Base64Str<T, $name, DecodeToHex> {
                    fn push_to_string(self, string: &mut Vec<u8>) {
                        use base64::Engine;

                        base64::engine::general_purpose::$name
                            .decode(self.inner.as_ref())
                            .unwrap_or_default()
                            .into_iter()
                            .map(NumStr::hex_byte_default)
                            .push_to_string(string);
                    }
                }
            )*
        };
    }

    enum_padding!(STANDARD STANDARD_NO_PAD URL_SAFE URL_SAFE_NO_PAD);
}

#[derive(Debug)]
/// Command: Encode, ZST marker struct
pub struct Encode;

#[derive(Debug)]
/// Command: Decode, ZST marker struct
///
/// Notice: Will do nothing if the decoded string is not valid UTF-8 encoded.
pub struct Decode;

#[derive(Debug)]
/// Command: Decode, ZST marker struct
///
/// This means the decoded string can be invalid UTF-8.
pub struct DecodeToAny;

#[derive(Debug)]
/// Command: Decode, ZST marker struct
///
/// This means the decoded byte will be hex encoded, lowercase.
pub struct DecodeToHex;

#[derive(Debug)]
#[repr(transparent)]
/// Base64 string, to encode or decode.
///
/// This struct can only be created by [`b64_padding::STANDARD`], etc.
///
/// Notice: will do nothing if the inner is not base64 encoded when decoding.
pub struct Base64Str<T: AsRef<[u8]>, P = b64_padding::STANDARD, C = Encode> {
    inner: T,
    padding: PhantomData<P>,
    command: PhantomData<C>,
}

impl<T: AsRef<[u8]>, P, C> ops::Deref for Base64Str<T, P, C> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: AsRef<[u8]>, P, C> AsRef<[u8]> for Base64Str<T, P, C> {
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

#[allow(unsafe_code, reason = "test unsafe")]
#[cfg(test)]
mod test {
    use crate::string::{base64::b64_padding, StringExtT};

    #[test]
    fn test_base64() {
        assert_eq!(
            b64_padding::STANDARD::encode(b"hello world").to_string_ext(),
            "aGVsbG8gd29ybGQ="
        );
        assert_eq!(
            b64_padding::STANDARD::encode(b"hello world").to_string_ext(),
            "aGVsbG8gd29ybGQ="
        );
        assert_eq!(
            b64_padding::STANDARD::encode("hello world").to_string_ext(),
            "aGVsbG8gd29ybGQ="
        );
        assert_eq!(
            b64_padding::STANDARD::encode("hello world").to_string_ext(),
            "aGVsbG8gd29ybGQ="
        );
        assert_eq!(
            b64_padding::STANDARD::decode(b"aGVsbG8gd29ybGQ=").to_string_ext(),
            "hello world"
        );
        assert_eq!(
            unsafe { b64_padding::STANDARD::decode_to_any(b"aGVsbG8gd29ybGQ=") }.to_string_ext(),
            "hello world"
        );
        assert_eq!(
            b64_padding::STANDARD::decode_to_hex(
                b64_padding::STANDARD::encode(vec![0x11, 0x45, 0x14, 0x19, 0x19, 0x81, 0x00])
                    .to_string_ext()
            )
            .to_string_ext(),
            "11451419198100"
        );
    }
}
