// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an IPv6 address.</p>
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
pub struct InstanceIpv6Address {
    /// <p>The IPv6 address.</p>
    #[doc(hidden)]
    pub ipv6_address: std::option::Option<std::string::String>,
}
impl InstanceIpv6Address {
    /// <p>The IPv6 address.</p>
    pub fn ipv6_address(&self) -> std::option::Option<&str> {
        self.ipv6_address.as_deref()
    }
}
impl InstanceIpv6Address {
    /// Creates a new builder-style object to manufacture [`InstanceIpv6Address`](crate::types::InstanceIpv6Address).
    pub fn builder() -> crate::types::builders::InstanceIpv6AddressBuilder {
        crate::types::builders::InstanceIpv6AddressBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::InstanceIpv6Address;
/// A builder for [`InstanceIpv6Address`](crate::types::InstanceIpv6Address).
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
pub struct InstanceIpv6AddressBuilder {
    pub(crate) ipv6_address: std::option::Option<std::string::String>,
}
impl InstanceIpv6AddressBuilder {
    /// <p>The IPv6 address.</p>
    pub fn ipv6_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.ipv6_address = Some(input.into());
        self
    }
    /// <p>The IPv6 address.</p>
    pub fn set_ipv6_address(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ipv6_address = input;
        self
    }
    /// Consumes the builder and constructs a [`InstanceIpv6Address`](crate::types::InstanceIpv6Address).
    pub fn build(self) -> crate::types::InstanceIpv6Address {
        crate::types::InstanceIpv6Address {
            ipv6_address: self.ipv6_address,
        }
    }
}