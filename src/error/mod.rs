use std::*;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
/// Struct to contain errors encountered by Void Builder
pub struct VoidBuilderError<T: Debug + 'static> {
    source: T,
    message: String
}

impl<T: Debug> VoidBuilderError<T> {
    /// Creates a new VoidBuilderError from the given error
    fn new(message: String, source: T) -> VoidBuilderError<T> {
        VoidBuilderError {
            source,
            message
        }
    }
}

impl<T: Debug> Display for VoidBuilderError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Implement std::error::Error on VoidBuilderError
impl<T: Debug + error::Error> error::Error for VoidBuilderError<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        return Some(&self.source);
    }
}

/// Implements From<git2::Error> on VoidBuilderError
impl From<git2::Error> for VoidBuilderError<git2::Error> {
    /// Creates a VoidBuilderError from a git2::Error
    fn from(value: git2::Error) -> Self {
        return VoidBuilderError::new(value.message().to_string(), value);
    }
}

/// Implements From<std::io::Error> on VoidBuilderError
impl From<io::Error> for VoidBuilderError<io::Error> {
    /// Create a VoidBuilderError from a std::io::Error
    fn from(value: io::Error) -> Self {
        return VoidBuilderError::new(value.to_string(), value);
    }
}

/// Implements From<url::ParseError> on VoidBuilderError
impl From<url::ParseError> for VoidBuilderError<url::ParseError> {
    /// Create a VoidBuilderError from a url::ParseError
    fn from(value: url::ParseError) -> Self {
        return VoidBuilderError::new(value.to_string(), value);
    }
}

impl From<daemonize::Error> for VoidBuilderError<daemonize::Error> {
    fn from(value: daemonize::Error) -> Self {
        return VoidBuilderError::new(value.to_string(), value)
    }
}