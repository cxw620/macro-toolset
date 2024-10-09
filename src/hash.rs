//! Hash related macros

// re-export const_hex
pub use const_hex;

#[macro_export]
/// Calculate MD5 hash.
///
/// Returns [`const_hex::Buffer`], you may use [`const_hex::Buffer::as_str`] to get the string.
///
/// # Examples
///
/// Just add `md-5 = "0.10"` to your crate deps then try:
///
/// ```
/// # use macro_toolset::md5;
///
/// // General usage. Multiple params supported.
/// let example = md5!("hello", "world");
/// assert_eq!(example.as_str(), "fc5e038d38a57032085441e7fe7010b0");
///
/// // You may pass an iterator as input
/// let example = md5!(ITER => ["hello", "world"]);
/// assert_eq!(example.as_str(), "fc5e038d38a57032085441e7fe7010b0");
///
/// // Uppercase
/// let example = md5!(UPPERCASE; "hello", "world");
/// assert_eq!(example.as_str(), "FC5E038D38A57032085441E7FE7010B0");
///
/// // Uppercase with iterator passed in
/// let example = md5!(UPPERCASE; ITER => ["hello", "world"]);
/// assert_eq!(example.as_str(), "FC5E038D38A57032085441E7FE7010B0");
/// ```
///
/// # Safety
///
/// `GenericArray<u8, 16>` is with memory alignment like [u8; 16].
///
/// See [GenericArray](https://docs.rs/generic-array/latest/generic_array/struct.GenericArray.html)
macro_rules! md5 {
    ($($input_str:expr),+) => {{
        use md5::{Digest, Md5};
        let mut hasher = Md5::new();
        $(hasher.update(&($input_str));)+
        let result = hasher.finalize();
        const_hex::Buffer::<16, false>::new().const_format(unsafe { &*{ (result).as_ptr() as *const [u8; 16] } })
    }};
    (ITER => $input_iter:expr) => {{
        use md5::{Digest, Md5};
        let mut hasher = Md5::new();
        for i in $input_iter {
            hasher.update(i);
        }
        let result = hasher.finalize();
        const_hex::Buffer::<16, false>::new().const_format(unsafe { &*{ (result).as_ptr() as *const [u8; 16] } })
    }};
    (UPPERCASE; $($input_str:expr),+) => {{
        use md5::{Digest, Md5};
        let mut hasher = Md5::new();
        $(hasher.update(&($input_str));)+
        let result = hasher.finalize();
        const_hex::Buffer::<16, false>::new().const_format_upper(unsafe { &*{ (result).as_ptr() as *const [u8; 16] } })
    }};
    (UPPERCASE; ITER => $input_iter:expr) => {{
        use md5::{Digest, Md5};
        let mut hasher = Md5::new();
        for i in $input_iter {
            hasher.update(i);
        }
        let result = hasher.finalize();
        const_hex::Buffer::<16, false>::new().const_format_upper(unsafe { &*{ (result).as_ptr() as *const [u8; 16] } })
    }};
}