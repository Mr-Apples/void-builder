use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::*;

/// Handles a Result<T, VoidBuilderError>. If it is a Ok(T) return Some(T), otherwise print the error message and return None
pub fn handle<T, G: Error>(result: Result<T, G>) -> Result<T, VoidBuilderError> {
    return match result {
        Ok(value) => Ok(value),
        Err(e) => {
            eprintln!("Void-Builder: {}", e.to_string());
            return Err(VoidBuilderError::new(e.to_string()));
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
    pub fn print(&self) {
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
