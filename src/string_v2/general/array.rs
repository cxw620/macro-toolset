//! Implementations for array-or-slice-like types.
//!
//! - `[T; N]`
//! - `&[T; N]`
//! - `&mut [T; N]`
//! - `&[T]`
//! - `&mut [T]`
//! - `Vec<T>`

use super::StringT;
use crate::str_iter_wrapper;

macro_rules! impl_for_array_or_slice_like {
    ($($tt:tt)*) => {
        $($tt)* {
            #[inline]
            fn encode_to_buf(self, string: &mut Vec<u8>) {
                str_iter_wrapper!(self.into_iter()).encode_to_buf(string);
            }

            #[inline]
            fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                str_iter_wrapper!(self.into_iter()).encode_to_buf_with_separator(string, separator);
            }

            #[inline]
            #[cfg(feature = "feat-string-ext-bytes")]
            fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                str_iter_wrapper!(self.into_iter()).encode_to_bytes_buf(string);
            }

            #[inline]
            #[cfg(feature = "feat-string-ext-bytes")]
            fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                str_iter_wrapper!(self.into_iter()).encode_to_bytes_buf_with_separator(string, separator);
            }
        }
    };
}

impl_for_array_or_slice_like!(impl<T, const N: usize> StringT for [T; N] where T: StringT);
impl_for_array_or_slice_like!(impl<T, const N: usize> StringT for &[T; N] where for<'a> &'a T: StringT);
impl_for_array_or_slice_like!(impl<T> StringT for &[T] where for<'a> &'a T: StringT);
impl_for_array_or_slice_like!(impl<T> StringT for Vec<T> where T: StringT);
