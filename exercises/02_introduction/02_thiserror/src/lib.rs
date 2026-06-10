/// Use `thiserror` to derive `Error` and `Display` for this enum.
///
/// The tests below spell out exactly what's expected of each variant — read them to
/// figure out the `Display` messages, the error source, and the `From` conversion you need.
/// The `thiserror` documentation covers the helper attributes that make this possible:
/// https://docs.rs/thiserror
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("resource not found")]
    NotFound,
    #[error("invalid input: {msg}")]
    InvalidInput { msg: String },
    #[error("internal error")]
    Internal(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::assert_matches;
    use std::error::Error;

    #[test]
    fn not_found_display() {
        let err = AppError::NotFound;
        assert_eq!(err.to_string(), "resource not found");
    }

    #[test]
    fn invalid_input_display() {
        let err = AppError::InvalidInput {
            msg: "bad value".to_string(),
        };
        assert_eq!(err.to_string(), "invalid input: bad value");
    }

    #[test]
    fn internal_display() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "disk full");
        let err = AppError::Internal(io_err);
        assert_eq!(err.to_string(), "internal error");
    }

    #[test]
    fn internal_source() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "disk full");
        let err = AppError::Internal(io_err);
        assert!(err.source().is_some());
    }

    #[test]
    fn from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "oops");
        let err: AppError = io_err.into();
        assert_matches!(err, AppError::Internal(_));
    }
}
