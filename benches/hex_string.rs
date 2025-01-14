#![allow(missing_docs)]

use std::fmt;

use criterion::{criterion_group, criterion_main, Criterion};
use macro_toolset::{
    str_concat,
    string::{HexStr, NumStr, StringExtT},
};

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

#[repr(transparent)]
struct StdHexSliceUppercase<'a>(&'a [u8]);

impl fmt::Display for StdHexSliceUppercase<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &byte in self.0 {
            write!(f, "{:0>2X}", byte)?;
        }
        Ok(())
    }
}

fn bench_hex_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("HexString");

    let result = {
        use md5::{Digest, Md5};
        let mut hasher = Md5::new();
        hasher.update("helloworld");
        hasher.finalize()
    };

    #[allow(unsafe_code)]
    let result = unsafe { &*{ (result).as_ptr() as *const [u8; 16] } };

    group.bench_function("std/lowercase", |b| {
        b.iter(|| std::hint::black_box(format!("{}", StdHexSlice(result))));
    });

    group.bench_function("std/uppercase", |b| {
        b.iter(|| std::hint::black_box(format!("{}", StdHexSliceUppercase(result))));
    });

    group.bench_function("const_hex::Buffer::const_format/lowercase", |b| {
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

    group.bench_function("NumStr/lowercase", |b| {
        b.iter(|| {
            std::hint::black_box({
                str_concat!(result.iter().map(|n| NumStr::hex_byte_default(*n)))
            })
        });
    });

    group.bench_function("NumStr/uppercase", |b| {
        b.iter(|| {
            std::hint::black_box({
                str_concat!(result
                    .iter()
                    .map(|n| NumStr::hex_byte_default(*n).set_uppercase::<true>()))
            })
        });
    });

    group.bench_function("NumStr/uppercase/direct", |b| {
        b.iter(|| {
            std::hint::black_box({
                str_concat!(result.iter().map(|n| NumStr::<16, true, 0, 0, u8>::new(*n)))
            })
        });
    });

    group.bench_function("HexStr/lowercase", |b| {
        b.iter(|| std::hint::black_box(HexStr::<16>::new(result).to_string_ext()));
    });

    group.bench_function("HexStr/uppercase", |b| {
        b.iter(|| {
            std::hint::black_box({
                HexStr::<16>::new(result)
                    .set_uppercase::<true>()
                    .to_string_ext()
            })
        });
    });

    group.bench_function("HexStr/uppercase/direct", |b| {
        b.iter(|| std::hint::black_box(HexStr::<16, false, true>::new(result).to_string_ext()));
    });
}

criterion_group!(benches, bench_hex_string);
criterion_main!(benches);
