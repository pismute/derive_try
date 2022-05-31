# `derive_try`

[![Build Status](https://github.com/pismute/derive_try/workflows/CI/badge.svg)](https://github.com/pismute/derive_try/actions)
[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/pismute/derive_try/master/LICENSE)
[![Rust Nightly](https://img.shields.io/badge/rust-nightly-red.svg)](https://rust-lang.github.io/rfcs/3058-try-trait-v2.html)

This library is derive macros for [`Try` trait version 2](https://rust-lang.github.io/rfcs/3058-try-trait-v2.html). `Try` trait version 2 is experimental in nightly version. You need a nightly compiler.

It has two macros for newtypes:

- `IdTry`: It implements infallible `Try` trait. It can be used like `Identity` monad.
- `Try`: It implements `Try` trait working through inner type.

## Examples

```rust
#[derive(IdTry)]
struct Id<T>(T);

fn id_sample() -> Id<u8> {
    Id(Id(1)? + Id(2)? + Id(3)?)
}

#[derive(Try)]
struct NewResult<T, E>(Result<T, E>);

impl<T, E> NewResult<T, E> {
    fn new(value: T) -> Self {
        Self(Ok(value))
    }
}

fn new_result_sample() -> NewResult<u8, String> {
    NewResult::new(NewResult::new(1)? + NewResult::new(2)? + NewResult::new(3)?)
}

#[derive(Try)]
struct New<T: ops::Try>(T);

fn new_sample() -> New<Option<u8>> {
    New(Some(New(Some(1))? + New(Some(2))? + New(Some(3))?))
}
```

I made hand-made and macro-generated versions:

- [Try](examples/monadic_try.rs)
- [Try with macros](examples/monadic_try_macro.rs)

## Docs

I made these macros to use `Try` trait on Tagless Final encoding in Rust. I created some docs:

- [Monadic Try](https://github.com/pismute/blog/blob/master/rust/20210515_monadic-try.org)
- [Tagless Final in Rust](https://github.com/pismute/blog/blob/master/rust/20210529_tagless-final.org)
