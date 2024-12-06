//! A slice, separated by a separator.

use super::{SeparatorT, StringExtT};

#[macro_export]
/// Create a [`SliceSep`].
///
/// # Examples
///
/// Generally, this macro takes a slice of [`StringExtT`] types, like `Vec<T>`
/// or `&[T]` where T implements [`StringExtT`].
///
/// ```
/// # use macro_toolset::{slice_sep, str_concat};
/// let slice_sep = slice_sep!(["a", "b", "c"], ',');
/// assert_eq!(
///     str_concat!("Hello: ", &slice_sep, " World!"),
///     "Hello: a,b,c World!"
/// );
/// let slice_sep = slice_sep!(vec!["a", "b", "c"], ',');
/// assert_eq!(
///     str_concat!(sep = ';'; "TEXT_1", slice_sep, "2_TEXT"),
///     "TEXT_1;a,b,c;2_TEXT"
/// );
/// ```
///
/// In fact all types that implements `StringExtT` can be used, though useless
/// and do not do so.
///
/// ```
/// # use macro_toolset::{slice_sep, str_concat};
/// let slice_sep = slice_sep!("a", ',');
/// assert_eq!(
///     str_concat!(sep = ';'; "TEXT_1", &slice_sep, "2_TEXT"),
///     "TEXT_1;a;2_TEXT"
/// );
/// ```
macro_rules! slice_sep {
    ($inner:expr, $sep:expr) => {
        $crate::string::SliceSep {
            inner: $inner,
            separator: $sep,
        }
    };
}

#[derive(Debug, Clone, Copy)]
/// A slice, separated by a separator.
///
/// The separator you specify like `str_concat!(sep = ',', ...)` will override
/// the one you set when creating [`SliceSep`].
pub struct SliceSep<S: SeparatorT, T: StringExtT> {
    /// The inner slice.
    pub inner: T,

    /// The separator.
    pub separator: S,
}

impl<S: SeparatorT, T: StringExtT> SliceSep<S, T> {
    #[inline]
    /// Create a new slice, separated by a separator.
    pub const fn new(slice: T, separator: S) -> Self {
        Self {
            inner: slice,
            separator,
        }
    }
}

impl<S: SeparatorT, T: StringExtT> StringExtT for SliceSep<S, T> {
    fn push_to_string(self, string: &mut Vec<u8>) {
        self.inner
            .push_to_string_with_separator(string, self.separator);
        self.separator.remove_end(string);
    }
}

impl<S: SeparatorT, T: StringExtT + Copy> StringExtT for &SliceSep<S, T> {
    fn push_to_string(self, string: &mut Vec<u8>) {
        (*self).push_to_string(string);
    }
}
