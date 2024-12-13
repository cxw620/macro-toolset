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
/// [Map](std::iter::Map) is accepted too.
///
/// ```
/// # use macro_toolset::{slice_sep, str_concat};
/// let slice_sep = slice_sep!(["a", "b", "c"].iter().map(|s| s.to_ascii_uppercase()), ',');
/// assert_eq!(
///     str_concat!("Hello: ", slice_sep, " World!"),
///     "Hello: A,B,C World!"
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
///
/// # Notes
///
/// Take the following as an example:
///
/// ```should_panic
/// # use macro_toolset::{slice_sep, string::StringExtT};
/// let post_ids = vec![1, 2, 3];
/// let l = slice_sep!(
///     post_ids
///         .iter()
///         .map(|id| { slice_sep!(("post_ids[]=", id), "") }),
///     '&'
/// )
/// .to_string_ext();
/// let r = slice_sep!(post_ids.iter().map(|id| { ("post_ids[]=", id) }), '&').to_string_ext();
/// assert_eq!(l, r);
/// ```
///
/// Since the contents in tuple like `("post_ids[]=", id)` are recognized as
/// independent, the separator will be also inserted between them.
///
/// To avoid this, you can use `slice_sep!` with an empty separator to avoid
/// recognizing the contents as independent, while `()` is better:
///
/// ```
/// # use macro_toolset::{slice_sep, string::StringExtT};
/// let post_ids = vec![1, 2, 3];
/// let l = slice_sep!(
///     post_ids
///         .iter()
///         .map(|id| { slice_sep!(("post_ids[]=", id), "") }),
///     '&'
/// )
/// .to_string_ext();
/// let r = slice_sep!(
///     post_ids
///         .iter()
///         .map(|id| { slice_sep!(("post_ids[]=", id)) }), // No separator specified.
///     '&'
/// )
/// .to_string_ext();
/// assert_eq!(l, r);
/// ```
macro_rules! slice_sep {
    ($inner:expr) => {
        $crate::string::SliceSep {
            inner: $inner,
            separator: (),
        }
    };
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
pub struct SliceSep<S, T: StringExtT> {
    /// The inner slice.
    pub inner: T,

    /// The separator.
    pub separator: S,
}

impl<S: SeparatorT, T: StringExtT> SliceSep<S, T> {
    #[inline]
    /// Create a new slice, separated by a separator.
    pub const fn new(inner: T, separator: S) -> Self {
        Self { inner, separator }
    }
}

impl<S: SeparatorT, T: StringExtT> StringExtT for SliceSep<S, T> {
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        self.inner
            .push_to_string_with_separator(string, self.separator);
        self.separator.remove_end(string);
    }
}

impl<T: StringExtT> StringExtT for SliceSep<(), T> {
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        self.inner.push_to_string(string);
    }
}

impl<S: SeparatorT, T: StringExtT + Copy> StringExtT for &SliceSep<S, T> {
    #[inline]
    fn push_to_string(self, string: &mut Vec<u8>) {
        (*self).push_to_string(string);
    }
}
