use std::convert::Infallible;
use thiserror::Error;

/// Error type used by `gittemplate`.
#[derive(Error, Debug)]
pub enum Error {
    /// An expression cannot be parsed into an AST.
    #[error("ParseError: {0}: {1}")]
    ParseError(String, String),

    /// Error caused by libgit2.
    #[error("{0}")]
    Git2(#[from] git2::Error),

    /// Error from `serde_json`.
    #[error("{0}")]
    Json(#[from] serde_json::Error),

    /// Error from `regex`.
    #[error("{0}")]
    Regex(#[from] regex::Error),

    /// Error caused by gitrevset.
    #[error("cannot evaluate revset: {0}")]
    Revset(#[from] gitrevset::Error),

    /// Error caused by formatter.
    #[error("format error: '{0}'")]
    Format(#[from] std::fmt::Error),

    /// Error caused by writing to IO.
    #[error("IOError: '{0}'")]
    Io(#[from] std::io::Error),

    /// Invalid time (out of range).
    #[error("invalid time: {0}")]
    InvalidTime(String),

    /// A name cannot be resolved.
    #[error("symbol not found: {0}")]
    UnresolvedSymbol(String),

    /// Type mismatch.
    #[error("expression {0} does not match expected type {1}")]
    MismatchedType(String, String),

    /// A function call with wrong number of arguments.
    #[error("function {0} requires {1:?} arguments, but got {2} arguments")]
    MismatchedArguments(String, Box<dyn std::fmt::Debug>, usize),

    /// Incompatible types.
    #[error("incompatible types: {0}")]
    IncompatibleTypes(String),
}

impl From<gitrevset::dag::Error> for Error {
    fn from(e: gitrevset::dag::Error) -> Self {
        gitrevset::Error::from(e).into()
    }
}

impl From<Infallible> for Error {
    fn from(_e: Infallible) -> Self {
        unreachable!()
    }
}
