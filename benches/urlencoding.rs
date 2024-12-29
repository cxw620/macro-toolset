#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main, Criterion};
use macro_toolset::{string_v2::StringExtT, urlencoding_str};

fn bench_urlencoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("UrlEncoding");

    group.bench_function("urlencoding/encode", |b| {
        b.iter(|| std::hint::black_box(urlencoding::encode("你好, 世界")));
    });

    group.bench_function("string_v2/urlencoding/encode", |b| {
        b.iter(|| {
            std::hint::black_box(
                urlencoding_str!(
                    E: "你好, 世界"
                )
                .to_string_ext(),
            )
        });
    });

    group.bench_function("urlencoding/decode", |b| {
        b.iter(|| {
            std::hint::black_box(urlencoding::decode(
                "%E4%BD%A0%E5%A5%BD%2C%20%E4%B8%96%E7%95%8C",
            ))
        });
    });

    group.bench_function("string_v2/urlencoding/decode", |b| {
        b.iter(|| {
            std::hint::black_box(
                urlencoding_str!(D: "%E4%BD%A0%E5%A5%BD%2C%20%E4%B8%96%E7%95%8C").to_string_ext(),
            )
        });
    });
}

criterion_group!(benches, bench_urlencoding);
criterion_main!(benches);
