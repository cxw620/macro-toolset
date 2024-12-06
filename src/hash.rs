//! Hash related macros

// re-export const_hex
pub use const_hex;

#[macro_export]
/// Calculate MD5 hash.
///
/// Just a shortcut for `calc_hash!(MD5: ...)`.
///
/// See [`calc_hash`] for more details.
///
/// ```
/// # use macro_toolset::md5;
///
/// // General usage. Multiple params supported.
/// # assert_eq!(
/// md5!("hello", "world")
/// # .as_str(), "fc5e038d38a57032085441e7fe7010b0");
/// ```
macro_rules! md5 {
    ($($tt:tt)*) => {
        $crate::calc_hash!(MD5: $($tt)*)
    };
}

#[macro_export]
/// Calc SHA256 hash.
///
/// Just a shortcut for `calc_hash!(SHA256: ...)`.
///
/// See [`calc_hash`] for more details.
///
/// ```
/// # use macro_toolset::sha256;
///
/// // General usage. Multiple params supported.
/// # assert_eq!(
/// sha256!("hello", "world")
/// # .as_str(), "936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af");
/// ```
macro_rules! sha256 {
    ($($tt:tt)*) => {
        $crate::calc_hash!(SHA256: $($tt)*)
    };
}

#[macro_export]
/// Calculate SHA384 hash.
///
/// Just a shortcut for `calc_hash!(SHA384: ...)`.
///
/// See [`calc_hash`] for more details.
///
/// ```
/// # use macro_toolset::sha384;
///
/// // General usage. Multiple params supported.
/// # assert_eq!(
/// sha384!("hello", "world")
/// # .as_str(), "97982a5b1414b9078103a1c008c4e3526c27b41cdbcf80790560a40f2a9bf2ed4427ab1428789915ed4b3dc07c454bd9");
/// ```
macro_rules! sha384 {
    ($($tt:tt)*) => {
        $crate::calc_hash!(SHA384: $($tt)*)
    };
}

#[macro_export]
/// Calculate SHA512 hash.
///
/// Just a shortcut for `calc_hash!(SHA512: ...)`.
///
/// See [`calc_hash`] for more details.
///
/// ```
/// # use macro_toolset::sha512;
///
/// // General usage. Multiple params supported.
/// # assert_eq!(
/// sha512!("hello", "world")
/// # .as_str(), "1594244d52f2d8c12b142bb61f47bc2eaf503d6d9ca8480cae9fcf112f66e4967dc5e8fa98285e36db8af1b8ffa8b84cb15e0fbcf836c3deb803c13f37659a60");
macro_rules! sha512 {
    ($($tt:tt)*) => {
        $crate::calc_hash!(SHA512: $($tt)*)
    };
}

#[macro_export]
/// A helper macro to get string from hash result
///
/// This is equivalent to `calc_hash!(...).as_str()`.
///
/// # Examples
///
/// ```
/// # use macro_toolset::calc_hash_str;
///
/// // General usage. Multiple params supported.
/// # assert_eq!(
/// calc_hash_str!(SHA256: "hello", "world")
/// # , "936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af");
/// ```
macro_rules! calc_hash_str {
    ($($tt:tt)*) => {
        $crate::calc_hash!($($tt)*).as_str()
    };
}

