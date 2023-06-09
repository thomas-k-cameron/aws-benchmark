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
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DisableKeyRotationOutput {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for DisableKeyRotationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DisableKeyRotationOutput {
    /// Creates a new builder-style object to manufacture [`DisableKeyRotationOutput`](crate::operation::disable_key_rotation::DisableKeyRotationOutput).
    pub fn builder(
    ) -> crate::operation::disable_key_rotation::builders::DisableKeyRotationOutputBuilder {
        crate::operation::disable_key_rotation::builders::DisableKeyRotationOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::disable_key_rotation::DisableKeyRotationOutput;
/// A builder for [`DisableKeyRotationOutput`](crate::operation::disable_key_rotation::DisableKeyRotationOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct DisableKeyRotationOutputBuilder {
    _request_id: Option<String>,
}
impl DisableKeyRotationOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DisableKeyRotationOutput`](crate::operation::disable_key_rotation::DisableKeyRotationOutput).
    pub fn build(self) -> crate::operation::disable_key_rotation::DisableKeyRotationOutput {
        crate::operation::disable_key_rotation::DisableKeyRotationOutput {
            _request_id: self._request_id,
        }
    }
}
