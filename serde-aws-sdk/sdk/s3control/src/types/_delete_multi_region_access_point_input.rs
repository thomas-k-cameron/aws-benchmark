// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container for the information associated with a <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_DeleteMultiRegionAccessPoint.html">DeleteMultiRegionAccessPoint</a> request.</p>
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
pub struct DeleteMultiRegionAccessPointInput {
    /// <p>The name of the Multi-Region Access Point associated with this request.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
}
impl DeleteMultiRegionAccessPointInput {
    /// <p>The name of the Multi-Region Access Point associated with this request.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl DeleteMultiRegionAccessPointInput {
    /// Creates a new builder-style object to manufacture [`DeleteMultiRegionAccessPointInput`](crate::types::DeleteMultiRegionAccessPointInput).
    pub fn builder() -> crate::types::builders::DeleteMultiRegionAccessPointInputBuilder {
        crate::types::builders::DeleteMultiRegionAccessPointInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::DeleteMultiRegionAccessPointInput;
/// A builder for [`DeleteMultiRegionAccessPointInput`](crate::types::DeleteMultiRegionAccessPointInput).
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
pub struct DeleteMultiRegionAccessPointInputBuilder {
    pub(crate) name: std::option::Option<std::string::String>,
}
impl DeleteMultiRegionAccessPointInputBuilder {
    /// <p>The name of the Multi-Region Access Point associated with this request.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of the Multi-Region Access Point associated with this request.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteMultiRegionAccessPointInput`](crate::types::DeleteMultiRegionAccessPointInput).
    pub fn build(self) -> crate::types::DeleteMultiRegionAccessPointInput {
        crate::types::DeleteMultiRegionAccessPointInput { name: self.name }
    }
}