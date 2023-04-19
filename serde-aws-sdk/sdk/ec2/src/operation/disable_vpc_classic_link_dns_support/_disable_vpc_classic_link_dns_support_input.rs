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
pub struct DisableVpcClassicLinkDnsSupportInput {
    /// <p>The ID of the VPC.</p>
    #[doc(hidden)]
    pub vpc_id: std::option::Option<std::string::String>,
}
impl DisableVpcClassicLinkDnsSupportInput {
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(&self) -> std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
}
impl DisableVpcClassicLinkDnsSupportInput {
    /// Creates a new builder-style object to manufacture [`DisableVpcClassicLinkDnsSupportInput`](crate::operation::disable_vpc_classic_link_dns_support::DisableVpcClassicLinkDnsSupportInput).
    pub fn builder() -> crate::operation::disable_vpc_classic_link_dns_support::builders::DisableVpcClassicLinkDnsSupportInputBuilder{
        crate::operation::disable_vpc_classic_link_dns_support::builders::DisableVpcClassicLinkDnsSupportInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::disable_vpc_classic_link_dns_support::DisableVpcClassicLinkDnsSupportInput;
/// A builder for [`DisableVpcClassicLinkDnsSupportInput`](crate::operation::disable_vpc_classic_link_dns_support::DisableVpcClassicLinkDnsSupportInput).
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
pub struct DisableVpcClassicLinkDnsSupportInputBuilder {
    pub(crate) vpc_id: std::option::Option<std::string::String>,
}
impl DisableVpcClassicLinkDnsSupportInputBuilder {
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.vpc_id = Some(input.into());
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn set_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DisableVpcClassicLinkDnsSupportInput`](crate::operation::disable_vpc_classic_link_dns_support::DisableVpcClassicLinkDnsSupportInput).
    pub fn build(self) -> Result<crate::operation::disable_vpc_classic_link_dns_support::DisableVpcClassicLinkDnsSupportInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::disable_vpc_classic_link_dns_support::DisableVpcClassicLinkDnsSupportInput {
                vpc_id: self.vpc_id
                ,
            }
        )
    }
}