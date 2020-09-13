use thiserror::Error;

/// enumerates all possible errors returned by this library
#[derive(Error, Debug)]
pub enum Error {

    // (TBD more specific)
    #[error("generic error")]
    Generic,

    #[error("{0}")]
    OutOfBounds(String),

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

}


#[cfg(test)]
mod tests_error {
    use super::Error;

    fn assert<T: std::fmt::Display>(expected: &str, value: T) {
        assert_eq!(expected, value.to_string());
    }

    #[test]
    fn test_error() {
        assert("generic error", Error::Generic);
    }

}
