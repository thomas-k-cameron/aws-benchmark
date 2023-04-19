// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <note>
/// <p>We are retiring EC2-Classic. We recommend that you migrate from EC2-Classic to a VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-migrate.html">Migrate from EC2-Classic to a VPC</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
/// </note>
/// <p>Describes the VPC peering connection options.</p>
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
pub struct VpcPeeringConnectionOptionsDescription {
    /// <p>Indicates whether a local VPC can resolve public DNS hostnames to private IP addresses when queried from instances in a peer VPC.</p>
    #[doc(hidden)]
    pub allow_dns_resolution_from_remote_vpc: std::option::Option<bool>,
    /// <p>Indicates whether a local ClassicLink connection can communicate with the peer VPC over the VPC peering connection.</p>
    #[doc(hidden)]
    pub allow_egress_from_local_classic_link_to_remote_vpc: std::option::Option<bool>,
    /// <p>Indicates whether a local VPC can communicate with a ClassicLink connection in the peer VPC over the VPC peering connection.</p>
    #[doc(hidden)]
    pub allow_egress_from_local_vpc_to_remote_classic_link: std::option::Option<bool>,
}
impl VpcPeeringConnectionOptionsDescription {
    /// <p>Indicates whether a local VPC can resolve public DNS hostnames to private IP addresses when queried from instances in a peer VPC.</p>
    pub fn allow_dns_resolution_from_remote_vpc(&self) -> std::option::Option<bool> {
        self.allow_dns_resolution_from_remote_vpc
    }
    /// <p>Indicates whether a local ClassicLink connection can communicate with the peer VPC over the VPC peering connection.</p>
    pub fn allow_egress_from_local_classic_link_to_remote_vpc(&self) -> std::option::Option<bool> {
        self.allow_egress_from_local_classic_link_to_remote_vpc
    }
    /// <p>Indicates whether a local VPC can communicate with a ClassicLink connection in the peer VPC over the VPC peering connection.</p>
    pub fn allow_egress_from_local_vpc_to_remote_classic_link(&self) -> std::option::Option<bool> {
        self.allow_egress_from_local_vpc_to_remote_classic_link
    }
}
impl VpcPeeringConnectionOptionsDescription {
    /// Creates a new builder-style object to manufacture [`VpcPeeringConnectionOptionsDescription`](crate::types::VpcPeeringConnectionOptionsDescription).
    pub fn builder() -> crate::types::builders::VpcPeeringConnectionOptionsDescriptionBuilder {
        crate::types::builders::VpcPeeringConnectionOptionsDescriptionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::VpcPeeringConnectionOptionsDescription;
/// A builder for [`VpcPeeringConnectionOptionsDescription`](crate::types::VpcPeeringConnectionOptionsDescription).
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
pub struct VpcPeeringConnectionOptionsDescriptionBuilder {
    pub(crate) allow_dns_resolution_from_remote_vpc: std::option::Option<bool>,
    pub(crate) allow_egress_from_local_classic_link_to_remote_vpc: std::option::Option<bool>,
    pub(crate) allow_egress_from_local_vpc_to_remote_classic_link: std::option::Option<bool>,
}
impl VpcPeeringConnectionOptionsDescriptionBuilder {
    /// <p>Indicates whether a local VPC can resolve public DNS hostnames to private IP addresses when queried from instances in a peer VPC.</p>
    pub fn allow_dns_resolution_from_remote_vpc(mut self, input: bool) -> Self {
        self.allow_dns_resolution_from_remote_vpc = Some(input);
        self
    }
    /// <p>Indicates whether a local VPC can resolve public DNS hostnames to private IP addresses when queried from instances in a peer VPC.</p>
    pub fn set_allow_dns_resolution_from_remote_vpc(
        mut self,
        input: std::option::Option<bool>,
    ) -> Self {
        self.allow_dns_resolution_from_remote_vpc = input;
        self
    }
    /// <p>Indicates whether a local ClassicLink connection can communicate with the peer VPC over the VPC peering connection.</p>
    pub fn allow_egress_from_local_classic_link_to_remote_vpc(mut self, input: bool) -> Self {
        self.allow_egress_from_local_classic_link_to_remote_vpc = Some(input);
        self
    }
    /// <p>Indicates whether a local ClassicLink connection can communicate with the peer VPC over the VPC peering connection.</p>
    pub fn set_allow_egress_from_local_classic_link_to_remote_vpc(
        mut self,
        input: std::option::Option<bool>,
    ) -> Self {
        self.allow_egress_from_local_classic_link_to_remote_vpc = input;
        self
    }
    /// <p>Indicates whether a local VPC can communicate with a ClassicLink connection in the peer VPC over the VPC peering connection.</p>
    pub fn allow_egress_from_local_vpc_to_remote_classic_link(mut self, input: bool) -> Self {
        self.allow_egress_from_local_vpc_to_remote_classic_link = Some(input);
        self
    }
    /// <p>Indicates whether a local VPC can communicate with a ClassicLink connection in the peer VPC over the VPC peering connection.</p>
    pub fn set_allow_egress_from_local_vpc_to_remote_classic_link(
        mut self,
        input: std::option::Option<bool>,
    ) -> Self {
        self.allow_egress_from_local_vpc_to_remote_classic_link = input;
        self
    }
    /// Consumes the builder and constructs a [`VpcPeeringConnectionOptionsDescription`](crate::types::VpcPeeringConnectionOptionsDescription).
    pub fn build(self) -> crate::types::VpcPeeringConnectionOptionsDescription {
        crate::types::VpcPeeringConnectionOptionsDescription {
            allow_dns_resolution_from_remote_vpc: self.allow_dns_resolution_from_remote_vpc,
            allow_egress_from_local_classic_link_to_remote_vpc: self
                .allow_egress_from_local_classic_link_to_remote_vpc,
            allow_egress_from_local_vpc_to_remote_classic_link: self
                .allow_egress_from_local_vpc_to_remote_classic_link,
        }
    }
}