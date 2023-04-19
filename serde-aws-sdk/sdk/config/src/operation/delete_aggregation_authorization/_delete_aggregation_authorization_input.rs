// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct DeleteAggregationAuthorizationInput {
    /// <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    #[doc(hidden)]
    pub authorized_account_id: std::option::Option<std::string::String>,
    /// <p>The region authorized to collect aggregated data.</p>
    #[doc(hidden)]
    pub authorized_aws_region: std::option::Option<std::string::String>,
}
impl DeleteAggregationAuthorizationInput {
    /// <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    pub fn authorized_account_id(&self) -> std::option::Option<&str> {
        self.authorized_account_id.as_deref()
    }
    /// <p>The region authorized to collect aggregated data.</p>
    pub fn authorized_aws_region(&self) -> std::option::Option<&str> {
        self.authorized_aws_region.as_deref()
    }
}
impl DeleteAggregationAuthorizationInput {
    /// Creates a new builder-style object to manufacture [`DeleteAggregationAuthorizationInput`](crate::operation::delete_aggregation_authorization::DeleteAggregationAuthorizationInput).
    pub fn builder() -> crate::operation::delete_aggregation_authorization::builders::DeleteAggregationAuthorizationInputBuilder{
        crate::operation::delete_aggregation_authorization::builders::DeleteAggregationAuthorizationInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::delete_aggregation_authorization::DeleteAggregationAuthorizationInput;
/// A builder for [`DeleteAggregationAuthorizationInput`](crate::operation::delete_aggregation_authorization::DeleteAggregationAuthorizationInput).
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
pub struct DeleteAggregationAuthorizationInputBuilder {
    pub(crate) authorized_account_id: std::option::Option<std::string::String>,
    pub(crate) authorized_aws_region: std::option::Option<std::string::String>,
}
impl DeleteAggregationAuthorizationInputBuilder {
    /// <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    pub fn authorized_account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.authorized_account_id = Some(input.into());
        self
    }
    /// <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    pub fn set_authorized_account_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.authorized_account_id = input;
        self
    }
    /// <p>The region authorized to collect aggregated data.</p>
    pub fn authorized_aws_region(mut self, input: impl Into<std::string::String>) -> Self {
        self.authorized_aws_region = Some(input.into());
        self
    }
    /// <p>The region authorized to collect aggregated data.</p>
    pub fn set_authorized_aws_region(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.authorized_aws_region = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteAggregationAuthorizationInput`](crate::operation::delete_aggregation_authorization::DeleteAggregationAuthorizationInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_aggregation_authorization::DeleteAggregationAuthorizationInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_aggregation_authorization::DeleteAggregationAuthorizationInput {
                authorized_account_id: self.authorized_account_id
                ,
                authorized_aws_region: self.authorized_aws_region
                ,
            }
        )
    }
}