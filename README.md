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
- ...

The features listed in `Cargo.toml`:

- `dev`: For development and enable all features. Not recommended since introducing axum, etc will add many dependencies.
- `macros-base64`: Base64 encode and decode related utilities.
- `macros-hash`: Hash algorithms (MD5, SHA256, SHA384, SHA512) related utilities.
- `macros-random`: Random number / string related utilities. You shall add `rand` to your `Cargo.toml`.
- `macros-random-fast`: Random number / string related utilities. You shall add `rand` to your `Cargo.toml`.
- `macros-string`: String related utilities.
  - `macros-string-ext-axum`: implement `IntoResponse` for StringExt.
  - `macros-string-ext-base64`: wrapper that indicates the inner slice should be encoded in base64 over `&[u8]`
  - `macros-string-ext-hex`: advanced hex encoding based on `const-hex`, with better performance than simple `NumStr`.
  - `macros-string-ext-ryu`: float number support.

## MSRV

1.66

## Notice

This crate is currently intended for personal use mostly, but contributions are welcome.

## LICENSE

GPL-3.0-only
