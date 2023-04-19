// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a local gateway virtual interface.</p>
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
pub struct LocalGatewayVirtualInterface {
    /// <p>The ID of the virtual interface.</p>
    #[doc(hidden)]
    pub local_gateway_virtual_interface_id: std::option::Option<std::string::String>,
    /// <p>The ID of the local gateway.</p>
    #[doc(hidden)]
    pub local_gateway_id: std::option::Option<std::string::String>,
    /// <p>The ID of the VLAN.</p>
    #[doc(hidden)]
    pub vlan: std::option::Option<i32>,
    /// <p>The local address.</p>
    #[doc(hidden)]
    pub local_address: std::option::Option<std::string::String>,
    /// <p>The peer address.</p>
    #[doc(hidden)]
    pub peer_address: std::option::Option<std::string::String>,
    /// <p>The Border Gateway Protocol (BGP) Autonomous System Number (ASN) of the local gateway.</p>
    #[doc(hidden)]
    pub local_bgp_asn: std::option::Option<i32>,
    /// <p>The peer BGP ASN.</p>
    #[doc(hidden)]
    pub peer_bgp_asn: std::option::Option<i32>,
    /// <p>The ID of the Amazon Web Services account that owns the local gateway virtual interface.</p>
    #[doc(hidden)]
    pub owner_id: std::option::Option<std::string::String>,
    /// <p>The tags assigned to the virtual interface.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl LocalGatewayVirtualInterface {
    /// <p>The ID of the virtual interface.</p>
    pub fn local_gateway_virtual_interface_id(&self) -> std::option::Option<&str> {
        self.local_gateway_virtual_interface_id.as_deref()
    }
    /// <p>The ID of the local gateway.</p>
    pub fn local_gateway_id(&self) -> std::option::Option<&str> {
        self.local_gateway_id.as_deref()
    }
    /// <p>The ID of the VLAN.</p>
    pub fn vlan(&self) -> std::option::Option<i32> {
        self.vlan
    }
    /// <p>The local address.</p>
    pub fn local_address(&self) -> std::option::Option<&str> {
        self.local_address.as_deref()
    }
    /// <p>The peer address.</p>
    pub fn peer_address(&self) -> std::option::Option<&str> {
        self.peer_address.as_deref()
    }
    /// <p>The Border Gateway Protocol (BGP) Autonomous System Number (ASN) of the local gateway.</p>
    pub fn local_bgp_asn(&self) -> std::option::Option<i32> {
        self.local_bgp_asn
    }
    /// <p>The peer BGP ASN.</p>
    pub fn peer_bgp_asn(&self) -> std::option::Option<i32> {
        self.peer_bgp_asn
    }
    /// <p>The ID of the Amazon Web Services account that owns the local gateway virtual interface.</p>
    pub fn owner_id(&self) -> std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p>The tags assigned to the virtual interface.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl LocalGatewayVirtualInterface {
    /// Creates a new builder-style object to manufacture [`LocalGatewayVirtualInterface`](crate::types::LocalGatewayVirtualInterface).
    pub fn builder() -> crate::types::builders::LocalGatewayVirtualInterfaceBuilder {
        crate::types::builders::LocalGatewayVirtualInterfaceBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::LocalGatewayVirtualInterface;
/// A builder for [`LocalGatewayVirtualInterface`](crate::types::LocalGatewayVirtualInterface).
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
pub struct LocalGatewayVirtualInterfaceBuilder {
    pub(crate) local_gateway_virtual_interface_id: std::option::Option<std::string::String>,
    pub(crate) local_gateway_id: std::option::Option<std::string::String>,
    pub(crate) vlan: std::option::Option<i32>,
    pub(crate) local_address: std::option::Option<std::string::String>,
    pub(crate) peer_address: std::option::Option<std::string::String>,
    pub(crate) local_bgp_asn: std::option::Option<i32>,
    pub(crate) peer_bgp_asn: std::option::Option<i32>,
    pub(crate) owner_id: std::option::Option<std::string::String>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl LocalGatewayVirtualInterfaceBuilder {
    /// <p>The ID of the virtual interface.</p>
    pub fn local_gateway_virtual_interface_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.local_gateway_virtual_interface_id = Some(input.into());
        self
    }
    /// <p>The ID of the virtual interface.</p>
    pub fn set_local_gateway_virtual_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.local_gateway_virtual_interface_id = input;
        self
    }
    /// <p>The ID of the local gateway.</p>
    pub fn local_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.local_gateway_id = Some(input.into());
        self
    }
    /// <p>The ID of the local gateway.</p>
    pub fn set_local_gateway_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.local_gateway_id = input;
        self
    }
    /// <p>The ID of the VLAN.</p>
    pub fn vlan(mut self, input: i32) -> Self {
        self.vlan = Some(input);
        self
    }
    /// <p>The ID of the VLAN.</p>
    pub fn set_vlan(mut self, input: std::option::Option<i32>) -> Self {
        self.vlan = input;
        self
    }
    /// <p>The local address.</p>
    pub fn local_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.local_address = Some(input.into());
        self
    }
    /// <p>The local address.</p>
    pub fn set_local_address(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.local_address = input;
        self
    }
    /// <p>The peer address.</p>
    pub fn peer_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.peer_address = Some(input.into());
        self
    }
    /// <p>The peer address.</p>
    pub fn set_peer_address(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.peer_address = input;
        self
    }
    /// <p>The Border Gateway Protocol (BGP) Autonomous System Number (ASN) of the local gateway.</p>
    pub fn local_bgp_asn(mut self, input: i32) -> Self {
        self.local_bgp_asn = Some(input);
        self
    }
    /// <p>The Border Gateway Protocol (BGP) Autonomous System Number (ASN) of the local gateway.</p>
    pub fn set_local_bgp_asn(mut self, input: std::option::Option<i32>) -> Self {
        self.local_bgp_asn = input;
        self
    }
    /// <p>The peer BGP ASN.</p>
    pub fn peer_bgp_asn(mut self, input: i32) -> Self {
        self.peer_bgp_asn = Some(input);
        self
    }
    /// <p>The peer BGP ASN.</p>
    pub fn set_peer_bgp_asn(mut self, input: std::option::Option<i32>) -> Self {
        self.peer_bgp_asn = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the local gateway virtual interface.</p>
    pub fn owner_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.owner_id = Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the local gateway virtual interface.</p>
    pub fn set_owner_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags assigned to the virtual interface.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>The tags assigned to the virtual interface.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`LocalGatewayVirtualInterface`](crate::types::LocalGatewayVirtualInterface).
    pub fn build(self) -> crate::types::LocalGatewayVirtualInterface {
        crate::types::LocalGatewayVirtualInterface {
            local_gateway_virtual_interface_id: self.local_gateway_virtual_interface_id,
            local_gateway_id: self.local_gateway_id,
            vlan: self.vlan,
            local_address: self.local_address,
            peer_address: self.peer_address,
            local_bgp_asn: self.local_bgp_asn,
            peer_bgp_asn: self.peer_bgp_asn,
            owner_id: self.owner_id,
            tags: self.tags,
        }
    }
}