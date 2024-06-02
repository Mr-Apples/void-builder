#[cfg(test)]
/// A module containing all the tests for the VoidBuilderError type
mod error_tests {
    use crate::error::*;

    #[test]
    // Test instantiating an error
    fn test_custom_error_instantiate() {
        assert_eq!(
            VoidBuilderError::new("Error".to_string()),
            VoidBuilderError {
            message: "Error".to_string()
        })
    }
    
    #[test]
    // Test handling an Ok() variant
    fn test_handle_result_ok() {
        let result = Ok(());
        
        assert_eq!(
            Some(()),
            handle(result)
        )
    }
    
    #[test]
    // Test handling an Err() variant
    fn test_handle_result_err() {
        let result: Result<(), VoidBuilderError> = Err(VoidBuilderError::new("Error".to_string()));
        
        assert_eq!(
            None,
            handle(result)
        )
    }
}

#[cfg(test)]
/// A module containing all the tests for the git_helper module
mod git_helper_tests {
    
}