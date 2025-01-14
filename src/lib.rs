//! Dev deps: some useful macros.

#[cfg(feature = "feat-base64")]
pub mod base64;
#[cfg(feature = "feat-hash")]
pub mod hash;
pub mod misc;
#[cfg(feature = "feat-random")]
pub mod random;
#[cfg(feature = "feat-string")]
pub mod string;

#[cfg(feature = "feat-string")]
#[deprecated(since = "0.8.0", note = "Use `string` in release version instead")]
pub use string as string_v2;

#[macro_export]
/// Faster way to get current timestamp other than
/// `chrono::Local::now().timestamp()`, 12x faster on my machine.
///
/// # Example
///
/// ```rust
/// # use macro_toolset::now;
/// let now_ts_sec = now!().as_secs(); // Seconds since UNIX_EPOCH
/// let now_ts_millis = now!().as_millis(); // Milliseconds since UNIX_EPOCH
/// ```
///
/// See [`Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html) for more details.
macro_rules! now {
    () => {{
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(t) => t,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }};
}

#[macro_export]
/// Init `tracing_subscriber` with default settings.
///
/// This is useful when running tests.
macro_rules! init_tracing_simple {
    () => {{
        use tracing::level_filters::LevelFilter;
        use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

        let fmt_layer = tracing_subscriber::fmt::layer().with_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy()
                .add_directive("otel::tracing=trace".parse().unwrap())
                .add_directive("h2=error".parse().unwrap())
                .add_directive("tower=error".parse().unwrap())
                .add_directive("hyper=error".parse().unwrap()),
        );

        tracing_subscriber::registry().with(fmt_layer).init();
    }};
}

#[macro_export]
/// Helper macro for creating a wrapper type.
///
/// The wrapper type will implement [`Deref`](std::ops::Deref),
/// [`DerefMut`](std::ops::DerefMut), [`From`] and [`AsRef`].
///
/// # Example
///
/// ```rust
/// # use macro_toolset::wrapper;
/// wrapper!(pub MyString(String));
/// // Derive is OK!
/// wrapper!(pub MyStringDerived(String), derive(Debug, Clone, PartialEq, Eq, Hash));
/// // Lifetime is supported too!
/// wrapper!(pub MyStringLifetime<'a>(&'a str));
/// wrapper!(pub MyStringLifetimePubDerived<'a>(pub &'a str), derive(Debug, Clone, PartialEq, Eq, Hash));
/// ```
///
/// Reference: <https://stackoverflow.com/a/61189128>
macro_rules! wrapper {
    ($(#[$outer:meta])* $vis:vis $name:ident$(<$($lt:tt$(:$clt:tt$(+$dlt:tt)*)?),+>)? ($inner_vis:vis $inner:ty) $(, <$($plt_name:ident: $plt:lifetime),+>)? $(, derive($($derive:path),+))?) => {
        $(#[$outer])*
        $(#[derive($($derive),+)])?
        #[repr(transparent)]
        #[doc = concat!("\n\nThis is an auto-generated wrapper over `", stringify!($inner), "`")]
        $vis struct $name$(<$($lt),+>)? {
            /// Inner value
            $inner_vis inner: $inner,
            $($(
                $plt_name: std::marker::PhantomData<&$plt ()>,
            ),+)?
        }

        impl$(<$($lt),+>)? From<$inner> for $name$(<$($lt),+>)? {
            #[inline]
            fn from(inner: $inner) -> Self {
                Self {
                    inner,
                    $($($plt_name: std::marker::PhantomData),+)?
                }
            }
        }

        impl$(<$($lt),+>)? std::borrow::Borrow<$inner> for $name$(<$($lt),+>)? {
            fn borrow(&self) -> &$inner {
                &self.inner
            }
        }

        impl$(<$($lt),+>)? std::ops::Deref for $name$(<$($lt),+>)? {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl$(<$($lt),+>)? std::ops::DerefMut for $name$(<$($lt),+>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl$(<$($lt),+>)? AsRef<$inner> for $name$(<$($lt),+>)? {
            fn as_ref(&self) -> &$inner {
                &self.inner
            }
        }

        impl$(<$($lt),+>)? $name$(<$($lt),+>)? {
            #[inline]
            #[doc = concat!("Creates a new instance of [`", stringify!($name), "`]")]
            $vis const fn new(inner: $inner) -> Self {
                Self {
                    inner,
                    $($($plt_name: std::marker::PhantomData),+)?
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    #![allow(unreachable_pub)]

    // Simple wrapper!
    wrapper!(pub MyString(String));

    wrapper!(#[derive(Debug)] pub MyStringPub(pub String));
    wrapper!(pub MyStringPubCrate(pub(crate) String));
    // Derive is OK!
    wrapper!(pub MyStringDerived(String), derive(Debug, Clone, PartialEq, Eq, Hash));
    wrapper! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        /// MyStringDerived
        pub MyStringDerived2(String)
    }
    wrapper!(pub MyStringPubInnerDerived(pub String), derive(Debug, Clone, PartialEq, Eq, Hash));
    wrapper!(pub MyStringPubCrateInnerDerived(pub(crate) String), derive(Debug, Clone, PartialEq, Eq, Hash));
    // Lifetime is supported too!
    wrapper!(pub MyStringLifetime<'a>(&'a str));
    wrapper!(pub MyStringLifetimePubInner<'a>(pub &'a str));
    wrapper!(pub MyStringLifetimePubCrateInner<'a>(pub(crate) &'a str));
    wrapper!(pub MyStringLifetimePubDerived<'a>(pub &'a str), derive(Debug, Clone, PartialEq, Eq, Hash));
    wrapper!(pub MyStringLifetimePubMutDerived<'a>(pub &'a mut str), derive(Debug, PartialEq, Eq, Hash));
    // Generic is supported too!
    wrapper!(pub MyStringLifetimePubT<T>(pub T), derive(Debug, Clone));
    // Complicated!
    wrapper!(pub MyStringLifetimePubRefT<'a, T>(pub &'a T), derive(Debug, Clone));
}
