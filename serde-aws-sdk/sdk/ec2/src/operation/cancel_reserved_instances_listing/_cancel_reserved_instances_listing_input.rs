// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for CancelReservedInstancesListing.</p>
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
pub struct CancelReservedInstancesListingInput {
    /// <p>The ID of the Reserved Instance listing.</p>
    #[doc(hidden)]
    pub reserved_instances_listing_id: std::option::Option<std::string::String>,
}
impl CancelReservedInstancesListingInput {
    /// <p>The ID of the Reserved Instance listing.</p>
    pub fn reserved_instances_listing_id(&self) -> std::option::Option<&str> {
        self.reserved_instances_listing_id.as_deref()
    }
}
impl CancelReservedInstancesListingInput {
    /// Creates a new builder-style object to manufacture [`CancelReservedInstancesListingInput`](crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingInput).
    pub fn builder() -> crate::operation::cancel_reserved_instances_listing::builders::CancelReservedInstancesListingInputBuilder{
        crate::operation::cancel_reserved_instances_listing::builders::CancelReservedInstancesListingInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingInput;
/// A builder for [`CancelReservedInstancesListingInput`](crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingInput).
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
pub struct CancelReservedInstancesListingInputBuilder {
    pub(crate) reserved_instances_listing_id: std::option::Option<std::string::String>,
}
impl CancelReservedInstancesListingInputBuilder {
    /// <p>The ID of the Reserved Instance listing.</p>
    pub fn reserved_instances_listing_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.reserved_instances_listing_id = Some(input.into());
        self
    }
    /// <p>The ID of the Reserved Instance listing.</p>
    pub fn set_reserved_instances_listing_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.reserved_instances_listing_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CancelReservedInstancesListingInput`](crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingInput {
                reserved_instances_listing_id: self.reserved_instances_listing_id
                ,
            }
        )
    }
}