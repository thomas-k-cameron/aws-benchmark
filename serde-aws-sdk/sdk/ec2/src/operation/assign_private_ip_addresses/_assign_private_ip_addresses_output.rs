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
pub struct AssignPrivateIpAddressesOutput {
    /// <p>The ID of the network interface.</p>
    #[doc(hidden)]
    pub network_interface_id: std::option::Option<std::string::String>,
    /// <p>The private IP addresses assigned to the network interface.</p>
    #[doc(hidden)]
    pub assigned_private_ip_addresses:
        std::option::Option<std::vec::Vec<crate::types::AssignedPrivateIpAddress>>,
    /// <p>The IPv4 prefixes that are assigned to the network interface.</p>
    #[doc(hidden)]
    pub assigned_ipv4_prefixes:
        std::option::Option<std::vec::Vec<crate::types::Ipv4PrefixSpecification>>,
    _request_id: Option<String>,
}
impl AssignPrivateIpAddressesOutput {
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(&self) -> std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p>The private IP addresses assigned to the network interface.</p>
    pub fn assigned_private_ip_addresses(
        &self,
    ) -> std::option::Option<&[crate::types::AssignedPrivateIpAddress]> {
        self.assigned_private_ip_addresses.as_deref()
    }
    /// <p>The IPv4 prefixes that are assigned to the network interface.</p>
    pub fn assigned_ipv4_prefixes(
        &self,
    ) -> std::option::Option<&[crate::types::Ipv4PrefixSpecification]> {
        self.assigned_ipv4_prefixes.as_deref()
    }
}
impl aws_http::request_id::RequestId for AssignPrivateIpAddressesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AssignPrivateIpAddressesOutput {
    /// Creates a new builder-style object to manufacture [`AssignPrivateIpAddressesOutput`](crate::operation::assign_private_ip_addresses::AssignPrivateIpAddressesOutput).
    pub fn builder() -> crate::operation::assign_private_ip_addresses::builders::AssignPrivateIpAddressesOutputBuilder{
        crate::operation::assign_private_ip_addresses::builders::AssignPrivateIpAddressesOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::assign_private_ip_addresses::AssignPrivateIpAddressesOutput;
/// A builder for [`AssignPrivateIpAddressesOutput`](crate::operation::assign_private_ip_addresses::AssignPrivateIpAddressesOutput).
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
pub struct AssignPrivateIpAddressesOutputBuilder {
    pub(crate) network_interface_id: std::option::Option<std::string::String>,
    pub(crate) assigned_private_ip_addresses:
        std::option::Option<std::vec::Vec<crate::types::AssignedPrivateIpAddress>>,
    pub(crate) assigned_ipv4_prefixes:
        std::option::Option<std::vec::Vec<crate::types::Ipv4PrefixSpecification>>,
    _request_id: Option<String>,
}
impl AssignPrivateIpAddressesOutputBuilder {
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.network_interface_id = Some(input.into());
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn set_network_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_interface_id = input;
        self
    }
    /// Appends an item to `assigned_private_ip_addresses`.
    ///
    /// To override the contents of this collection use [`set_assigned_private_ip_addresses`](Self::set_assigned_private_ip_addresses).
    ///
    /// <p>The private IP addresses assigned to the network interface.</p>
    pub fn assigned_private_ip_addresses(
        mut self,
        input: crate::types::AssignedPrivateIpAddress,
    ) -> Self {
        let mut v = self.assigned_private_ip_addresses.unwrap_or_default();
        v.push(input);
        self.assigned_private_ip_addresses = Some(v);
        self
    }
    /// <p>The private IP addresses assigned to the network interface.</p>
    pub fn set_assigned_private_ip_addresses(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AssignedPrivateIpAddress>>,
    ) -> Self {
        self.assigned_private_ip_addresses = input;
        self
    }
    /// Appends an item to `assigned_ipv4_prefixes`.
    ///
    /// To override the contents of this collection use [`set_assigned_ipv4_prefixes`](Self::set_assigned_ipv4_prefixes).
    ///
    /// <p>The IPv4 prefixes that are assigned to the network interface.</p>
    pub fn assigned_ipv4_prefixes(mut self, input: crate::types::Ipv4PrefixSpecification) -> Self {
        let mut v = self.assigned_ipv4_prefixes.unwrap_or_default();
        v.push(input);
        self.assigned_ipv4_prefixes = Some(v);
        self
    }
    /// <p>The IPv4 prefixes that are assigned to the network interface.</p>
    pub fn set_assigned_ipv4_prefixes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Ipv4PrefixSpecification>>,
    ) -> Self {
        self.assigned_ipv4_prefixes = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`AssignPrivateIpAddressesOutput`](crate::operation::assign_private_ip_addresses::AssignPrivateIpAddressesOutput).
    pub fn build(
        self,
    ) -> crate::operation::assign_private_ip_addresses::AssignPrivateIpAddressesOutput {
        crate::operation::assign_private_ip_addresses::AssignPrivateIpAddressesOutput {
            network_interface_id: self.network_interface_id,
            assigned_private_ip_addresses: self.assigned_private_ip_addresses,
            assigned_ipv4_prefixes: self.assigned_ipv4_prefixes,
            _request_id: self._request_id,
        }
    }
}