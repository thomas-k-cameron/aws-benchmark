// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the IPv6 delegated prefixes assigned to a network interface.</p>
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
pub struct Ipv6PrefixSpecificationResponse {
    /// <p>The IPv6 delegated prefixes assigned to the network interface.</p>
    #[doc(hidden)]
    pub ipv6_prefix: std::option::Option<std::string::String>,
}
impl Ipv6PrefixSpecificationResponse {
    /// <p>The IPv6 delegated prefixes assigned to the network interface.</p>
    pub fn ipv6_prefix(&self) -> std::option::Option<&str> {
        self.ipv6_prefix.as_deref()
    }
}
impl Ipv6PrefixSpecificationResponse {
    /// Creates a new builder-style object to manufacture [`Ipv6PrefixSpecificationResponse`](crate::types::Ipv6PrefixSpecificationResponse).
    pub fn builder() -> crate::types::builders::Ipv6PrefixSpecificationResponseBuilder {
        crate::types::builders::Ipv6PrefixSpecificationResponseBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Ipv6PrefixSpecificationResponse;
/// A builder for [`Ipv6PrefixSpecificationResponse`](crate::types::Ipv6PrefixSpecificationResponse).
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
pub struct Ipv6PrefixSpecificationResponseBuilder {
    pub(crate) ipv6_prefix: std::option::Option<std::string::String>,
}
impl Ipv6PrefixSpecificationResponseBuilder {
    /// <p>The IPv6 delegated prefixes assigned to the network interface.</p>
    pub fn ipv6_prefix(mut self, input: impl Into<std::string::String>) -> Self {
        self.ipv6_prefix = Some(input.into());
        self
    }
    /// <p>The IPv6 delegated prefixes assigned to the network interface.</p>
    pub fn set_ipv6_prefix(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ipv6_prefix = input;
        self
    }
    /// Consumes the builder and constructs a [`Ipv6PrefixSpecificationResponse`](crate::types::Ipv6PrefixSpecificationResponse).
    pub fn build(self) -> crate::types::Ipv6PrefixSpecificationResponse {
        crate::types::Ipv6PrefixSpecificationResponse {
            ipv6_prefix: self.ipv6_prefix,
        }
    }
}
