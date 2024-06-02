use config::ConfigError;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::*;

/// Handles a Result<T, VoidBuilderError>. If it is a Ok(T) return Some(T), otherwise print the error message and return None
pub fn handle<T>(result: Result<T, VoidBuilderError>) -> Option<T> {
    return match result {
        Ok(value) => Some(value),
        Err(e) => {
            eprintln!("Void-Builder: {}", e.message);
            None
        }
    };
}

/// Handles a Result<T, ConfigError>. If it is a Ok(T) return Some(T), otherwise print the error message and return None
pub fn handle_config_error<T>(result: Result<T, ConfigError>) -> Option<T> {
    return match result {
        Ok(value) => Some(value),
        Err(e) => {
            eprintln!("Void-Builder: {}", e.to_string());
            None
        }
    };
}

#[derive(Debug)]
/// Struct to contain errors encountered by Void Builder
pub struct VoidBuilderError {
    pub message: String,
}

impl VoidBuilderError {
    /// Creates a new VoidBuilderError from the given error
    pub fn new(message: String) -> VoidBuilderError {
        VoidBuilderError { message }
    }

    /// Prints the error
    pub fn print(self) {
        eprintln!("{}", self.message);
    }
}

impl PartialEq for VoidBuilderError {
    fn eq(&self, other: &Self) -> bool {
        return if *self.message == *other.message {
            true
        } else {
            false
        };
    }

    fn ne(&self, other: &Self) -> bool {
        return if *self.message == *other.message {
            true
        } else {
            false
        };
    }
}

impl Display for VoidBuilderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Implement std::error::Error on VoidBuilderError
impl Error for VoidBuilderError {}

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

/// Implements From<daemonize::Error> on VoidBuilderError
impl From<daemonize::Error> for VoidBuilderError {
    /// Create a VoidBuilderError from a daemonize::Error
    fn from(value: daemonize::Error) -> Self {
        return VoidBuilderError::new(value.to_string());
    }
}

/// Implements From<config::error::Result> on VoidBuilderError
impl From<ConfigError> for VoidBuilderError {
    fn from(value: ConfigError) -> Self {
        return VoidBuilderError::new(value.to_string());
    }
}
