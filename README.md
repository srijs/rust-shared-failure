# `shared_failure`

[![Build Status](https://travis-ci.org/srijs/rust-shared-failure.svg?branch=master)](https://travis-ci.org/srijs/rust-shared-failure)
[![Dependency Status](https://deps.rs/repo/github/srijs/rust-shared-failure/status.svg)](https://deps.rs/repo/github/srijs/rust-shared-failure)

This crate aims to provide a convenient and lightweight way
to clone errors and share them across thread-boundaries.

It is designed to be used in conjunction with the
[`failure`](https://crates.io/crates/failure) crate.

## Example

```rust
let custom_error = std::io::Error::new(std::io::ErrorKind::Other, "oh no!");

let shared_error = SharedFailure::from_fail(custom_error);

// can be cloned, even though std::io::Error does not impl Clone
let cloned_error = shared_error.clone();

assert_eq!(shared_error.to_string(), "oh no!");
assert_eq!(cloned_error.to_string(), "oh no!");

assert_eq!(shared_error.downcast_ref::<std::io::Error>().unwrap().kind(),
    std::io::ErrorKind::Other);
```
