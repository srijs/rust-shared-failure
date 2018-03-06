//! This crate aims to provide a convenient and lightweight way
//! to clone errors and share them across thread-boundaries.
//!
//! It is designed to be used in conjunction with the
//! [`failure`](https://crates.io/crates/failure) crate.
//!
//! # Example
//!
//! ```rust
//! # extern crate failure;
//! # extern crate shared_failure;
//! #
//! # use shared_failure::SharedFailure;
//! #
//! # fn main() {
//! let custom_error = std::io::Error::new(std::io::ErrorKind::Other, "oh no!");
//!
//! let shared_error = SharedFailure::from_fail(custom_error);
//!
//! // can be cloned, even though std::io::Error does not impl Clone
//! let cloned_error = shared_error.clone();
//!
//! assert_eq!(shared_error.to_string(), "oh no!");
//! assert_eq!(cloned_error.to_string(), "oh no!");
//!
//! assert_eq!(shared_error.downcast_ref::<std::io::Error>().unwrap().kind(),
//!     std::io::ErrorKind::Other);
//! # }
//! ```

extern crate failure;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::Arc;

use failure::{Backtrace, Error, Fail};

/// Failure that can be cloned and shared across thread boundaries.
#[derive(Clone, Debug)]
pub struct SharedFailure(Arc<Error>);

impl SharedFailure {
    /// Wraps the provided error into a `SharedFailure`.
    pub fn from_fail<T: Fail>(err: T) -> SharedFailure {
        SharedFailure(Arc::new(err.into()))
    }

    /// Attempts to downcast this `SharedFailure` to a particular `Fail` type by reference.
    ///
    /// If the underlying error is not of type `T`, this will return [`None`](None()).
    pub fn downcast_ref<T: Fail>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }
}

impl Fail for SharedFailure {
    fn cause(&self) -> Option<&Fail> {
        Some(self.0.cause())
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        Some(self.0.backtrace())
    }
}

impl Display for SharedFailure {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl From<Error> for SharedFailure {
    fn from(err: Error) -> SharedFailure {
        SharedFailure(Arc::new(err))
    }
}
