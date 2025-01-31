//! Base64 related macros

pub use base64::*;

#[macro_export]
/// Encode given buffer into base64 string.
///
/// # Example:
/// ```
/// use macro_toolset::{b64_encode, base64::engine::general_purpose};
///
/// // Default padding: STANDARD, returns `String`.
/// # let example =
/// b64_encode!(b"hello world");
/// # assert_eq!(example, "aGVsbG8gd29ybGQ=");
/// // To given `String`.
/// let mut string = String::new();
/// b64_encode!(b"hello world" => &mut string);
/// # assert_eq!(string, "aGVsbG8gd29ybGQ=");
///
/// // Returns `String`.
/// // Available padding: STANDARD / STANDARD_NO_PAD / URL_SAFE / URL_SAFE_NO_PAD
/// // No need to import!
/// # let example =
/// b64_encode!(URL_SAFE_NO_PAD: b"hello world");
/// # assert_eq!(example, "aGVsbG8gd29ybGQ");
/// // To given `String`.
/// let mut string = String::new();
/// b64_encode!(URL_SAFE_NO_PAD: b"hello world" => STRING: &mut string);
/// # assert_eq!(string, "aGVsbG8gd29ybGQ");
///
/// // Returns `bytes::BytesMut`.
/// # let example =
/// b64_encode!(URL_SAFE_NO_PAD: b"hello world" => BYTES);
/// # assert_eq!(&example[..], b"aGVsbG8gd29ybGQ");
/// // To given `bytes::BytesMut`.
/// let mut string = bytes::BytesMut::new();
/// b64_encode!(URL_SAFE_NO_PAD: b"hello world" => BYTES: &mut string);
/// # assert_eq!(&string[..], b"aGVsbG8gd29ybGQ");
/// ```
macro_rules! b64_encode {
    ($data:expr) => {
        $crate::b64_encode!(STANDARD: $data)
    };
    ($data:expr => $string:expr) => {
        $crate::b64_encode!(STANDARD: $data => STRING: $string)
    };
    (STANDARD: $($tt:tt)+) => {
        $crate::b64_encode!($crate::base64::engine::general_purpose::STANDARD, $($tt)+)
    };
    (STANDARD_NO_PAD: $($tt:tt)+) => {
        $crate::b64_encode!($crate::base64::engine::general_purpose::STANDARD_NO_PAD, $($tt)+)
    };
    (URL_SAFE: $($tt:tt)+) => {
        $crate::b64_encode!($crate::base64::engine::general_purpose::URL_SAFE, $($tt)+)
    };
    (URL_SAFE_NO_PAD: $($tt:tt)+) => {
        $crate::b64_encode!($crate::base64::engine::general_purpose::URL_SAFE_NO_PAD, $($tt)+)
    };
    ($padding:path, $data:expr) => {{
        let mut string = String::with_capacity($data.len() * 4 / 3 + 4 + 64);

        $crate::base64::Engine::encode_string(&$padding, $data, &mut string);

        string
    }};
    ($padding:path, $data:expr => STRING: $string:expr) => {
        $crate::base64::Engine::encode_string(&$padding, $data, $string)
    };
    ($padding:path, $data:expr => BYTES) => {{
        let target_len = $data.len() * 4 / 3 + 4 + 64;

        // Allocate buffer
        let mut bytes_buf = bytes::BytesMut::with_capacity(target_len);
        // `target_len` is the exact length of the base64 string.
        #[allow(unsafe_code)]
        unsafe {
            bytes_buf.set_len(target_len)
        };

        let bytes_written =
            $crate::base64::Engine::encode_slice(&$padding, $data, bytes_buf.as_mut()).unwrap_or(0);

        bytes_buf.truncate(bytes_written);
        bytes_buf
    }};
    ($padding:path, $data:expr => BYTES: $bytes_buf:expr) => {{
        let target_len = $data.len() * 4 / 3 + 4 + 64;

        // Allocate buffer
        $bytes_buf.reserve(target_len);
        // `target_len` is the exact length of the base64 string.
        #[allow(unsafe_code)]
        unsafe {
            $bytes_buf.set_len(target_len)
        };

        let bytes_written =
            $crate::base64::Engine::encode_slice(&$padding, $data, $bytes_buf).unwrap_or(0);

        $bytes_buf.truncate(bytes_written);
    }};
}

#[macro_export]
/// Decode given base64 string to bytes.
///
/// # Example:
/// ```
/// use macro_toolset::{b64_decode, base64::engine::general_purpose};
/// // Default padding: STANDARD, returns `Vec<u8>`.
/// # let example =
/// b64_decode!("aGVsbG8gd29ybGQ=")  
/// # .unwrap();
/// # assert_eq!(example, b"hello world");
/// // Available padding: STANDARD / STANDARD_NO_PAD / URL_SAFE / URL_SAFE_NO_PAD.
/// // No need to import!
/// # let example =
/// b64_decode!(URL_SAFE_NO_PAD: "aGVsbG8gd29ybGQ")
/// # .unwrap();
/// # assert_eq!(example, b"hello world");
/// // To given `Vec<u8>`
/// # let mut example = Vec::new();
/// b64_decode!(URL_SAFE_NO_PAD: "aGVsbG8gd29ybGQ", &mut example);
/// # assert_eq!(&example, b"hello world");
/// // Though not recommended, we support the full path as well.
/// # let example =
/// b64_decode!(general_purpose::URL_SAFE_NO_PAD, "aGVsbG8gd29ybGQ")
/// # .unwrap();
/// # assert_eq!(example, b"hello world");
/// ```
macro_rules! b64_decode {
    ($data:expr) => {
        $crate::b64_decode!(STANDARD: $data)
    };
    (STANDARD: $($tt:tt)+) => {
        $crate::b64_decode!($crate::base64::engine::general_purpose::STANDARD, $($tt)+)
    };
    (STANDARD_NO_PAD: $($tt:tt)+) => {
        $crate::b64_decode!($crate::base64::engine::general_purpose::STANDARD_NO_PAD, $($tt)+)
    };
    (URL_SAFE: $($tt:tt)+) => {
        $crate::b64_decode!($crate::base64::engine::general_purpose::URL_SAFE, $($tt)+)
    };
    (URL_SAFE_NO_PAD: $($tt:tt)+) => {
        $crate::b64_decode!($crate::base64::engine::general_purpose::URL_SAFE_NO_PAD, $($tt)+)
    };
    ($padding:path, $data:expr) => {
        $crate::base64::Engine::decode(&$padding, $data)
    };
    ($padding:path, $data:expr, $buf:expr) => {
        $crate::base64::Engine::decode_vec(&$padding, $data, $buf)
    };
}

// ============ Deprecated ================

#[deprecated(since = "0.8.0-rc.5", note = "use `b64_encode!` instead")]
#[macro_export]
/// Base64 encode with [`bytes::Bytes`] returned
macro_rules! b64_encode_bytes {
    ($data:expr) => {
        $crate::b64_encode_bytes!($data, $crate::base64::engine::general_purpose::STANDARD)
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
            $crate::base64::Engine::encode_slice(&$padding, $data, bytes_buf.as_mut()).unwrap_or(0);
        bytes_buf.truncate(bytes_written);
        bytes_buf.freeze()
    }};
}
