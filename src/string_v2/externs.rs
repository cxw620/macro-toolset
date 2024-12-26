//! Implementations of extern crate types

macro_rules! impl_for_extern_type {
    ($(#[$outer:meta])* $type:ty: $self:ident, $arg:ident => $block:block) => {
        $(#[$outer])*
        impl super::StringT for $type {
            #[inline]
            fn encode_to_buf($self: Self, $arg: &mut Vec<u8>) $block

            #[inline]
            fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, _separator: &str) {
                self.encode_to_buf(string);
            }

            #[inline]
            #[cfg(feature = "feat-string-ext-bytes")]
            fn encode_to_bytes_buf($self: Self, $arg: &mut bytes::BytesMut) $block

            #[inline]
            #[cfg(feature = "feat-string-ext-bytes")]
            fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, _separator: &str) {
                self.encode_to_bytes_buf(string);
            }
        }
    };
}

#[cfg(feature = "feat-string-ext-ammonia")]
impl super::StringT for ammonia::Document {
    #[inline]
    fn encode_to_buf(self, string: &mut Vec<u8>) {
        self.write_to(string)
            .expect("Writing to a string should not fail (except on OOM)");
    }

    #[inline]
    fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, _separator: &str) {
        self.encode_to_buf(string);
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
        use bytes::BufMut;

        self.write_to(string.writer())
            .expect("Writing to a string should not fail (except on OOM)");
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, _separator: &str) {
        self.encode_to_bytes_buf(string);
    }
}

#[cfg(feature = "feat-string-ext-chrono")]
impl<'a, I: Iterator<Item = B> + Clone, B: std::borrow::Borrow<chrono::format::Item<'a>>>
    super::StringT for chrono::format::DelayedFormat<I>
{
    #[inline]
    fn encode_to_buf(self, string: &mut Vec<u8>) {
        // TODO: Avoid allocation here, though chrono doesn't provide a way to do so.
        string.extend(self.to_string().as_bytes());
    }

    #[inline]
    fn encode_to_buf_with_separator(self, string: &mut Vec<u8>, _separator: &str) {
        self.encode_to_buf(string);
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf(self, string: &mut bytes::BytesMut) {
        // TODO: Avoid allocation here, though chrono doesn't provide a way to do so.
        string.extend(self.to_string().as_bytes());
    }

    #[inline]
    #[cfg(feature = "feat-string-ext-bytes")]
    fn encode_to_bytes_buf_with_separator(self, string: &mut bytes::BytesMut, _separator: &str) {
        self.encode_to_bytes_buf(string);
    }
}

impl_for_extern_type! {
    #[cfg(feature = "feat-string-ext-http")]
    http::HeaderName: self, string => {
        string.extend(self.as_str().as_bytes());
    }
}

impl_for_extern_type! {
    #[cfg(feature = "feat-string-ext-http")]
    http::Method: self, string => {
        string.extend(self.as_str().as_bytes());
    }
}

impl_for_extern_type! {
    #[cfg(feature = "feat-string-ext-http")]
    http::StatusCode: self, string => {
        string.extend(self.as_str().as_bytes());
    }
}

impl_for_extern_type! {
    #[cfg(feature = "feat-string-ext-http")]
    http::Uri: self, string => {
        if let Some(scheme) = self.scheme() {
            string.extend(scheme.as_str().as_bytes());
            string.extend(b"://");
        }

        if let Some(authority) = self.authority() {
            string.extend(authority.as_str().as_bytes());
        }

        string.extend(self.path().as_bytes());

        if let Some(query) = self.query() {
            string.extend(b"?");
            string.extend(query.as_bytes());
        }
    }
}

impl_for_extern_type! {
    #[cfg(feature = "feat-string-ext-http")]
    http::Version: self, string => {
        let str_byte = match self {
            http::Version::HTTP_09 => &b"HTTP/0.9"[..],
            http::Version::HTTP_10 => &b"HTTP/1.0"[..],
            http::Version::HTTP_11 => &b"HTTP/1.1"[..],
            http::Version::HTTP_2 => &b"HTTP/2.0"[..],
            http::Version::HTTP_3 => &b"HTTP/3.0"[..],
            _ => {
                string.extend(format!("{self:?}").as_bytes());
                return;
            }
        };

        string.extend(str_byte);
    }
}
