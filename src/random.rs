//! Random number / string generation utilities

#[macro_export]
/// Generate random `String`.
///
/// For [`PushAnyT::push_any`](crate::string::PushAnyT::push_any),
/// [`random_str`](crate::random_str) is recommended.
///
/// # Example
///
/// Just add `rand = "0.8.5"` to your crate deps then try:
///
/// ```rust
/// # use macro_toolset::random_string;
/// #
/// // Use default charset `b"0123456789abcdef"` **NOT RECOMMEND, use RandHexStr instead**
/// let rs_1 = random_string!(32);
/// # assert_eq!(rs_1.len(), 32);
/// // Use custom charset
/// let rs_2 = random_string!(32, b"0123456789abcdefABCDEF");
/// # assert_eq!(rs_2.len(), 32);
/// // Provide your own string and the randon string will be appended to it
/// # let mut your_own_string = "test".to_string();
/// random_string!(32 => your_own_string);
/// # assert_eq!(&your_own_string[0..4], "test");
/// # assert_eq!(your_own_string.len(), 36);
/// // Of course, custom charset is supported
/// # let mut your_own_string = "test".to_string();
/// random_string!(32, b"0123456789abcdefABCDEF" => your_own_string);
/// # assert_eq!(&your_own_string[0..4], "test");
/// # assert_eq!(your_own_string.len(), 36);
/// ```
macro_rules! random_string {
    ($range:expr, $charset:expr => $string:expr) => {{
        use ::rand::{distributions::Slice, Rng};

        $string.extend(
            ::rand::thread_rng()
                .sample_iter(Slice::new($charset).unwrap())
                .take($range)
                .map(|&c| c as char)
        );
    }};
    ($range:expr, $charset:expr) => {{
        use ::rand::{distributions::Slice, Rng};

        let mut string = String::with_capacity($range);
        string.extend(
            ::rand::thread_rng()
                .sample_iter(Slice::new($charset).unwrap())
                .take($range)
                .map(|&c| c as char)
        );
        string
    }};
    ($range:expr => $string:expr) => {
        $crate::random_string!($range, b"0123456789abcdef" => $string)
    };
    ($range:expr) => {
        $crate::random_string!($range, b"0123456789abcdef")
    };
}

#[deprecated(since = "0.7.12", note = "Use `RandHexStr` instead")]
#[cfg(feature = "feat-string")]
#[macro_export]
/// Generate random string base on xor-shift algorithm.
///
/// Notice: Length of string should be always <= 16 (u64)
macro_rules! random_string_fast {
    ($b:expr, $l:expr) => {{
        use $crate::string::StringExtT;
        $crate::string::NumStr::hex_default($crate::random::fast_random())
            .set_uppercase::<$b>()
            .to_string_ext()
    }};
}

#[inline]
/// [xorshift*] is a fast pseudorandom number generator which will
/// even tolerate weak seeding, as long as it's not zero.
///
/// [xorshift*]: https://en.wikipedia.org/wiki/Xorshift#xorshift*
pub fn fast_random() -> u64 {
    #[cfg(not(feature = "feat-random-fast"))]
    use std::hash::RandomState;
    use std::{
        cell::Cell,
        hash::{BuildHasher, Hasher},
        num::Wrapping,
    };

    #[cfg(feature = "feat-random-fast")]
    use ::foldhash::fast::RandomState;

    thread_local! {
        static RNG: Cell<Wrapping<u64>> = Cell::new(Wrapping(seed()));
    }

    fn seed() -> u64 {
        let seed = RandomState::default();

        let mut out = 0;
        let mut cnt = 0;
        while out == 0 {
            cnt += 1;
            let mut hasher = seed.build_hasher();
            hasher.write_usize(cnt);
            out = hasher.finish();
        }
        out
    }

    RNG.with(|rng| {
        let mut n = rng.get();
        debug_assert_ne!(n.0, 0);
        n ^= n >> 12;
        n ^= n << 25;
        n ^= n >> 27;
        rng.set(n);
        n.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    })
}

#[macro_export]
/// Generate a random string by choosing ones from given candidates.
///
/// Candidates should be `Vec<&str>` or `[&'a str]`.
///
/// # Examples
///
/// Here's an example rewritten from the original JavaScript code.
///
/// ```
/// # use macro_toolset::random_choice;
/// #
/// static DIGHT_MAP: [&'static str; 17] = [
/// "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "10",
/// ];
///
/// let rc_1 = random_choice!(32, DIGHT_MAP);
/// let rc_2 = random_choice!(8, 4, 4, 4, 12; "-"; DIGHT_MAP); // like `8310B0E0A-40105-9EC3-8298-36C75D10FEA59`
/// ```
macro_rules! random_choice {
    ($range:expr, $choice_set:expr) => {{
        let mut rng = ::rand::thread_rng();
        let mut result = String::with_capacity(32);
        (0..$range).for_each(|_| {
            result.push_str($choice_set[::rand::Rng::gen_range(&mut rng, 0..$choice_set.len())]);
        });
        result
    }};
    ($($range:expr),+; $split:expr; $choice_set:expr) => {{
        let mut rng = ::rand::thread_rng();
        let mut result = String::with_capacity(32);
        $(
            (0..$range).for_each(|_| {
                result.push_str($choice_set[::rand::Rng::gen_range(&mut rng, 0..$choice_set.len())]);
            });
            result.push_str($split);
        )+
        result.truncate(result.len() - $split.len());
        result
    }};
}
