// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <note>
/// <p>We are retiring EC2-Classic. We recommend that you migrate from EC2-Classic to a VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-migrate.html">Migrate from EC2-Classic to a VPC</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
/// </note>
/// <p>The VPC peering connection options.</p>
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
pub struct PeeringConnectionOptionsRequest {
    /// <p>If true, enables a local VPC to resolve public DNS hostnames to private IP addresses when queried from instances in the peer VPC.</p>
    #[doc(hidden)]
    pub allow_dns_resolution_from_remote_vpc: std::option::Option<bool>,
    /// <p>If true, enables outbound communication from an EC2-Classic instance that's linked to a local VPC using ClassicLink to instances in a peer VPC.</p>
    #[doc(hidden)]
    pub allow_egress_from_local_classic_link_to_remote_vpc: std::option::Option<bool>,
    /// <p>If true, enables outbound communication from instances in a local VPC to an EC2-Classic instance that's linked to a peer VPC using ClassicLink.</p>
    #[doc(hidden)]
    pub allow_egress_from_local_vpc_to_remote_classic_link: std::option::Option<bool>,
}
impl PeeringConnectionOptionsRequest {
    /// <p>If true, enables a local VPC to resolve public DNS hostnames to private IP addresses when queried from instances in the peer VPC.</p>
    pub fn allow_dns_resolution_from_remote_vpc(&self) -> std::option::Option<bool> {
        self.allow_dns_resolution_from_remote_vpc
    }
    /// <p>If true, enables outbound communication from an EC2-Classic instance that's linked to a local VPC using ClassicLink to instances in a peer VPC.</p>
    pub fn allow_egress_from_local_classic_link_to_remote_vpc(&self) -> std::option::Option<bool> {
        self.allow_egress_from_local_classic_link_to_remote_vpc
    }
    /// <p>If true, enables outbound communication from instances in a local VPC to an EC2-Classic instance that's linked to a peer VPC using ClassicLink.</p>
    pub fn allow_egress_from_local_vpc_to_remote_classic_link(&self) -> std::option::Option<bool> {
        self.allow_egress_from_local_vpc_to_remote_classic_link
    }
}
impl PeeringConnectionOptionsRequest {
    /// Creates a new builder-style object to manufacture [`PeeringConnectionOptionsRequest`](crate::types::PeeringConnectionOptionsRequest).
    pub fn builder() -> crate::types::builders::PeeringConnectionOptionsRequestBuilder {
        crate::types::builders::PeeringConnectionOptionsRequestBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::PeeringConnectionOptionsRequest;
/// A builder for [`PeeringConnectionOptionsRequest`](crate::types::PeeringConnectionOptionsRequest).
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
pub struct PeeringConnectionOptionsRequestBuilder {
    pub(crate) allow_dns_resolution_from_remote_vpc: std::option::Option<bool>,
    pub(crate) allow_egress_from_local_classic_link_to_remote_vpc: std::option::Option<bool>,
    pub(crate) allow_egress_from_local_vpc_to_remote_classic_link: std::option::Option<bool>,
}
impl PeeringConnectionOptionsRequestBuilder {
    /// <p>If true, enables a local VPC to resolve public DNS hostnames to private IP addresses when queried from instances in the peer VPC.</p>
    pub fn allow_dns_resolution_from_remote_vpc(mut self, input: bool) -> Self {
        self.allow_dns_resolution_from_remote_vpc = Some(input);
        self
    }
    /// <p>If true, enables a local VPC to resolve public DNS hostnames to private IP addresses when queried from instances in the peer VPC.</p>
    pub fn set_allow_dns_resolution_from_remote_vpc(
        mut self,
        input: std::option::Option<bool>,
    ) -> Self {
        self.allow_dns_resolution_from_remote_vpc = input;
        self
    }
    /// <p>If true, enables outbound communication from an EC2-Classic instance that's linked to a local VPC using ClassicLink to instances in a peer VPC.</p>
    pub fn allow_egress_from_local_classic_link_to_remote_vpc(mut self, input: bool) -> Self {
        self.allow_egress_from_local_classic_link_to_remote_vpc = Some(input);
        self
    }
    /// <p>If true, enables outbound communication from an EC2-Classic instance that's linked to a local VPC using ClassicLink to instances in a peer VPC.</p>
    pub fn set_allow_egress_from_local_classic_link_to_remote_vpc(
        mut self,
        input: std::option::Option<bool>,
    ) -> Self {
        self.allow_egress_from_local_classic_link_to_remote_vpc = input;
        self
    }
    /// <p>If true, enables outbound communication from instances in a local VPC to an EC2-Classic instance that's linked to a peer VPC using ClassicLink.</p>
    pub fn allow_egress_from_local_vpc_to_remote_classic_link(mut self, input: bool) -> Self {
        self.allow_egress_from_local_vpc_to_remote_classic_link = Some(input);
        self
    }
    /// <p>If true, enables outbound communication from instances in a local VPC to an EC2-Classic instance that's linked to a peer VPC using ClassicLink.</p>
    pub fn set_allow_egress_from_local_vpc_to_remote_classic_link(
        mut self,
        input: std::option::Option<bool>,
    ) -> Self {
        self.allow_egress_from_local_vpc_to_remote_classic_link = input;
        self
    }
    /// Consumes the builder and constructs a [`PeeringConnectionOptionsRequest`](crate::types::PeeringConnectionOptionsRequest).
    pub fn build(self) -> crate::types::PeeringConnectionOptionsRequest {
        crate::types::PeeringConnectionOptionsRequest {
            allow_dns_resolution_from_remote_vpc: self.allow_dns_resolution_from_remote_vpc,
            allow_egress_from_local_classic_link_to_remote_vpc: self
                .allow_egress_from_local_classic_link_to_remote_vpc,
            allow_egress_from_local_vpc_to_remote_classic_link: self
                .allow_egress_from_local_vpc_to_remote_classic_link,
        }
    }
}