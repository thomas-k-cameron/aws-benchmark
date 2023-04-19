// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct GenerateRandomOutput {
    /// <p>The random byte string. When you use the HTTP API or the Amazon Web Services CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[doc(hidden)]
    pub plaintext: std::option::Option<aws_smithy_types::Blob>,
    _request_id: Option<String>,
}
impl GenerateRandomOutput {
    /// <p>The random byte string. When you use the HTTP API or the Amazon Web Services CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    pub fn plaintext(&self) -> std::option::Option<&aws_smithy_types::Blob> {
        self.plaintext.as_ref()
    }
}
impl std::fmt::Debug for GenerateRandomOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GenerateRandomOutput");
        formatter.field("plaintext", &"*** Sensitive Data Redacted ***");
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl aws_http::request_id::RequestId for GenerateRandomOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GenerateRandomOutput {
    /// Creates a new builder-style object to manufacture [`GenerateRandomOutput`](crate::operation::generate_random::GenerateRandomOutput).
    pub fn builder() -> crate::operation::generate_random::builders::GenerateRandomOutputBuilder {
        crate::operation::generate_random::builders::GenerateRandomOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::generate_random::GenerateRandomOutput;
/// A builder for [`GenerateRandomOutput`](crate::operation::generate_random::GenerateRandomOutput).
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
pub struct GenerateRandomOutputBuilder {
    pub(crate) plaintext: std::option::Option<aws_smithy_types::Blob>,
    _request_id: Option<String>,
}
impl GenerateRandomOutputBuilder {
    /// <p>The random byte string. When you use the HTTP API or the Amazon Web Services CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    pub fn plaintext(mut self, input: aws_smithy_types::Blob) -> Self {
        self.plaintext = Some(input);
        self
    }
    /// <p>The random byte string. When you use the HTTP API or the Amazon Web Services CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    pub fn set_plaintext(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
        self.plaintext = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GenerateRandomOutput`](crate::operation::generate_random::GenerateRandomOutput).
    pub fn build(self) -> crate::operation::generate_random::GenerateRandomOutput {
        crate::operation::generate_random::GenerateRandomOutput {
            plaintext: self.plaintext,
            _request_id: self._request_id,
        }
    }
}
impl std::fmt::Debug for GenerateRandomOutputBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GenerateRandomOutputBuilder");
        formatter.field("plaintext", &"*** Sensitive Data Redacted ***");
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}