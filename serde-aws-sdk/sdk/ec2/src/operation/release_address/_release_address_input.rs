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
pub struct ReleaseAddressInput {
    /// <p>[EC2-VPC] The allocation ID. Required for EC2-VPC.</p>
    #[doc(hidden)]
    pub allocation_id: std::option::Option<std::string::String>,
    /// <p>[EC2-Classic] The Elastic IP address. Required for EC2-Classic.</p>
    #[doc(hidden)]
    pub public_ip: std::option::Option<std::string::String>,
    /// <p>The set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    /// <p>If you provide an incorrect network border group, you receive an <code>InvalidAddress.NotFound</code> error.</p>
    /// <p>You cannot use a network border group with EC2 Classic. If you attempt this operation on EC2 classic, you receive an <code>InvalidParameterCombination</code> error.</p>
    #[doc(hidden)]
    pub network_border_group: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl ReleaseAddressInput {
    /// <p>[EC2-VPC] The allocation ID. Required for EC2-VPC.</p>
    pub fn allocation_id(&self) -> std::option::Option<&str> {
        self.allocation_id.as_deref()
    }
    /// <p>[EC2-Classic] The Elastic IP address. Required for EC2-Classic.</p>
    pub fn public_ip(&self) -> std::option::Option<&str> {
        self.public_ip.as_deref()
    }
    /// <p>The set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    /// <p>If you provide an incorrect network border group, you receive an <code>InvalidAddress.NotFound</code> error.</p>
    /// <p>You cannot use a network border group with EC2 Classic. If you attempt this operation on EC2 classic, you receive an <code>InvalidParameterCombination</code> error.</p>
    pub fn network_border_group(&self) -> std::option::Option<&str> {
        self.network_border_group.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl ReleaseAddressInput {
    /// Creates a new builder-style object to manufacture [`ReleaseAddressInput`](crate::operation::release_address::ReleaseAddressInput).
    pub fn builder() -> crate::operation::release_address::builders::ReleaseAddressInputBuilder {
        crate::operation::release_address::builders::ReleaseAddressInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::release_address::ReleaseAddressInput;
/// A builder for [`ReleaseAddressInput`](crate::operation::release_address::ReleaseAddressInput).
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
pub struct ReleaseAddressInputBuilder {
    pub(crate) allocation_id: std::option::Option<std::string::String>,
    pub(crate) public_ip: std::option::Option<std::string::String>,
    pub(crate) network_border_group: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl ReleaseAddressInputBuilder {
    /// <p>[EC2-VPC] The allocation ID. Required for EC2-VPC.</p>
    pub fn allocation_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.allocation_id = Some(input.into());
        self
    }
    /// <p>[EC2-VPC] The allocation ID. Required for EC2-VPC.</p>
    pub fn set_allocation_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.allocation_id = input;
        self
    }
    /// <p>[EC2-Classic] The Elastic IP address. Required for EC2-Classic.</p>
    pub fn public_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.public_ip = Some(input.into());
        self
    }
    /// <p>[EC2-Classic] The Elastic IP address. Required for EC2-Classic.</p>
    pub fn set_public_ip(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.public_ip = input;
        self
    }
    /// <p>The set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    /// <p>If you provide an incorrect network border group, you receive an <code>InvalidAddress.NotFound</code> error.</p>
    /// <p>You cannot use a network border group with EC2 Classic. If you attempt this operation on EC2 classic, you receive an <code>InvalidParameterCombination</code> error.</p>
    pub fn network_border_group(mut self, input: impl Into<std::string::String>) -> Self {
        self.network_border_group = Some(input.into());
        self
    }
    /// <p>The set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    /// <p>If you provide an incorrect network border group, you receive an <code>InvalidAddress.NotFound</code> error.</p>
    /// <p>You cannot use a network border group with EC2 Classic. If you attempt this operation on EC2 classic, you receive an <code>InvalidParameterCombination</code> error.</p>
    pub fn set_network_border_group(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_border_group = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`ReleaseAddressInput`](crate::operation::release_address::ReleaseAddressInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::release_address::ReleaseAddressInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::release_address::ReleaseAddressInput {
            allocation_id: self.allocation_id,
            public_ip: self.public_ip,
            network_border_group: self.network_border_group,
            dry_run: self.dry_run,
        })
    }
}