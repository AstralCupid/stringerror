// Error is just a wrapper around a string.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Error(pub String);

// Define the core methods for error.
impl Error {
    pub fn add_context(&self, context: &str) -> Error {
        return format!("{}: {}", context, self).into();
    }
}

// Implement the Display code for the error.
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// Implement the std::error:Error function for the Error type. It's okay for it to be empty.
impl std::error::Error for Error {}

// Implement 'from' so we can create errors automatically from strings.
impl<T: Into<String>> From<T> for Error {
    fn from(s: T) -> Self { Error(s.into()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ensure that Display is working correctly.
    #[test]
    fn error_from_str() {
        let error: Error = "this is a test error".into();
        let err_display = format!("{}", error);
        assert_eq!(err_display, "this is a test error");
    }

    // Ensure that add_context is working correctly.
    #[test]
    fn add_context() {
        let err: Error = "this is a test error".into();
        let with_context = err.add_context("some context");
        let with_context_display = format!("{}", with_context);
        assert_eq!(with_context_display, "some context: this is a test error");
    }
}
