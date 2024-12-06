//! Base64 related macros

#[macro_export]
/// Base64 encode
///
/// # Param:
///  + `data`: impl [`AsRef`]<[u8]>
///  + `padding`: `base64::engine::general_purpose::{}`, `STANDARD`(default) /
///    `STANDARD_NO_PAD` / `URL_SAFE` / `URL_SAFE_NO_PAD`
///
/// # Example:
///
/// ```rust
/// # use macro_toolset::b64_encode;
/// # let example =
/// b64_encode!(b"hello world");
/// # assert_eq!(example, "aGVsbG8gd29ybGQ=");
/// ```
macro_rules! b64_encode {
    ($data:expr) => {
        $crate::b64_encode!($data, ::base64::engine::general_purpose::STANDARD)
    };
    ($data:expr, $padding:path) => {{
        let mut string_buf = String::with_capacity(256);
        ::base64::Engine::encode_string(&$padding, $data, &mut string_buf);
        string_buf
    }};
}

#[macro_export]
/// Base64 encode with [`bytes::Bytes`] returned
///
/// Param:
///  + `data`: impl [`AsRef`]<[u8]>
///  + `padding`: `base64::engine::general_purpose::{}`, `STANDARD`(default) /
///    `STANDARD_NO_PAD` / `URL_SAFE` / `URL_SAFE_NO_PAD`
///
/// # Example:
///
/// ```rust
/// # use macro_toolset::b64_encode_bytes;
/// # let example =
/// b64_encode_bytes!(b"hello world");
/// # assert_eq!(&example[..], &b"aGVsbG8gd29ybGQ="[..]);
/// ```
macro_rules! b64_encode_bytes {
    ($data:expr) => {
        b64_encode_bytes!($data, ::base64::engine::general_purpose::STANDARD)
    };
    ($data:expr, $padding:path) => {{
        let data = $data.as_ref();
        let target_len = data.len() * 4 / 3 + 4;
        let mut bytes_buf = bytes::BytesMut::with_capacity(target_len + 64);
        #[allow(unsafe_code)]
        // Safety: `target_len` is the exact length of the base64 string.
        unsafe {
            bytes_buf.set_len(target_len)
        };
        let bytes_written =
            ::base64::Engine::encode_slice(&$padding, $data, bytes_buf.as_mut()).unwrap_or(0);
        bytes_buf.truncate(bytes_written);
        bytes_buf.freeze()
    }};
}

#[macro_export]
/// Base64 decode
///
/// # Param:
///  + `data`
///  + `padding`: `base64::engine::general_purpose::{}`, `STANDARD`(default) /
///    `STANDARD_NO_PAD` / `URL_SAFE` / `URL_SAFE_NO_PAD`
///
/// # Example:
/// ```
/// # use macro_toolset::b64_decode;
/// // "hello world"
/// # let example =
/// b64_decode!("aGVsbG8gd29ybGQ=")
/// # .unwrap();
/// # assert_eq!(example, b"hello world");
/// ```
macro_rules! b64_decode {
    ($data:expr) => {
        b64_decode!($data, ::base64::engine::general_purpose::STANDARD)
    };
    ($data:expr, $padding:path) => {
        ::base64::Engine::decode(&$padding, $data)
    };
}
