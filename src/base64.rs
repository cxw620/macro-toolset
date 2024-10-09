//! Base64 related macros

#[macro_export]
/// Base64 encode
///
/// Param:
///  + `data`: impl [`AsRef`]<[u8]>
///  + `padding`: `base64::engine::general_purpose::{}`, `STANDARD`(default) / `STANDARD_NO_PAD` / `URL_SAFE` / `URL_SAFE_NO_PAD`
macro_rules! b64_encode {
    ($data:expr) => {
        $crate::b64_encode!($data, base64::engine::general_purpose::STANDARD)
    };
    ($data:expr, $padding:path) => {{
        let mut string_buf = String::with_capacity(256);
        base64::Engine::encode_string(&$padding, $data, &mut string_buf);
        string_buf
    }};
}

#[macro_export]
/// Base64 encode with [`bytes::Bytes`] returned
///
/// Param:
///  + `data`: impl [`AsRef`]<[u8]>
///  + `padding`: `base64::engine::general_purpose::{}`, `STANDARD`(default) / `STANDARD_NO_PAD` / `URL_SAFE` / `URL_SAFE_NO_PAD`
macro_rules! b64_encode_bytes {
    ($data:expr) => {
        b64_encode_bytes!($data, base64::engine::general_purpose::STANDARD)
    };
    ($data:expr, $padding:path) => {{
        let data = $data.as_ref();
        // make sure we'll have a slice big enough for base64 + padding
        let mut bytes_buf = bytes::BytesMut::with_capacity(data.len() * 4 / 3 + 256);
        let _ = base64::Engine::encode_slice(&$padding, $data, bytes_buf.as_mut());
        bytes_buf.freeze()
    }};
}

#[macro_export]
/// Base64 decode
/// 
/// # Param:
///  + `data`
///  + `padding`: `base64::engine::general_purpose::{}`, `STANDARD`(default) / `STANDARD_NO_PAD` / `URL_SAFE` / `URL_SAFE_NO_PAD`
macro_rules! b64_decode {
    ($data:expr) => {
        b64_decode!($data, base64::engine::general_purpose::STANDARD)
    };
    ($data:expr, $padding:path) => {
        base64::Engine::decode(&$padding, $data)
    };
}
