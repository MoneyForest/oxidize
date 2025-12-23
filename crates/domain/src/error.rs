use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    Internal,
}

#[derive(Error, Debug)]
#[error("{message}")]
pub struct DomainError {
    pub code: &'static str,
    pub category: ErrorCategory,
    pub message: String,
}

impl DomainError {
    pub fn new(code: &'static str, category: ErrorCategory, message: impl Into<String>) -> Self {
        Self {
            code,
            category,
            message: message.into(),
        }
    }

    pub fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(code, ErrorCategory::BadRequest, message)
    }

    pub fn not_found(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(code, ErrorCategory::NotFound, message)
    }

    pub fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(code, ErrorCategory::Internal, message)
    }
}

pub type Result<T> = std::result::Result<T, DomainError>;

pub mod errors {
    use super::*;

    pub fn internal() -> DomainError {
        DomainError::internal("E100001", "Internal error")
    }

    pub fn invalid_argument() -> DomainError {
        DomainError::bad_request("E100002", "Invalid argument")
    }

    pub fn tenant_not_found() -> DomainError {
        DomainError::not_found("E200101", "Tenant not found")
    }

    pub fn staff_not_found() -> DomainError {
        DomainError::not_found("E200201", "Staff not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = errors::tenant_not_found();
        assert_eq!(err.code, "E200101");
        assert_eq!(err.category, ErrorCategory::NotFound);
    }

    #[test]
    fn test_result_with_question_mark() {
        fn may_fail(should_fail: bool) -> Result<String> {
            if should_fail {
                return Err(errors::internal());
            }
            Ok("success".to_string())
        }

        fn caller() -> Result<String> {
            let result = may_fail(false)?;
            Ok(result)
        }

        assert!(caller().is_ok());
    }
}
