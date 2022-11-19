use std::{env, error, fmt, io};

/// A generic error type.
///
/// Certain domain-specific errors (`env::VarError`, `io::Error`, or `ureq::Error`)
/// can be implicitly converted to this error type, e.g., when using the `?` operator
/// to propagate the error from inside the function.
///
/// An alternative approach to propagate different errors from a function would be
/// to return `Result<T, Box<dyn std::error::Error>>`, and pay the price of handling
/// the error type dynamically at runtime.
///
/// # Example
///
/// ```
/// use std::{env, fs, result::Result};
/// use adventofcode as aoc;
///
/// fn do_something(choice: i32) -> Result<(), aoc::Error> {
///     match choice {
///         0 => { fs::read("nonexistent.path")?; }
///         1 => { env::var("NONEXISTENT")?; }
///         2 => { ureq::get("foo").call()?; }
///         _ => {}
///     };
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: impl Into<String>) -> Self {
        Self { msg: msg.into() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.msg)
    }
}

impl error::Error for Error {}

macro_rules! impl_from_trait(
    ($($error_type:ty),+) => {
        $(
            impl core::convert::From<$error_type> for Error {
                fn from(err: $error_type) -> Self {
                    Self { msg: err.to_string() }
                }
            }
        )+
    }
);

impl_from_trait!(env::VarError, io::Error, ureq::Error);

/// Macro to return adventofcode::Error as Result.
///
/// # Example
/// ```no_run
/// use adventofcode::{make_err, Error};
/// use std::result::Result;
///
/// fn do_stuff(what: &str) -> Result<(), Error> {
///     make_err!("Don't know how to do stuff: {}", what)
/// }
/// ```
#[macro_export]
macro_rules! make_err {
    ($($arg:tt)*) => {
        Err($crate::Error::new(format!($($arg)*)))
    };
}
