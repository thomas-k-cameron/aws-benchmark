// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p></p>
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
pub struct Ssekms {
    /// <p>A container for the ARN of the SSE-KMS encryption. This property is read-only and follows the following format: <code> arn:aws:kms:<i>us-east-1</i>:<i>example-account-id</i>:key/<i>example-9a73-4afc-8d29-8f5900cef44e</i> </code> </p>
    #[doc(hidden)]
    pub key_id: std::option::Option<std::string::String>,
}
impl Ssekms {
    /// <p>A container for the ARN of the SSE-KMS encryption. This property is read-only and follows the following format: <code> arn:aws:kms:<i>us-east-1</i>:<i>example-account-id</i>:key/<i>example-9a73-4afc-8d29-8f5900cef44e</i> </code> </p>
    pub fn key_id(&self) -> std::option::Option<&str> {
        self.key_id.as_deref()
    }
}
impl Ssekms {
    /// Creates a new builder-style object to manufacture [`Ssekms`](crate::types::Ssekms).
    pub fn builder() -> crate::types::builders::SsekmsBuilder {
        crate::types::builders::SsekmsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Ssekms;
/// A builder for [`Ssekms`](crate::types::Ssekms).
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
pub struct SsekmsBuilder {
    pub(crate) key_id: std::option::Option<std::string::String>,
}
impl SsekmsBuilder {
    /// <p>A container for the ARN of the SSE-KMS encryption. This property is read-only and follows the following format: <code> arn:aws:kms:<i>us-east-1</i>:<i>example-account-id</i>:key/<i>example-9a73-4afc-8d29-8f5900cef44e</i> </code> </p>
    pub fn key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.key_id = Some(input.into());
        self
    }
    /// <p>A container for the ARN of the SSE-KMS encryption. This property is read-only and follows the following format: <code> arn:aws:kms:<i>us-east-1</i>:<i>example-account-id</i>:key/<i>example-9a73-4afc-8d29-8f5900cef44e</i> </code> </p>
    pub fn set_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.key_id = input;
        self
    }
    /// Consumes the builder and constructs a [`Ssekms`](crate::types::Ssekms).
    pub fn build(self) -> crate::types::Ssekms {
        crate::types::Ssekms {
            key_id: self.key_id,
        }
    }
}