//! Random number / string generation utilities

#[macro_export]
/// Generate random string.
///
/// # Example
///
/// Just add `rand = "0.8"` to your crate deps then try:
///
/// ```rust
/// # use macro_toolset::random_string;
/// #
/// // Use default charset `b"0123456789abcdef"`
/// let rs_1 = random_string!(32);
/// # assert_eq!(rs_1.len(), 32);
/// // Use custom charset
/// let rs_2 = random_string!(32, b"0123456789abcdefABCDEF");
/// # assert_eq!(rs_2.len(), 32);
/// // Provide your own string and the randon string will be appended to it
/// # let mut your_own_string = "test".to_string();
/// random_string!(STR = your_own_string; 32);
/// # assert_eq!(&your_own_string[0..4], "test");
/// # assert_eq!(your_own_string.len(), 36);
/// // Of course, custom charset is supported
/// # let mut your_own_string = "test".to_string();
/// random_string!(STR = your_own_string; 32, b"0123456789abcdefABCDEF");
/// # assert_eq!(&your_own_string[0..4], "test");
/// # assert_eq!(your_own_string.len(), 36);
/// ```
macro_rules! random_string {
    (STR = $string:expr; $range:expr, $charset:expr) => {{
        use ::rand::Rng;

        let mut rng = ::rand::thread_rng();
        (0..$range)
            .for_each(|_| {
                let idx = rng.gen_range(0..$charset.len());
                $string.push($charset[idx] as char);
            });
    }};
    (STR = $string:expr; $range:expr) => {
        $crate::random_string!(STR = $string; $range, b"0123456789abcdef")
    };
    ($range:expr, $charset:expr) => {{
        use ::rand::Rng;

        let mut rng = ::rand::thread_rng();
        (0..$range)
            .map(|_| {
                let idx = rng.gen_range(0..$charset.len());
                $charset[idx] as char
            })
            .collect::<String>()
    }};
    ($range:expr) => {
        $crate::random_string!($range, b"0123456789abcdef")
    };
}

#[cfg(feature = "macros-string")]
#[macro_export]
/// Generate random string base on xor-shift algorithm.
///
/// Notice: Length of string should be always <= 32 (u64)
///
/// # Example
///
/// ```rust
/// # fn main() {
/// # use macro_toolset::random_string_fast;
/// # use macro_toolset::string::StringExtT;
///
/// let rs_lowercase = random_string_fast!(false, 32);
/// let rs_uppercase = random_string_fast!(true, 32);
/// # }
/// ```
macro_rules! random_string_fast {
    ($b:expr, $l:expr) => {{
        use $crate::string::StringExtT;
        $crate::string::NumStr::new_default_hex($crate::random::fast_random())
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
    #[cfg(not(feature = "macros-random-fast"))]
    use std::hash::RandomState;
    use std::{
        cell::Cell,
        hash::{BuildHasher, Hasher},
        num::Wrapping,
    };

    #[cfg(feature = "macros-random-fast")]
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
