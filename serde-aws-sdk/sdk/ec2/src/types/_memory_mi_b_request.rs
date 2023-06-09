// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The minimum and maximum amount of memory, in MiB.</p>
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
pub struct MemoryMiBRequest {
    /// <p>The minimum amount of memory, in MiB. To specify no minimum limit, specify <code>0</code>.</p>
    #[doc(hidden)]
    pub min: std::option::Option<i32>,
    /// <p>The maximum amount of memory, in MiB. To specify no maximum limit, omit this parameter.</p>
    #[doc(hidden)]
    pub max: std::option::Option<i32>,
}
impl MemoryMiBRequest {
    /// <p>The minimum amount of memory, in MiB. To specify no minimum limit, specify <code>0</code>.</p>
    pub fn min(&self) -> std::option::Option<i32> {
        self.min
    }
    /// <p>The maximum amount of memory, in MiB. To specify no maximum limit, omit this parameter.</p>
    pub fn max(&self) -> std::option::Option<i32> {
        self.max
    }
}
impl MemoryMiBRequest {
    /// Creates a new builder-style object to manufacture [`MemoryMiBRequest`](crate::types::MemoryMiBRequest).
    pub fn builder() -> crate::types::builders::MemoryMiBRequestBuilder {
        crate::types::builders::MemoryMiBRequestBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::MemoryMiBRequest;
/// A builder for [`MemoryMiBRequest`](crate::types::MemoryMiBRequest).
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
pub struct MemoryMiBRequestBuilder {
    pub(crate) min: std::option::Option<i32>,
    pub(crate) max: std::option::Option<i32>,
}
impl MemoryMiBRequestBuilder {
    /// <p>The minimum amount of memory, in MiB. To specify no minimum limit, specify <code>0</code>.</p>
    pub fn min(mut self, input: i32) -> Self {
        self.min = Some(input);
        self
    }
    /// <p>The minimum amount of memory, in MiB. To specify no minimum limit, specify <code>0</code>.</p>
    pub fn set_min(mut self, input: std::option::Option<i32>) -> Self {
        self.min = input;
        self
    }
    /// <p>The maximum amount of memory, in MiB. To specify no maximum limit, omit this parameter.</p>
    pub fn max(mut self, input: i32) -> Self {
        self.max = Some(input);
        self
    }
    /// <p>The maximum amount of memory, in MiB. To specify no maximum limit, omit this parameter.</p>
    pub fn set_max(mut self, input: std::option::Option<i32>) -> Self {
        self.max = input;
        self
    }
    /// Consumes the builder and constructs a [`MemoryMiBRequest`](crate::types::MemoryMiBRequest).
    pub fn build(self) -> crate::types::MemoryMiBRequest {
        crate::types::MemoryMiBRequest {
            min: self.min,
            max: self.max,
        }
    }
}
