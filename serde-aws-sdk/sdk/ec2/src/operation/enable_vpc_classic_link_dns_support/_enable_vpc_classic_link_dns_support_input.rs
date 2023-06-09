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
pub struct EnableVpcClassicLinkDnsSupportInput {
    /// <p>The ID of the VPC.</p>
    #[doc(hidden)]
    pub vpc_id: std::option::Option<std::string::String>,
}
impl EnableVpcClassicLinkDnsSupportInput {
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(&self) -> std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
}
impl EnableVpcClassicLinkDnsSupportInput {
    /// Creates a new builder-style object to manufacture [`EnableVpcClassicLinkDnsSupportInput`](crate::operation::enable_vpc_classic_link_dns_support::EnableVpcClassicLinkDnsSupportInput).
    pub fn builder() -> crate::operation::enable_vpc_classic_link_dns_support::builders::EnableVpcClassicLinkDnsSupportInputBuilder{
        crate::operation::enable_vpc_classic_link_dns_support::builders::EnableVpcClassicLinkDnsSupportInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::enable_vpc_classic_link_dns_support::EnableVpcClassicLinkDnsSupportInput;
/// A builder for [`EnableVpcClassicLinkDnsSupportInput`](crate::operation::enable_vpc_classic_link_dns_support::EnableVpcClassicLinkDnsSupportInput).
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
pub struct EnableVpcClassicLinkDnsSupportInputBuilder {
    pub(crate) vpc_id: std::option::Option<std::string::String>,
}
impl EnableVpcClassicLinkDnsSupportInputBuilder {
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
    /// Consumes the builder and constructs a [`EnableVpcClassicLinkDnsSupportInput`](crate::operation::enable_vpc_classic_link_dns_support::EnableVpcClassicLinkDnsSupportInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::enable_vpc_classic_link_dns_support::EnableVpcClassicLinkDnsSupportInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::enable_vpc_classic_link_dns_support::EnableVpcClassicLinkDnsSupportInput {
                vpc_id: self.vpc_id
                ,
            }
        )
    }
}
