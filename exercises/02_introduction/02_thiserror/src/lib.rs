/// Use `thiserror` to derive `Error` and `Display` for this enum.
///
/// Requirements:
/// - `NotFound` should display as "resource not found"
/// - `InvalidInput` should display as "invalid input: {msg}" where `msg` is the field value
/// - `Internal` should display as "internal error" and its `source()` should return the inner error
/// - An `std::io::Error` should be convertible into `AppError::Internal` via `From`
///
/// Hint: you need `#[derive(...)]` and `#[error("...")]` attributes on each variant.
/// Check out the `thiserror` documentation for `#[from]` and `#[source]`:
/// https://docs.rs/thiserror
// TODO: add the right derive and attributes on the enum and its variants
#[derive(Debug)]
pub enum AppError {
    NotFound,
    InvalidInput { msg: String },
    Internal(std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert!(matches!(err, AppError::Internal(_)));
    }
}
