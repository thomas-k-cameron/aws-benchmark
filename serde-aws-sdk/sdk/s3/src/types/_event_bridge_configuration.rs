// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container for specifying the configuration for Amazon EventBridge.</p>
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
pub struct EventBridgeConfiguration {}
impl EventBridgeConfiguration {
    /// Creates a new builder-style object to manufacture [`EventBridgeConfiguration`](crate::types::EventBridgeConfiguration).
    pub fn builder() -> crate::types::builders::EventBridgeConfigurationBuilder {
        crate::types::builders::EventBridgeConfigurationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::EventBridgeConfiguration;
/// A builder for [`EventBridgeConfiguration`](crate::types::EventBridgeConfiguration).
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
pub struct EventBridgeConfigurationBuilder {}
impl EventBridgeConfigurationBuilder {
    /// Consumes the builder and constructs a [`EventBridgeConfiguration`](crate::types::EventBridgeConfiguration).
    pub fn build(self) -> crate::types::EventBridgeConfiguration {
        crate::types::EventBridgeConfiguration {}
    }
}
