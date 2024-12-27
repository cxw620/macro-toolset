#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main, Criterion};
use macro_toolset::{str_concat, str_concat_v2, string::NumStr, string_v2::NumStr as NumStrV2};

fn bench_str_concat(c: &mut Criterion) {
    let mut group = c.benchmark_group("StrConcat");

    group.bench_function("std/format", |b| {
        b.iter(|| std::hint::black_box(format!("{}{}{:2x}{}{:2X}", "test1", 2, 0x3, "test4", 0x5)));
    });

    group.bench_function("string/str_concat", |b| {
        b.iter(|| {
            std::hint::black_box(str_concat!(
                "test1",
                2,
                NumStr::hex_byte_default(0x3),
                "test4",
                NumStr::hex_byte_default(0x5).set_uppercase::<true>(),
                "test1",
                2,
                NumStr::hex_byte_default(0x3),
                "test4",
                NumStr::hex_byte_default(0x5).set_uppercase::<true>()
            ))
        });
    });

    group.bench_function("string_v2/str_concat_v2", |b| {
        b.iter(|| {
            std::hint::black_box(str_concat_v2!(
                "test1",
                2,
                NumStrV2::hex_byte_default(0x3),
                "test4",
                NumStrV2::hex_byte_default(0x5).set_uppercase::<true>(),
                "test1",
                2,
                NumStrV2::hex_byte_default(0x3),
                "test4",
                NumStrV2::hex_byte_default(0x5).set_uppercase::<true>()
            ))
        });
    });
}

criterion_group!(benches, bench_str_concat);
criterion_main!(benches);
