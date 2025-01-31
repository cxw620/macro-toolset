//! Implementations for tuples.
//!
//! Since array-or-slice-like types can only carry  elements of the same type,
//! tuple is the only way to `push` multiple elements with different types.

use super::{StringExtT, StringT};
use crate::wrapper;

wrapper! {
    #[derive(Debug, Clone, Copy)]
    /// Tuple but will not insert separator between elements.
    pub SeplessTuple<T>(pub T)
}

macro_rules! impl_for_tuple {
    ( $( $name:ident )+ ) => {
        #[allow(non_snake_case)]
        impl<$($name: StringT),+> StringT for ($($name,)+) {
            #[inline]
            fn encode_to_buf(self, string: &mut Vec<u8>) {
                let ($($name,)+) = self;
                $(
                    $name.encode_to_buf(string);
                )+
            }

            #[inline]
            fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                let ($($name,)+) = self;
                $(
                    $name.encode_to_buf_with_separator(string, separator);
                )+
            }

            #[inline]
            fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                let ($($name,)+) = self;
                $(
                    $name.encode_to_bytes_buf(string);
                )+
            }

            #[inline]
            fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                let ($($name,)+) = self;
                $(
                    $name.encode_to_bytes_buf_with_separator(string, separator);
                )+
            }
        }

        #[allow(non_snake_case)]
        impl<$($name: StringExtT),+> StringExtT for ($($name,)+) {
            #[inline]
            fn with_prefix<P: StringExtT + Copy>(self, prefix: P) -> impl StringExtT {
                let ($($name,)+) = self;

                (
                    $(
                        SeplessTuple {
                            inner: (prefix, $name),
                        },
                    )+
                )
            }

            #[inline]
            fn with_suffix<S: StringExtT + Copy>(self, suffix: S) -> impl StringExtT {
                let ($($name,)+) = self;

                (
                    $(
                        SeplessTuple {
                            inner: ($name, suffix),
                        },
                    )+
                )
            }
        }

        #[allow(non_snake_case)]
        impl<$($name: StringT),+> StringT for SeplessTuple<($($name,)+)> {
            #[inline]
            fn encode_to_buf(self, string: &mut Vec<u8>) {
                self.inner.encode_to_buf(string);
            }

            #[inline]
            fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, separator: &str) {
                self.inner.encode_to_buf(string);
                string.extend(separator.as_bytes());
            }

            #[inline]
            fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
                self.inner.encode_to_bytes_buf(string);
            }

            #[inline]
            fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, separator: &str) {
                self.inner.encode_to_bytes_buf(string);
                string.extend(separator.as_bytes());
            }
        }

        #[allow(non_snake_case)]
        impl<$($name: StringExtT),+> StringExtT for SeplessTuple<($($name,)+)> {}
    };
}

impl_for_tuple!(T1);
impl_for_tuple!(T1 T2);
impl_for_tuple!(T1 T2 T3);
impl_for_tuple!(T1 T2 T3 T4);
impl_for_tuple!(T1 T2 T3 T4 T5);
impl_for_tuple!(T1 T2 T3 T4 T5 T6);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28 T29);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28 T29 T30);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28 T29 T30 T31);
impl_for_tuple!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28 T29 T30 T31 T32);
