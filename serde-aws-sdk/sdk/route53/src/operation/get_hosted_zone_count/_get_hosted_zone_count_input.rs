// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A request to retrieve a count of all the hosted zones that are associated with the current Amazon Web Services account.</p>
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
pub struct GetHostedZoneCountInput {}
impl GetHostedZoneCountInput {
    /// Creates a new builder-style object to manufacture [`GetHostedZoneCountInput`](crate::operation::get_hosted_zone_count::GetHostedZoneCountInput).
    pub fn builder(
    ) -> crate::operation::get_hosted_zone_count::builders::GetHostedZoneCountInputBuilder {
        crate::operation::get_hosted_zone_count::builders::GetHostedZoneCountInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_hosted_zone_count::GetHostedZoneCountInput;
/// A builder for [`GetHostedZoneCountInput`](crate::operation::get_hosted_zone_count::GetHostedZoneCountInput).
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
pub struct GetHostedZoneCountInputBuilder {}
impl GetHostedZoneCountInputBuilder {
    /// Consumes the builder and constructs a [`GetHostedZoneCountInput`](crate::operation::get_hosted_zone_count::GetHostedZoneCountInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_hosted_zone_count::GetHostedZoneCountInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::get_hosted_zone_count::GetHostedZoneCountInput {})
    }
}