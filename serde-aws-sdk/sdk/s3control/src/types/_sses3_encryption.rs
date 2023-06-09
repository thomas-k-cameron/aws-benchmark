// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Configuration for the use of SSE-S3 to encrypt generated manifest objects.</p>
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
pub struct Sses3Encryption {}
impl Sses3Encryption {
    /// Creates a new builder-style object to manufacture [`Sses3Encryption`](crate::types::Sses3Encryption).
    pub fn builder() -> crate::types::builders::Sses3EncryptionBuilder {
        crate::types::builders::Sses3EncryptionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Sses3Encryption;
/// A builder for [`Sses3Encryption`](crate::types::Sses3Encryption).
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
pub struct Sses3EncryptionBuilder {}
impl Sses3EncryptionBuilder {
    /// Consumes the builder and constructs a [`Sses3Encryption`](crate::types::Sses3Encryption).
    pub fn build(self) -> crate::types::Sses3Encryption {
        crate::types::Sses3Encryption {}
    }
}
