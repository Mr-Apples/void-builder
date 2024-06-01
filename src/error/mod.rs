use std::*;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
/// Struct to contain errors encountered by Void Builder
pub struct VoidBuilderError {
    message: String
}

impl VoidBuilderError {
    /// Creates a new VoidBuilderError from the given error
    fn new(message: String) -> VoidBuilderError {
        VoidBuilderError {
            message
        }
    }
}

impl Display for VoidBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Implement std::error::Error on VoidBuilderError
impl error::Error for VoidBuilderError {}

/// Implements From<git2::Error> on VoidBuilderError
impl From<git2::Error> for VoidBuilderError {
    /// Creates a VoidBuilderError from a git2::Error
    fn from(value: git2::Error) -> Self {
        return VoidBuilderError::new(value.message().to_string());
    }
}

/// Implements From<std::io::Error> on VoidBuilderError
impl From<io::Error> for VoidBuilderError {
    /// Create a VoidBuilderError from a std::io::Error
    fn from(value: io::Error) -> Self {
        return VoidBuilderError::new(value.to_string());
    }
}

/// Implements From<url::ParseError> on VoidBuilderError
impl From<url::ParseError> for VoidBuilderError {
    /// Create a VoidBuilderError from a url::ParseError
    fn from(value: url::ParseError) -> Self {
        return VoidBuilderError::new(value.to_string());
    }
}

impl From<daemonize::Error> for VoidBuilderError {
    fn from(value: daemonize::Error) -> Self {
        return VoidBuilderError::new(value.to_string())
    }
}