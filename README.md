# macro-toolset

![Crates.io Version](https://img.shields.io/crates/v/macro-toolset)
![docs.rs](https://img.shields.io/docsrs/macro-toolset)
![GitHub Tag](https://img.shields.io/github/v/tag/cxw620/macro-toolset)

Some useful macros, to make the repetitive code less.

## Features

- `str_concat` almost everything
- Hashing and encode to HEX
  - MD5
  - SHA256
  - SHA384
  - SHA512
- Macro for creating a wrapper struct.
- ...

The features listed in `Cargo.toml`:

- `dev`: For development and enable all features. Not recommended since introducing axum, etc will add many dependencies.
- `feat-base64`: Base64 encode and decode related utilities.
- `feat-hash`: Hash algorithms (MD5, SHA256, SHA384, SHA512) related utilities.
- `feat-random`: Random number / string related utilities. You shall add `rand` to your `Cargo.toml`.
- `feat-random-fast`: Random number / string related utilities. You shall add `rand` to your `Cargo.toml`.
- `feat-string`: String related utilities.
  - `feat-string-ext-ammonia`: Serializes an `ammonia::Document` instance without allocation.
  - `feat-string-ext-axum`: implement `IntoResponse` for `StringExt`.
  - `feat-string-ext-base64`: wrapper that indicates the inner slice should be encoded in base64 over `&[u8]`
  - `feat-string-ext-hex`: hex encoding based on `const-hex`, with better performance than `NumStr`.
  - `feat-string-ext-http`: crate `http` integration.
  - `feat-string-ext-ryu`: float number support.

## MSRV

1.66

## Notice

This crate is currently intended for personal use mostly, but contributions are welcome.

## LICENSE

GPL-3.0-only
