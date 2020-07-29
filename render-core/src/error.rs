#![allow(dead_code)]

use std::fmt;
use std::path::{Path, PathBuf};

/// A type alias for handling errors throughout render-core
pub type Result<T> = std::result::Result<T, Error>;

/// The specific kind of error that can occur.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error that occurred while interacting with a render backend
    #[error("backend error: {0}")]
    Backend(String),

    /// An error that occurred while interacting with a render device
    #[error("device error: {0}")]
    Device(String),

    /// An error that occurred while encoding render commands
    #[error("encoder error: {0}")]
    Encoder(String),

    /// An error that occurred while parsing a data source
    #[error("parse error: {0}")]
    Parse(String),

    /// An error that occurred while accessing or allocating memory
    #[error("memory error: {0}")]
    Memory(String),

    /// An error that occurred while working with a file path.
    #[error("{0}")]
    Path(PathBuf),

    /// Generally, these errors correspond to bugs in this
    /// library.
    #[error("BUG: {0}\nPlease report this bug with a backtrace at https://github.com/gwihlidal/render-rs")]
    Bug(String),

    /// An error occurred while reading/writing the index config.
    #[error("config error: {0}")]
    Config(String),

    /// An unexpected I/O error occurred.
    #[error("I/O error")]
    Io,

    /// An error occurred while parsing a number in a free-form query.
    #[error("error parsing number")]
    Number,

    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    #[error("invaild error")]
    __Nonexhaustive,
}

impl Error {
    pub fn backend(s: impl Into<String>) -> Self {
        Self::Backend(s.into())
    }
}
