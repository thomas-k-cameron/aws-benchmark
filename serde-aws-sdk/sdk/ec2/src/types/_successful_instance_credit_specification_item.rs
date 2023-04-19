// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the burstable performance instance whose credit option for CPU usage was successfully modified.</p>
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
pub struct SuccessfulInstanceCreditSpecificationItem {
    /// <p>The ID of the instance.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
}
impl SuccessfulInstanceCreditSpecificationItem {
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
}
impl SuccessfulInstanceCreditSpecificationItem {
    /// Creates a new builder-style object to manufacture [`SuccessfulInstanceCreditSpecificationItem`](crate::types::SuccessfulInstanceCreditSpecificationItem).
    pub fn builder() -> crate::types::builders::SuccessfulInstanceCreditSpecificationItemBuilder {
        crate::types::builders::SuccessfulInstanceCreditSpecificationItemBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::SuccessfulInstanceCreditSpecificationItem;
/// A builder for [`SuccessfulInstanceCreditSpecificationItem`](crate::types::SuccessfulInstanceCreditSpecificationItem).
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
pub struct SuccessfulInstanceCreditSpecificationItemBuilder {
    pub(crate) instance_id: std::option::Option<std::string::String>,
}
impl SuccessfulInstanceCreditSpecificationItemBuilder {
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_id = Some(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// Consumes the builder and constructs a [`SuccessfulInstanceCreditSpecificationItem`](crate::types::SuccessfulInstanceCreditSpecificationItem).
    pub fn build(self) -> crate::types::SuccessfulInstanceCreditSpecificationItem {
        crate::types::SuccessfulInstanceCreditSpecificationItem {
            instance_id: self.instance_id,
        }
    }
}