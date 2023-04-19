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
pub struct Sses3 {}
impl Sses3 {
    /// Creates a new builder-style object to manufacture [`Sses3`](crate::types::Sses3).
    pub fn builder() -> crate::types::builders::Sses3Builder {
        crate::types::builders::Sses3Builder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Sses3;
/// A builder for [`Sses3`](crate::types::Sses3).
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
pub struct Sses3Builder {}
impl Sses3Builder {
    /// Consumes the builder and constructs a [`Sses3`](crate::types::Sses3).
    pub fn build(self) -> crate::types::Sses3 {
        crate::types::Sses3 {}
    }
}