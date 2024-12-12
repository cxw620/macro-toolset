#![allow(missing_docs)]

use std::fmt;

use criterion::{criterion_group, criterion_main, Criterion};
use macro_toolset::{
    str_concat,
    string::{HexStr, NumStr, StringExtT},
};

// fn bench_num_str(c: &mut Criterion) {
//     let mut group = c.benchmark_group("NumStr");

//     group.bench_function("usize/123", |b| {
//         b.iter(||
// std::hint::black_box(NumStr::new_default(123_usize).encode()));     });

//     group.bench_function("usize/123_456", |b| {
//         b.iter(||
// std::hint::black_box(NumStr::new_default(123_456_usize).encode()));     });

//     group.bench_function("usize/123_456_789", |b| {
//         b.iter(||
// std::hint::black_box(NumStr::new_default(123_456_789_usize).encode()));
//     });

//     group.bench_function("usize/987_654_321", |b| {
//         b.iter(||
// std::hint::black_box(NumStr::new_default(987_654_321_usize).encode()));
//     });

//     group.bench_function("usize/usize::MAX", |b| {
//         b.iter(||
// std::hint::black_box(NumStr::new_default(usize::MAX).encode()));     });

//     // === u128 ===
//     group.bench_function("u128/123", |b| {
//         b.iter(||
// std::hint::black_box(NumStr::new_default(123_u128).encode()));     });

//     group.bench_function("u128/123_456", |b| {
//         b.iter(||
// std::hint::black_box(NumStr::new_default(123_456_u128).encode()));     });

//     group.bench_function("u128/123_456_789", |b| {
//         b.iter(||
// std::hint::black_box(NumStr::new_default(123_456_789_u128).encode()));
//     });

//     group.bench_function("u128/987_654_321", |b| {
//         b.iter(||
// std::hint::black_box(NumStr::new_default(987_654_321_u128).encode()));
//     });

//     group.bench_function("u128/usize::MAX", |b| {
//         b.iter(||
// std::hint::black_box(NumStr::new_default(0xFFFFFFFFFFFFFFFF_u128).encode()));
//     });
// }

#[repr(transparent)]
struct StdHexSlice<'a>(&'a [u8]);

impl fmt::Display for StdHexSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &byte in self.0 {
            write!(f, "{:0>2x}", byte)?;
        }
        Ok(())
    }
}

fn bench_hex_array(c: &mut Criterion) {
    let mut group = c.benchmark_group("HexArray");

    let result = {
        use md5::{Digest, Md5};
        let mut hasher = Md5::new();
        hasher.update("helloworld");
        hasher.finalize()
    };

    #[allow(unsafe_code)]
    let result = unsafe { &*{ (result).as_ptr() as *const [u8; 16] } };

    group.bench_function("std", |b| {
        b.iter(|| std::hint::black_box(format!("{}", StdHexSlice(result))));
    });

    group.bench_function("const_hex::Buffer::const_format", |b| {
        b.iter(|| {
            std::hint::black_box(
                const_hex::Buffer::<16, false>::new()
                    .const_format(result)
                    .to_string(),
            )
        });
    });

    group.bench_function("const_hex::Buffer::const_format/uppercase", |b| {
        b.iter(|| {
            std::hint::black_box(
                const_hex::Buffer::<16, false>::new()
                    .const_format_upper(result)
                    .to_string(),
            )
        });
    });

    group.bench_function("crate::string::NumStr", |b| {
        b.iter(|| {
            std::hint::black_box({
                str_concat!(result.iter().map(|n| NumStr::hex_byte_default(*n)))
            })
        });
    });

    group.bench_function("crate::string::NumStr/uppercase", |b| {
        b.iter(|| {
            std::hint::black_box({
                str_concat!(result
                    .iter()
                    .map(|n| NumStr::hex_byte_default(*n).set_uppercase::<true>()))
            })
        });
    });

    group.bench_function("crate::string::NumStr/uppercase/direct", |b| {
        b.iter(|| {
            std::hint::black_box({
                str_concat!(result.iter().map(|n| NumStr::<16, true, 0, 0, u8>::new(*n)))
            })
        });
    });

    group.bench_function("crate::string::HexStr", |b| {
        b.iter(|| std::hint::black_box(HexStr::<16>::new(result).to_string_ext()));
    });

    group.bench_function("crate::string::HexStr/uppercase", |b| {
        b.iter(|| {
            std::hint::black_box({
                HexStr::<16>::new(result)
                    .set_uppercase::<true>()
                    .to_string_ext()
            })
        });
    });

    group.bench_function("crate::string::HexStr/uppercase/direct", |b| {
        b.iter(|| std::hint::black_box(HexStr::<16, false, true>::new(result).to_string_ext()));
    });
}

criterion_group!(benches, bench_hex_array);
criterion_main!(benches);
