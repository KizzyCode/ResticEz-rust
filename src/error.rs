use std::backtrace::{Backtrace, BacktraceStatus};
use std::fmt::{self, Display, Formatter};

/// Creates a new variant
#[macro_export]
macro_rules! e {
    ($kind:expr, $($arg:tt)*) => ({ $crate::error::Error::new($kind, format!($($arg)*)) })
}
/// Creates a new `ErrorImpl::ExecError` kind
#[macro_export]
macro_rules! eexec {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::ExecError, $($arg)*) });
}
/// Creates a new `ErrorImpl::InOutError` kind
#[macro_export]
macro_rules! eio {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::InOutError, $($arg)*) });
}
/// Creates a new `ErrorImpl::InvalidValue` kind
#[macro_export]
macro_rules! einval {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::InvalidValue, $($arg)*) });
}

/// The error kind
#[derive(Debug)]
pub enum ErrorKind {
    /// Failed to execute a binary
    ExecError,
    /// An I/O-related error occurred
    InOutError,
    /// A value is invalid
    InvalidValue,
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::ExecError => write!(f, "Failed to execute a binary"),
            Self::InOutError => write!(f, "An I/O-error occurred"),
            Self::InvalidValue => write!(f, "A value is invalid"),
        }
    }
}

// Define our custom error type
#[derive(Debug)]
pub struct Error {
    /// The error kind
    kind: ErrorKind,
    /// The human readable error description
    description: String,
    /// The error backtrace
    backtrace: Backtrace,
}
impl Error {
    /// Creates a new error description
    pub fn new<Description>(err: ErrorKind, description: Description) -> Self
    where
        Description: ToString,
    {
        let description = description.to_string();
        let backtrace = Backtrace::capture();
        Self { kind: err, description, backtrace }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // Print the error
        write!(f, "{}: {}", self.kind, self.description)?;

        // Print the backtrace
        if self.backtrace.status() == BacktraceStatus::Captured {
            writeln!(f)?;
            writeln!(f, "Backtrace:")?;
            write!(f, "{}", self.backtrace)?;
        }
        Ok(())
    }
}
impl std::error::Error for Error {
    // No members to override
}
impl From<std::io::Error> for Error {
    fn from(underlying: std::io::Error) -> Self {
        Self::new(ErrorKind::InOutError, underlying)
    }
}
