//! Bytes to Base64 string

use std::marker::PhantomData;

use super::StringExtT;

macro_rules! enum_padding {
    ($($name:ident) *) => {
        $(
            #[derive(Debug)]
            #[allow(non_camel_case_types)]
            #[doc = "Base64 Padding: "]
            #[doc = stringify!($name) ]
            pub struct $name;

            impl $name {
                /// Create a new Base64 string from bytes
                pub fn encode(bytes: &[u8]) -> Base64Str<'_, $name> {
                    Base64Str {
                        inner: bytes,
                        padding: PhantomData,
                    }
                }
            }

            impl StringExtT for Base64Str<'_, $name> {
                fn push_to_string(self, string: &mut Vec<u8>) {
                    let current_len = string.len();
                    let base64_len = self.inner.len() * 4 / 3 + 4;
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
        )*
    };
}

pub mod b64_padding {
    //! Base64 padding

    use super::{Base64Str, PhantomData, StringExtT};

    enum_padding!(STANDARD STANDARD_NO_PAD URL_SAFE URL_SAFE_NO_PAD);
}

#[derive(Debug)]
/// Base64 inner string
pub struct Base64Str<'b, P = b64_padding::STANDARD> {
    inner: &'b [u8],
    padding: PhantomData<P>,
}

#[cfg(test)]
mod test {
    use crate::string::{base64::b64_padding::STANDARD, StringExtT};

    #[test]
    fn test_base64() {
        assert_eq!(
            STANDARD::encode(b"hello world").to_string_ext(),
            "aGVsbG8gd29ybGQ="
        );
    }
}
