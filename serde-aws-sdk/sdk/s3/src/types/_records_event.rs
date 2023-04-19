// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The container for the records event.</p>
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
pub struct RecordsEvent {
    /// <p>The byte array of partial, one or more result records.</p>
    #[doc(hidden)]
    pub payload: std::option::Option<aws_smithy_types::Blob>,
}
impl RecordsEvent {
    /// <p>The byte array of partial, one or more result records.</p>
    pub fn payload(&self) -> std::option::Option<&aws_smithy_types::Blob> {
        self.payload.as_ref()
    }
}
impl RecordsEvent {
    /// Creates a new builder-style object to manufacture [`RecordsEvent`](crate::types::RecordsEvent).
    pub fn builder() -> crate::types::builders::RecordsEventBuilder {
        crate::types::builders::RecordsEventBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::RecordsEvent;
/// A builder for [`RecordsEvent`](crate::types::RecordsEvent).
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
pub struct RecordsEventBuilder {
    pub(crate) payload: std::option::Option<aws_smithy_types::Blob>,
}
impl RecordsEventBuilder {
    /// <p>The byte array of partial, one or more result records.</p>
    pub fn payload(mut self, input: aws_smithy_types::Blob) -> Self {
        self.payload = Some(input);
        self
    }
    /// <p>The byte array of partial, one or more result records.</p>
    pub fn set_payload(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
        self.payload = input;
        self
    }
    /// Consumes the builder and constructs a [`RecordsEvent`](crate::types::RecordsEvent).
    pub fn build(self) -> crate::types::RecordsEvent {
        crate::types::RecordsEvent {
            payload: self.payload,
        }
    }
}