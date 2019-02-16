# syn-mid

[![Build Status](https://travis-ci.com/taiki-e/syn-mid.svg?branch=master)](https://travis-ci.com/taiki-e/syn-mid)
[![version](https://img.shields.io/crates/v/syn-mid.svg)](https://crates.io/crates/syn-mid/)
[![documentation](https://docs.rs/syn-mid/badge.svg)](https://docs.rs/syn-mid/)
[![license](https://img.shields.io/crates/l/syn-mid.svg)](https://crates.io/crates/syn-mid/)
[![Rustc Version](https://img.shields.io/badge/rustc-1.15+-lightgray.svg)](https://blog.rust-lang.org/2017/02/02/Rust-1.15.html)

Providing the features between "full" and "derive" of syn.

This crate provides the following two unique data structures.

* `syn_mid::ItemFn` -- A function whose body is not parsed.

  ```text
  fn process(n: usize) -> Result<()> { ... }
  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ^     ^
  ```

* `syn_mid::Block` -- A block whose body is not parsed.

  ```text
  { ... }
  ^     ^
  ```

Other data structures are the same as data structures of [syn]. These are defined in this crate because they cannot be used in [syn] without "full" feature.

[syn]: https://github.com/dtolnay/syn

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
syn-mid = "0.2"
```

The current version of syn-mid requires Rust 1.15 or later.

[**Examples**](examples)

[**Documentation**](https://docs.rs/syn-mid/)

## Optional features

syn-mid in the default features aims to provide the features between "full" and "derive" of [syn].

* **`clone-impls`** *(enabled by default)* — Clone impls for all syntax tree
  types.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
