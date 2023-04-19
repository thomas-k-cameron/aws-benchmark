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
pub struct DeprovisionIpamPoolCidrInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The ID of the pool that has the CIDR you want to deprovision.</p>
    #[doc(hidden)]
    pub ipam_pool_id: std::option::Option<std::string::String>,
    /// <p>The CIDR which you want to deprovision from the pool.</p>
    #[doc(hidden)]
    pub cidr: std::option::Option<std::string::String>,
}
impl DeprovisionIpamPoolCidrInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the pool that has the CIDR you want to deprovision.</p>
    pub fn ipam_pool_id(&self) -> std::option::Option<&str> {
        self.ipam_pool_id.as_deref()
    }
    /// <p>The CIDR which you want to deprovision from the pool.</p>
    pub fn cidr(&self) -> std::option::Option<&str> {
        self.cidr.as_deref()
    }
}
impl DeprovisionIpamPoolCidrInput {
    /// Creates a new builder-style object to manufacture [`DeprovisionIpamPoolCidrInput`](crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrInput).
    pub fn builder(
    ) -> crate::operation::deprovision_ipam_pool_cidr::builders::DeprovisionIpamPoolCidrInputBuilder
    {
        crate::operation::deprovision_ipam_pool_cidr::builders::DeprovisionIpamPoolCidrInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrInput;
/// A builder for [`DeprovisionIpamPoolCidrInput`](crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrInput).
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
pub struct DeprovisionIpamPoolCidrInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) ipam_pool_id: std::option::Option<std::string::String>,
    pub(crate) cidr: std::option::Option<std::string::String>,
}
impl DeprovisionIpamPoolCidrInputBuilder {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>The ID of the pool that has the CIDR you want to deprovision.</p>
    pub fn ipam_pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.ipam_pool_id = Some(input.into());
        self
    }
    /// <p>The ID of the pool that has the CIDR you want to deprovision.</p>
    pub fn set_ipam_pool_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ipam_pool_id = input;
        self
    }
    /// <p>The CIDR which you want to deprovision from the pool.</p>
    pub fn cidr(mut self, input: impl Into<std::string::String>) -> Self {
        self.cidr = Some(input.into());
        self
    }
    /// <p>The CIDR which you want to deprovision from the pool.</p>
    pub fn set_cidr(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.cidr = input;
        self
    }
    /// Consumes the builder and constructs a [`DeprovisionIpamPoolCidrInput`](crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrInput {
                dry_run: self.dry_run,
                ipam_pool_id: self.ipam_pool_id,
                cidr: self.cidr,
            },
        )
    }
}