#[macro_export]
/// Calculate Hash.
///
/// Returns [`::const_hex::Buffer`], you may use [`::const_hex::Buffer::as_str`]
/// to get the string.
///
/// # Examples
///
/// ```
/// # use macro_toolset::calc_hash;
///
/// // General usage. Multiple params supported.
/// # let example =
/// calc_hash!(MD5: "hello", "world");
/// # assert_eq!(example.as_str(), "fc5e038d38a57032085441e7fe7010b0");
/// # let example =
/// calc_hash!(SHA256: "hello", "world");
/// # assert_eq!(example.as_str(), "936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af");
/// # let example =
/// calc_hash!(SHA384: "hello", "world");
/// # assert_eq!(example.as_str(), "97982a5b1414b9078103a1c008c4e3526c27b41cdbcf80790560a40f2a9bf2ed4427ab1428789915ed4b3dc07c454bd9");
/// # let example =
/// calc_hash!(SHA512: "hello", "world");
/// # assert_eq!(example.as_str(), "1594244d52f2d8c12b142bb61f47bc2eaf503d6d9ca8480cae9fcf112f66e4967dc5e8fa98285e36db8af1b8ffa8b84cb15e0fbcf836c3deb803c13f37659a60");
/// // You may pass an iterator as input
/// # let example =
/// calc_hash!(MD5: ITER => ["hello", "world"]);
/// # assert_eq!(example.as_str(), "fc5e038d38a57032085441e7fe7010b0");
/// # let example =
/// calc_hash!(SHA256: ITER => ["hello", "world"]);
/// # assert_eq!(example.as_str(), "936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af");
/// # let example =
/// calc_hash!(SHA384: ITER => ["hello", "world"]);
/// # assert_eq!(example.as_str(), "97982a5b1414b9078103a1c008c4e3526c27b41cdbcf80790560a40f2a9bf2ed4427ab1428789915ed4b3dc07c454bd9");
/// # let example =
/// calc_hash!(SHA512: ITER => ["hello", "world"]);
/// # assert_eq!(example.as_str(), "1594244d52f2d8c12b142bb61f47bc2eaf503d6d9ca8480cae9fcf112f66e4967dc5e8fa98285e36db8af1b8ffa8b84cb15e0fbcf836c3deb803c13f37659a60");
/// // Uppercase is supported
/// # let example =
/// calc_hash!(SHA256: UPPERCASE => "hello", "world");
/// # assert_eq!(example.as_str(), "936A185CAAA266BB9CBE981E9E05CB78CD732B0B3280EB944412BB6F8F8F07AF");
/// // Uppercase with iterator passed in is supported
/// # let example =
/// calc_hash!(SHA256: UPPERCASE; ITER => ["hello", "world"]);
/// # assert_eq!(example.as_str(), "936A185CAAA266BB9CBE981E9E05CB78CD732B0B3280EB944412BB6F8F8F07AF");
/// ```
macro_rules! calc_hash {
    (MD5: $($input_str:expr),+) => {{
        use ::md5::Digest;
        let mut hasher = ::md5::Md5::new();
        $crate::calc_hash!(INTERNAL 16; hasher, $($input_str),+)
    }};
    (SHA256: $($input_str:expr),+) => {{
        use ::sha2::Digest;
        let mut hasher = ::sha2::Sha256::new();
        $crate::calc_hash!(INTERNAL 32; hasher, $($input_str),+)
    }};
    (SHA384: $($input_str:expr),+) => {{
        use ::sha2::Digest;
        let mut hasher = sha2::Sha384::new();
        $crate::calc_hash!(INTERNAL 48; hasher, $($input_str),+)
    }};
    (SHA512: $($input_str:expr),+) => {{
        use ::sha2::Digest;
        let mut hasher = sha2::Sha512::new();
        $crate::calc_hash!(INTERNAL 64; hasher, $($input_str),+)
    }};

    (MD5: UPPERCASE => $($input_str:expr),+) => {{
        use md5::Digest;
        let mut hasher = md5::Md5::new();
        $crate::calc_hash!(INTERNAL 16; UPPERCASE; hasher, $($input_str),+)
    }};
    (SHA256: UPPERCASE => $($input_str:expr),+) => {{
        use ::sha2::Digest;
        let mut hasher = ::sha2::Sha256::new();
        $crate::calc_hash!(INTERNAL 32; UPPERCASE; hasher, $($input_str),+)
    }};
    (SHA384: UPPERCASE => $($input_str:expr),+) => {{
        use ::sha2::Digest;
        let mut hasher = sha2::Sha384::new();
        $crate::calc_hash!(INTERNAL 48; UPPERCASE; hasher, $($input_str),+)
    }};
    (SHA512: UPPERCASE => $($input_str:expr),+) => {{
        use ::sha2::Digest;
        let mut hasher = sha2::Sha512::new();
        $crate::calc_hash!(INTERNAL 64; UPPERCASE; hasher, $($input_str),+)
    }};

    (MD5: ITER => $($input_str:expr),+) => {{
        use md5::Digest;
        let mut hasher = md5::Md5::new();
        $crate::calc_hash!(INTERNAL 16; ITER; hasher, $($input_str),+)
    }};
    (SHA256: ITER => $input_iter:expr) => {{
        use ::sha2::Digest;
        let mut hasher = ::sha2::Sha256::new();
        $crate::calc_hash!(INTERNAL 32; ITER; hasher, $input_iter)
    }};
    (SHA384: ITER => $input_iter:expr) => {{
        use ::sha2::Digest;
        let mut hasher = sha2::Sha384::new();
        $crate::calc_hash!(INTERNAL 48; ITER; hasher, $input_iter)
    }};
    (SHA512: ITER => $input_iter:expr) => {{
        use ::sha2::Digest;
        let mut hasher = sha2::Sha512::new();
        $crate::calc_hash!(INTERNAL 64; ITER; hasher, $input_iter)
    }};

    (MD5: UPPERCASE; ITER => $($input_str:expr),+) => {{
        use md5::Digest;
        let mut hasher = md5::Md5::new();
        $crate::calc_hash!(INTERNAL 16; UPPERCASE; ITER; hasher, $($input_str),+)
    }};
    (SHA256: UPPERCASE; ITER => $input_iter:expr) => {{
        use ::sha2::Digest;
        let mut hasher = ::sha2::Sha256::new();
        $crate::calc_hash!(INTERNAL 32; UPPERCASE; ITER; hasher, $input_iter)
    }};
    (SHA384: UPPERCASE; ITER => $input_iter:expr) => {{
        use ::sha2::Digest;
        let mut hasher = sha2::Sha384::new();
        $crate::calc_hash!(INTERNAL 48; UPPERCASE; ITER; hasher, $input_iter)
    }};
    (SHA512: UPPERCASE; ITER => $input_iter:expr) => {{
        use ::sha2::Digest;
        let mut hasher = sha2::Sha512::new();
        $crate::calc_hash!(INTERNAL 64; UPPERCASE; ITER; hasher, $input_iter)
    }};

    // INTERNAL
    (INTERNAL $len:expr; $hasher: expr, $($input_str:expr),+) => {{
        $($hasher.update(&($input_str));)+

        let result = $hasher.finalize();

        #[allow(unsafe_code)]
        ::const_hex::Buffer::<$len, false>::new().const_format(unsafe { &*{ (result).as_ptr() as *const [u8; $len] } })
    }};

    (INTERNAL $len:expr; UPPERCASE; $hasher: expr, $($input_str:expr),+) => {{
        $($hasher.update(&($input_str));)+

        let result = $hasher.finalize();

        #[allow(unsafe_code)]
        ::const_hex::Buffer::<$len, false>::new().const_format_upper(unsafe { &*{ (result).as_ptr() as *const [u8; $len] } })
    }};

    (INTERNAL $len:expr; ITER; $hasher: expr, $input_iter:expr) => {{
        for i in $input_iter {
            $hasher.update(i);
        }

        let result = $hasher.finalize();
        #[allow(unsafe_code)]
        ::const_hex::Buffer::<$len, false>::new().const_format(unsafe { &*{ (result).as_ptr() as *const [u8; $len] } })
    }};

    (INTERNAL $len:expr; UPPERCASE; ITER; $hasher: expr, $input_iter:expr) => {{
        for i in $input_iter {
            $hasher.update(i);
        }

        let result = $hasher.finalize();
        #[allow(unsafe_code)]
        ::const_hex::Buffer::<$len, false>::new().const_format_upper(unsafe { &*{ (result).as_ptr() as *const [u8; $len] } })
    }};
}
