// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for Parquet.</p>
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
pub struct ParquetInput {}
impl ParquetInput {
    /// Creates a new builder-style object to manufacture [`ParquetInput`](crate::types::ParquetInput).
    pub fn builder() -> crate::types::builders::ParquetInputBuilder {
        crate::types::builders::ParquetInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ParquetInput;
/// A builder for [`ParquetInput`](crate::types::ParquetInput).
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
pub struct ParquetInputBuilder {}
impl ParquetInputBuilder {
    /// Consumes the builder and constructs a [`ParquetInput`](crate::types::ParquetInput).
    pub fn build(self) -> crate::types::ParquetInput {
        crate::types::ParquetInput {}
    }
}
