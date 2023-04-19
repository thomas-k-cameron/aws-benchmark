// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an Elastic IP address, or a carrier IP address.</p>
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
pub struct Address {
    /// <p>The ID of the instance that the address is associated with (if any).</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>The Elastic IP address.</p>
    #[doc(hidden)]
    pub public_ip: std::option::Option<std::string::String>,
    /// <p>The ID representing the allocation of the address for use with EC2-VPC.</p>
    #[doc(hidden)]
    pub allocation_id: std::option::Option<std::string::String>,
    /// <p>The ID representing the association of the address with an instance in a VPC.</p>
    #[doc(hidden)]
    pub association_id: std::option::Option<std::string::String>,
    /// <p>Indicates whether this Elastic IP address is for use with instances in EC2-Classic (<code>standard</code>) or instances in a VPC (<code>vpc</code>).</p>
    #[doc(hidden)]
    pub domain: std::option::Option<crate::types::DomainType>,
    /// <p>The ID of the network interface.</p>
    #[doc(hidden)]
    pub network_interface_id: std::option::Option<std::string::String>,
    /// <p>The ID of the Amazon Web Services account that owns the network interface.</p>
    #[doc(hidden)]
    pub network_interface_owner_id: std::option::Option<std::string::String>,
    /// <p>The private IP address associated with the Elastic IP address.</p>
    #[doc(hidden)]
    pub private_ip_address: std::option::Option<std::string::String>,
    /// <p>Any tags assigned to the Elastic IP address.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    /// <p>The ID of an address pool.</p>
    #[doc(hidden)]
    pub public_ipv4_pool: std::option::Option<std::string::String>,
    /// <p>The name of the unique set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    #[doc(hidden)]
    pub network_border_group: std::option::Option<std::string::String>,
    /// <p>The customer-owned IP address.</p>
    #[doc(hidden)]
    pub customer_owned_ip: std::option::Option<std::string::String>,
    /// <p>The ID of the customer-owned address pool.</p>
    #[doc(hidden)]
    pub customer_owned_ipv4_pool: std::option::Option<std::string::String>,
    /// <p>The carrier IP address associated. This option is only available for network interfaces which reside in a subnet in a Wavelength Zone (for example an EC2 instance). </p>
    #[doc(hidden)]
    pub carrier_ip: std::option::Option<std::string::String>,
}
impl Address {
    /// <p>The ID of the instance that the address is associated with (if any).</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The Elastic IP address.</p>
    pub fn public_ip(&self) -> std::option::Option<&str> {
        self.public_ip.as_deref()
    }
    /// <p>The ID representing the allocation of the address for use with EC2-VPC.</p>
    pub fn allocation_id(&self) -> std::option::Option<&str> {
        self.allocation_id.as_deref()
    }
    /// <p>The ID representing the association of the address with an instance in a VPC.</p>
    pub fn association_id(&self) -> std::option::Option<&str> {
        self.association_id.as_deref()
    }
    /// <p>Indicates whether this Elastic IP address is for use with instances in EC2-Classic (<code>standard</code>) or instances in a VPC (<code>vpc</code>).</p>
    pub fn domain(&self) -> std::option::Option<&crate::types::DomainType> {
        self.domain.as_ref()
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(&self) -> std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p>The ID of the Amazon Web Services account that owns the network interface.</p>
    pub fn network_interface_owner_id(&self) -> std::option::Option<&str> {
        self.network_interface_owner_id.as_deref()
    }
    /// <p>The private IP address associated with the Elastic IP address.</p>
    pub fn private_ip_address(&self) -> std::option::Option<&str> {
        self.private_ip_address.as_deref()
    }
    /// <p>Any tags assigned to the Elastic IP address.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>The ID of an address pool.</p>
    pub fn public_ipv4_pool(&self) -> std::option::Option<&str> {
        self.public_ipv4_pool.as_deref()
    }
    /// <p>The name of the unique set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    pub fn network_border_group(&self) -> std::option::Option<&str> {
        self.network_border_group.as_deref()
    }
    /// <p>The customer-owned IP address.</p>
    pub fn customer_owned_ip(&self) -> std::option::Option<&str> {
        self.customer_owned_ip.as_deref()
    }
    /// <p>The ID of the customer-owned address pool.</p>
    pub fn customer_owned_ipv4_pool(&self) -> std::option::Option<&str> {
        self.customer_owned_ipv4_pool.as_deref()
    }
    /// <p>The carrier IP address associated. This option is only available for network interfaces which reside in a subnet in a Wavelength Zone (for example an EC2 instance). </p>
    pub fn carrier_ip(&self) -> std::option::Option<&str> {
        self.carrier_ip.as_deref()
    }
}
impl Address {
    /// Creates a new builder-style object to manufacture [`Address`](crate::types::Address).
    pub fn builder() -> crate::types::builders::AddressBuilder {
        crate::types::builders::AddressBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Address;
/// A builder for [`Address`](crate::types::Address).
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
pub struct AddressBuilder {
    pub(crate) instance_id: std::option::Option<std::string::String>,
    pub(crate) public_ip: std::option::Option<std::string::String>,
    pub(crate) allocation_id: std::option::Option<std::string::String>,
    pub(crate) association_id: std::option::Option<std::string::String>,
    pub(crate) domain: std::option::Option<crate::types::DomainType>,
    pub(crate) network_interface_id: std::option::Option<std::string::String>,
    pub(crate) network_interface_owner_id: std::option::Option<std::string::String>,
    pub(crate) private_ip_address: std::option::Option<std::string::String>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    pub(crate) public_ipv4_pool: std::option::Option<std::string::String>,
    pub(crate) network_border_group: std::option::Option<std::string::String>,
    pub(crate) customer_owned_ip: std::option::Option<std::string::String>,
    pub(crate) customer_owned_ipv4_pool: std::option::Option<std::string::String>,
    pub(crate) carrier_ip: std::option::Option<std::string::String>,
}
impl AddressBuilder {
    /// <p>The ID of the instance that the address is associated with (if any).</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_id = Some(input.into());
        self
    }
    /// <p>The ID of the instance that the address is associated with (if any).</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The Elastic IP address.</p>
    pub fn public_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.public_ip = Some(input.into());
        self
    }
    /// <p>The Elastic IP address.</p>
    pub fn set_public_ip(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.public_ip = input;
        self
    }
    /// <p>The ID representing the allocation of the address for use with EC2-VPC.</p>
    pub fn allocation_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.allocation_id = Some(input.into());
        self
    }
    /// <p>The ID representing the allocation of the address for use with EC2-VPC.</p>
    pub fn set_allocation_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.allocation_id = input;
        self
    }
    /// <p>The ID representing the association of the address with an instance in a VPC.</p>
    pub fn association_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.association_id = Some(input.into());
        self
    }
    /// <p>The ID representing the association of the address with an instance in a VPC.</p>
    pub fn set_association_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.association_id = input;
        self
    }
    /// <p>Indicates whether this Elastic IP address is for use with instances in EC2-Classic (<code>standard</code>) or instances in a VPC (<code>vpc</code>).</p>
    pub fn domain(mut self, input: crate::types::DomainType) -> Self {
        self.domain = Some(input);
        self
    }
    /// <p>Indicates whether this Elastic IP address is for use with instances in EC2-Classic (<code>standard</code>) or instances in a VPC (<code>vpc</code>).</p>
    pub fn set_domain(mut self, input: std::option::Option<crate::types::DomainType>) -> Self {
        self.domain = input;
        self
    }
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
    /// <p>The ID of the Amazon Web Services account that owns the network interface.</p>
    pub fn network_interface_owner_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.network_interface_owner_id = Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the network interface.</p>
    pub fn set_network_interface_owner_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_interface_owner_id = input;
        self
    }
    /// <p>The private IP address associated with the Elastic IP address.</p>
    pub fn private_ip_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.private_ip_address = Some(input.into());
        self
    }
    /// <p>The private IP address associated with the Elastic IP address.</p>
    pub fn set_private_ip_address(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.private_ip_address = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags assigned to the Elastic IP address.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>Any tags assigned to the Elastic IP address.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>The ID of an address pool.</p>
    pub fn public_ipv4_pool(mut self, input: impl Into<std::string::String>) -> Self {
        self.public_ipv4_pool = Some(input.into());
        self
    }
    /// <p>The ID of an address pool.</p>
    pub fn set_public_ipv4_pool(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.public_ipv4_pool = input;
        self
    }
    /// <p>The name of the unique set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    pub fn network_border_group(mut self, input: impl Into<std::string::String>) -> Self {
        self.network_border_group = Some(input.into());
        self
    }
    /// <p>The name of the unique set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    pub fn set_network_border_group(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_border_group = input;
        self
    }
    /// <p>The customer-owned IP address.</p>
    pub fn customer_owned_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.customer_owned_ip = Some(input.into());
        self
    }
    /// <p>The customer-owned IP address.</p>
    pub fn set_customer_owned_ip(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.customer_owned_ip = input;
        self
    }
    /// <p>The ID of the customer-owned address pool.</p>
    pub fn customer_owned_ipv4_pool(mut self, input: impl Into<std::string::String>) -> Self {
        self.customer_owned_ipv4_pool = Some(input.into());
        self
    }
    /// <p>The ID of the customer-owned address pool.</p>
    pub fn set_customer_owned_ipv4_pool(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.customer_owned_ipv4_pool = input;
        self
    }
    /// <p>The carrier IP address associated. This option is only available for network interfaces which reside in a subnet in a Wavelength Zone (for example an EC2 instance). </p>
    pub fn carrier_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.carrier_ip = Some(input.into());
        self
    }
    /// <p>The carrier IP address associated. This option is only available for network interfaces which reside in a subnet in a Wavelength Zone (for example an EC2 instance). </p>
    pub fn set_carrier_ip(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.carrier_ip = input;
        self
    }
    /// Consumes the builder and constructs a [`Address`](crate::types::Address).
    pub fn build(self) -> crate::types::Address {
        crate::types::Address {
            instance_id: self.instance_id,
            public_ip: self.public_ip,
            allocation_id: self.allocation_id,
            association_id: self.association_id,
            domain: self.domain,
            network_interface_id: self.network_interface_id,
            network_interface_owner_id: self.network_interface_owner_id,
            private_ip_address: self.private_ip_address,
            tags: self.tags,
            public_ipv4_pool: self.public_ipv4_pool,
            network_border_group: self.network_border_group,
            customer_owned_ip: self.customer_owned_ip,
            customer_owned_ipv4_pool: self.customer_owned_ipv4_pool,
            carrier_ip: self.carrier_ip,
        }
    }
}