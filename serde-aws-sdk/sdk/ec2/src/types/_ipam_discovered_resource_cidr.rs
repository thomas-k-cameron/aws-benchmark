// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An IPAM discovered resource CIDR. A discovered resource is a resource CIDR monitored under a resource discovery. The following resources can be discovered: VPCs, Public IPv4 pools, VPC subnets, and Elastic IP addresses. The discovered resource CIDR is the IP address range in CIDR notation that is associated with the resource.</p>
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
pub struct IpamDiscoveredResourceCidr {
    /// <p>The resource discovery ID.</p>
    #[doc(hidden)]
    pub ipam_resource_discovery_id: std::option::Option<std::string::String>,
    /// <p>The resource Region.</p>
    #[doc(hidden)]
    pub resource_region: std::option::Option<std::string::String>,
    /// <p>The resource ID.</p>
    #[doc(hidden)]
    pub resource_id: std::option::Option<std::string::String>,
    /// <p>The resource owner ID.</p>
    #[doc(hidden)]
    pub resource_owner_id: std::option::Option<std::string::String>,
    /// <p>The resource CIDR.</p>
    #[doc(hidden)]
    pub resource_cidr: std::option::Option<std::string::String>,
    /// <p>The resource type.</p>
    #[doc(hidden)]
    pub resource_type: std::option::Option<crate::types::IpamResourceType>,
    /// <p>The resource tags.</p>
    #[doc(hidden)]
    pub resource_tags: std::option::Option<std::vec::Vec<crate::types::IpamResourceTag>>,
    /// <p>The percentage of IP address space in use. To convert the decimal to a percentage, multiply the decimal by 100. Note the following:</p>
    /// <ul>
    /// <li> <p>For resources that are VPCs, this is the percentage of IP address space in the VPC that's taken up by subnet CIDRs. </p> </li>
    /// <li> <p>For resources that are subnets, if the subnet has an IPv4 CIDR provisioned to it, this is the percentage of IPv4 address space in the subnet that's in use. If the subnet has an IPv6 CIDR provisioned to it, the percentage of IPv6 address space in use is not represented. The percentage of IPv6 address space in use cannot currently be calculated. </p> </li>
    /// <li> <p>For resources that are public IPv4 pools, this is the percentage of IP address space in the pool that's been allocated to Elastic IP addresses (EIPs). </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub ip_usage: std::option::Option<f64>,
    /// <p>The VPC ID.</p>
    #[doc(hidden)]
    pub vpc_id: std::option::Option<std::string::String>,
    /// <p>The last successful resource discovery time.</p>
    #[doc(hidden)]
    pub sample_time: std::option::Option<aws_smithy_types::DateTime>,
}
impl IpamDiscoveredResourceCidr {
    /// <p>The resource discovery ID.</p>
    pub fn ipam_resource_discovery_id(&self) -> std::option::Option<&str> {
        self.ipam_resource_discovery_id.as_deref()
    }
    /// <p>The resource Region.</p>
    pub fn resource_region(&self) -> std::option::Option<&str> {
        self.resource_region.as_deref()
    }
    /// <p>The resource ID.</p>
    pub fn resource_id(&self) -> std::option::Option<&str> {
        self.resource_id.as_deref()
    }
    /// <p>The resource owner ID.</p>
    pub fn resource_owner_id(&self) -> std::option::Option<&str> {
        self.resource_owner_id.as_deref()
    }
    /// <p>The resource CIDR.</p>
    pub fn resource_cidr(&self) -> std::option::Option<&str> {
        self.resource_cidr.as_deref()
    }
    /// <p>The resource type.</p>
    pub fn resource_type(&self) -> std::option::Option<&crate::types::IpamResourceType> {
        self.resource_type.as_ref()
    }
    /// <p>The resource tags.</p>
    pub fn resource_tags(&self) -> std::option::Option<&[crate::types::IpamResourceTag]> {
        self.resource_tags.as_deref()
    }
    /// <p>The percentage of IP address space in use. To convert the decimal to a percentage, multiply the decimal by 100. Note the following:</p>
    /// <ul>
    /// <li> <p>For resources that are VPCs, this is the percentage of IP address space in the VPC that's taken up by subnet CIDRs. </p> </li>
    /// <li> <p>For resources that are subnets, if the subnet has an IPv4 CIDR provisioned to it, this is the percentage of IPv4 address space in the subnet that's in use. If the subnet has an IPv6 CIDR provisioned to it, the percentage of IPv6 address space in use is not represented. The percentage of IPv6 address space in use cannot currently be calculated. </p> </li>
    /// <li> <p>For resources that are public IPv4 pools, this is the percentage of IP address space in the pool that's been allocated to Elastic IP addresses (EIPs). </p> </li>
    /// </ul>
    pub fn ip_usage(&self) -> std::option::Option<f64> {
        self.ip_usage
    }
    /// <p>The VPC ID.</p>
    pub fn vpc_id(&self) -> std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>The last successful resource discovery time.</p>
    pub fn sample_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.sample_time.as_ref()
    }
}
impl IpamDiscoveredResourceCidr {
    /// Creates a new builder-style object to manufacture [`IpamDiscoveredResourceCidr`](crate::types::IpamDiscoveredResourceCidr).
    pub fn builder() -> crate::types::builders::IpamDiscoveredResourceCidrBuilder {
        crate::types::builders::IpamDiscoveredResourceCidrBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::IpamDiscoveredResourceCidr;
/// A builder for [`IpamDiscoveredResourceCidr`](crate::types::IpamDiscoveredResourceCidr).
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
pub struct IpamDiscoveredResourceCidrBuilder {
    pub(crate) ipam_resource_discovery_id: std::option::Option<std::string::String>,
    pub(crate) resource_region: std::option::Option<std::string::String>,
    pub(crate) resource_id: std::option::Option<std::string::String>,
    pub(crate) resource_owner_id: std::option::Option<std::string::String>,
    pub(crate) resource_cidr: std::option::Option<std::string::String>,
    pub(crate) resource_type: std::option::Option<crate::types::IpamResourceType>,
    pub(crate) resource_tags: std::option::Option<std::vec::Vec<crate::types::IpamResourceTag>>,
    pub(crate) ip_usage: std::option::Option<f64>,
    pub(crate) vpc_id: std::option::Option<std::string::String>,
    pub(crate) sample_time: std::option::Option<aws_smithy_types::DateTime>,
}
impl IpamDiscoveredResourceCidrBuilder {
    /// <p>The resource discovery ID.</p>
    pub fn ipam_resource_discovery_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.ipam_resource_discovery_id = Some(input.into());
        self
    }
    /// <p>The resource discovery ID.</p>
    pub fn set_ipam_resource_discovery_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.ipam_resource_discovery_id = input;
        self
    }
    /// <p>The resource Region.</p>
    pub fn resource_region(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_region = Some(input.into());
        self
    }
    /// <p>The resource Region.</p>
    pub fn set_resource_region(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resource_region = input;
        self
    }
    /// <p>The resource ID.</p>
    pub fn resource_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_id = Some(input.into());
        self
    }
    /// <p>The resource ID.</p>
    pub fn set_resource_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resource_id = input;
        self
    }
    /// <p>The resource owner ID.</p>
    pub fn resource_owner_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_owner_id = Some(input.into());
        self
    }
    /// <p>The resource owner ID.</p>
    pub fn set_resource_owner_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.resource_owner_id = input;
        self
    }
    /// <p>The resource CIDR.</p>
    pub fn resource_cidr(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_cidr = Some(input.into());
        self
    }
    /// <p>The resource CIDR.</p>
    pub fn set_resource_cidr(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resource_cidr = input;
        self
    }
    /// <p>The resource type.</p>
    pub fn resource_type(mut self, input: crate::types::IpamResourceType) -> Self {
        self.resource_type = Some(input);
        self
    }
    /// <p>The resource type.</p>
    pub fn set_resource_type(
        mut self,
        input: std::option::Option<crate::types::IpamResourceType>,
    ) -> Self {
        self.resource_type = input;
        self
    }
    /// Appends an item to `resource_tags`.
    ///
    /// To override the contents of this collection use [`set_resource_tags`](Self::set_resource_tags).
    ///
    /// <p>The resource tags.</p>
    pub fn resource_tags(mut self, input: crate::types::IpamResourceTag) -> Self {
        let mut v = self.resource_tags.unwrap_or_default();
        v.push(input);
        self.resource_tags = Some(v);
        self
    }
    /// <p>The resource tags.</p>
    pub fn set_resource_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::IpamResourceTag>>,
    ) -> Self {
        self.resource_tags = input;
        self
    }
    /// <p>The percentage of IP address space in use. To convert the decimal to a percentage, multiply the decimal by 100. Note the following:</p>
    /// <ul>
    /// <li> <p>For resources that are VPCs, this is the percentage of IP address space in the VPC that's taken up by subnet CIDRs. </p> </li>
    /// <li> <p>For resources that are subnets, if the subnet has an IPv4 CIDR provisioned to it, this is the percentage of IPv4 address space in the subnet that's in use. If the subnet has an IPv6 CIDR provisioned to it, the percentage of IPv6 address space in use is not represented. The percentage of IPv6 address space in use cannot currently be calculated. </p> </li>
    /// <li> <p>For resources that are public IPv4 pools, this is the percentage of IP address space in the pool that's been allocated to Elastic IP addresses (EIPs). </p> </li>
    /// </ul>
    pub fn ip_usage(mut self, input: f64) -> Self {
        self.ip_usage = Some(input);
        self
    }
    /// <p>The percentage of IP address space in use. To convert the decimal to a percentage, multiply the decimal by 100. Note the following:</p>
    /// <ul>
    /// <li> <p>For resources that are VPCs, this is the percentage of IP address space in the VPC that's taken up by subnet CIDRs. </p> </li>
    /// <li> <p>For resources that are subnets, if the subnet has an IPv4 CIDR provisioned to it, this is the percentage of IPv4 address space in the subnet that's in use. If the subnet has an IPv6 CIDR provisioned to it, the percentage of IPv6 address space in use is not represented. The percentage of IPv6 address space in use cannot currently be calculated. </p> </li>
    /// <li> <p>For resources that are public IPv4 pools, this is the percentage of IP address space in the pool that's been allocated to Elastic IP addresses (EIPs). </p> </li>
    /// </ul>
    pub fn set_ip_usage(mut self, input: std::option::Option<f64>) -> Self {
        self.ip_usage = input;
        self
    }
    /// <p>The VPC ID.</p>
    pub fn vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.vpc_id = Some(input.into());
        self
    }
    /// <p>The VPC ID.</p>
    pub fn set_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// <p>The last successful resource discovery time.</p>
    pub fn sample_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.sample_time = Some(input);
        self
    }
    /// <p>The last successful resource discovery time.</p>
    pub fn set_sample_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.sample_time = input;
        self
    }
    /// Consumes the builder and constructs a [`IpamDiscoveredResourceCidr`](crate::types::IpamDiscoveredResourceCidr).
    pub fn build(self) -> crate::types::IpamDiscoveredResourceCidr {
        crate::types::IpamDiscoveredResourceCidr {
            ipam_resource_discovery_id: self.ipam_resource_discovery_id,
            resource_region: self.resource_region,
            resource_id: self.resource_id,
            resource_owner_id: self.resource_owner_id,
            resource_cidr: self.resource_cidr,
            resource_type: self.resource_type,
            resource_tags: self.resource_tags,
            ip_usage: self.ip_usage,
            vpc_id: self.vpc_id,
            sample_time: self.sample_time,
        }
    }
}