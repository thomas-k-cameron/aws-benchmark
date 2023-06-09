// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A message that indicates the request is complete and no more messages will be sent. You should not assume that the request is complete until the client receives an <code>EndEvent</code>.</p>
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
pub struct EndEvent {}
impl EndEvent {
    /// Creates a new builder-style object to manufacture [`EndEvent`](crate::types::EndEvent).
    pub fn builder() -> crate::types::builders::EndEventBuilder {
        crate::types::builders::EndEventBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::EndEvent;
/// A builder for [`EndEvent`](crate::types::EndEvent).
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
pub struct EndEventBuilder {}
impl EndEventBuilder {
    /// Consumes the builder and constructs a [`EndEvent`](crate::types::EndEvent).
    pub fn build(self) -> crate::types::EndEvent {
        crate::types::EndEvent {}
    }
}
