// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Any error returned when the runtime version information for the function could not be retrieved.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RuntimeVersionError {
    /// <p>The error code.</p>
    #[doc(hidden)]
    pub error_code: std::option::Option<std::string::String>,
    /// <p>The error message.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl RuntimeVersionError {
    /// <p>The error code.</p>
    pub fn error_code(&self) -> std::option::Option<&str> {
        self.error_code.as_deref()
    }
    /// <p>The error message.</p>
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Debug for RuntimeVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RuntimeVersionError");
        formatter.field("error_code", &self.error_code);
        formatter.field("message", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl RuntimeVersionError {
    /// Creates a new builder-style object to manufacture [`RuntimeVersionError`](crate::types::RuntimeVersionError).
    pub fn builder() -> crate::types::builders::RuntimeVersionErrorBuilder {
        crate::types::builders::RuntimeVersionErrorBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::RuntimeVersionError;
/// A builder for [`RuntimeVersionError`](crate::types::RuntimeVersionError).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct RuntimeVersionErrorBuilder {
    pub(crate) error_code: std::option::Option<std::string::String>,
    pub(crate) message: std::option::Option<std::string::String>,
}
impl RuntimeVersionErrorBuilder {
    /// <p>The error code.</p>
    pub fn error_code(mut self, input: impl Into<std::string::String>) -> Self {
        self.error_code = Some(input.into());
        self
    }
    /// <p>The error code.</p>
    pub fn set_error_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.error_code = input;
        self
    }
    /// <p>The error message.</p>
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    /// <p>The error message.</p>
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Consumes the builder and constructs a [`RuntimeVersionError`](crate::types::RuntimeVersionError).
    pub fn build(self) -> crate::types::RuntimeVersionError {
        crate::types::RuntimeVersionError {
            error_code: self.error_code,
            message: self.message,
        }
    }
}
impl std::fmt::Debug for RuntimeVersionErrorBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RuntimeVersionErrorBuilder");
        formatter.field("error_code", &self.error_code);
        formatter.field("message", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}