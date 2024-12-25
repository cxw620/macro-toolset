#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main, Criterion};
use macro_toolset::string::{RandHexStr, RandStr, StringExtT};

fn bench_rand_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("RandString");

    group.bench_function("RandStr/0123456789abcdef/16/direct", |b| {
        b.iter(|| {
            std::hint::black_box(RandStr::<16>::with_charset(b"0123456789abcdef").to_string_ext())
        });
    });

    group.bench_function("RandHexStr/16/direct", |b| {
        b.iter(|| std::hint::black_box(RandHexStr::<16, 1, 0>::new().to_string_ext()));
    });

    group.bench_function("RandStr/0123456789abcdef/16", |b| {
        b.iter(|| {
            std::hint::black_box(
                RandStr::with_charset_default(b"0123456789abcdef")
                    .with_l::<16>()
                    .to_string_ext(),
            )
        });
    });

    group.bench_function("RandHexStr/16", |b| {
        b.iter(|| {
            std::hint::black_box(
                RandHexStr::new_default()
                    .with_l::<16>()
                    .with_rp::<1>()
                    .with_lp::<0>()
                    .to_string_ext(),
            )
        });
    });

    // 32

    group.bench_function("RandStr/0123456789abcdef/32/direct", |b| {
        b.iter(|| {
            std::hint::black_box(RandStr::<32>::with_charset(b"0123456789abcdef").to_string_ext())
        });
    });

    group.bench_function("RandHexStr/32/direct", |b| {
        b.iter(|| std::hint::black_box(RandHexStr::<16, 2, 0>::new().to_string_ext()));
    });

    group.bench_function("RandStr/0123456789abcdef/32", |b| {
        b.iter(|| {
            std::hint::black_box(
                RandStr::with_charset_default(b"0123456789abcdef")
                    .with_l::<32>()
                    .to_string_ext(),
            )
        });
    });

    group.bench_function("RandHexStr/32", |b| {
        b.iter(|| {
            std::hint::black_box(
                RandHexStr::new_default()
                    .with_l::<16>()
                    .with_rp::<2>()
                    .with_lp::<0>()
                    .to_string_ext(),
            )
        });
    });

    // 56

    group.bench_function("RandStr/0123456789abcdef/56/direct", |b| {
        b.iter(|| {
            std::hint::black_box(RandStr::<56>::with_charset(b"0123456789abcdef").to_string_ext())
        });
    });

    group.bench_function("RandHexStr/56/direct", |b| {
        b.iter(|| std::hint::black_box(RandHexStr::<16, 3, 8>::new().to_string_ext()));
    });

    group.bench_function("RandStr/0123456789abcdef/56", |b| {
        b.iter(|| {
            std::hint::black_box(
                RandStr::with_charset_default(b"0123456789abcdef")
                    .with_l::<56>()
                    .to_string_ext(),
            )
        });
    });

    group.bench_function("RandHexStr/56", |b| {
        b.iter(|| {
            std::hint::black_box(
                RandHexStr::new_default()
                    .with_l::<16>()
                    .with_rp::<3>()
                    .with_lp::<8>()
                    .to_string_ext(),
            )
        });
    });

    // 64

    group.bench_function("RandStr/0123456789abcdef/64/direct", |b| {
        b.iter(|| {
            std::hint::black_box(RandStr::<64>::with_charset(b"0123456789abcdef").to_string_ext())
        });
    });

    group.bench_function("RandHexStr/64/direct", |b| {
        b.iter(|| std::hint::black_box(RandHexStr::<16, 4, 0>::new().to_string_ext()));
    });

    group.bench_function("RandStr/0123456789abcdef/64", |b| {
        b.iter(|| {
            std::hint::black_box(
                RandStr::with_charset_default(b"0123456789abcdef")
                    .with_l::<64>()
                    .to_string_ext(),
            )
        });
    });

    group.bench_function("RandHexStr/64", |b| {
        b.iter(|| {
            std::hint::black_box(
                RandHexStr::new_default()
                    .with_l::<16>()
                    .with_rp::<4>()
                    .with_lp::<0>()
                    .to_string_ext(),
            )
        });
    });

    // 128

    group.bench_function("RandStr/0123456789abcdef/128/direct", |b| {
        b.iter(|| {
            std::hint::black_box(RandStr::<128>::with_charset(b"0123456789abcdef").to_string_ext())
        });
    });

    group.bench_function("RandHexStr/128/direct", |b| {
        b.iter(|| std::hint::black_box(RandHexStr::<16, 8, 0>::new().to_string_ext()));
    });

    group.bench_function("RandStr/0123456789abcdef/128", |b| {
        b.iter(|| {
            std::hint::black_box(
                RandStr::with_charset_default(b"0123456789abcdef")
                    .with_l::<128>()
                    .to_string_ext(),
            )
        });
    });

    group.bench_function("RandHexStr/128", |b| {
        b.iter(|| {
            std::hint::black_box(
                RandHexStr::new_default()
                    .with_l::<16>()
                    .with_rp::<8>()
                    .with_lp::<0>()
                    .to_string_ext(),
            )
        });
    });
}

criterion_group!(benches, bench_rand_string);
criterion_main!(benches);
