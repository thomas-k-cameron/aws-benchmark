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
pub struct AssignPrivateNatGatewayAddressInput {
    /// <p>The NAT gateway ID.</p>
    #[doc(hidden)]
    pub nat_gateway_id: std::option::Option<std::string::String>,
    /// <p>The private IPv4 addresses you want to assign to the private NAT gateway.</p>
    #[doc(hidden)]
    pub private_ip_addresses: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The number of private IP addresses to assign to the NAT gateway. You can't specify this parameter when also specifying private IP addresses.</p>
    #[doc(hidden)]
    pub private_ip_address_count: std::option::Option<i32>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl AssignPrivateNatGatewayAddressInput {
    /// <p>The NAT gateway ID.</p>
    pub fn nat_gateway_id(&self) -> std::option::Option<&str> {
        self.nat_gateway_id.as_deref()
    }
    /// <p>The private IPv4 addresses you want to assign to the private NAT gateway.</p>
    pub fn private_ip_addresses(&self) -> std::option::Option<&[std::string::String]> {
        self.private_ip_addresses.as_deref()
    }
    /// <p>The number of private IP addresses to assign to the NAT gateway. You can't specify this parameter when also specifying private IP addresses.</p>
    pub fn private_ip_address_count(&self) -> std::option::Option<i32> {
        self.private_ip_address_count
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl AssignPrivateNatGatewayAddressInput {
    /// Creates a new builder-style object to manufacture [`AssignPrivateNatGatewayAddressInput`](crate::operation::assign_private_nat_gateway_address::AssignPrivateNatGatewayAddressInput).
    pub fn builder() -> crate::operation::assign_private_nat_gateway_address::builders::AssignPrivateNatGatewayAddressInputBuilder{
        crate::operation::assign_private_nat_gateway_address::builders::AssignPrivateNatGatewayAddressInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::assign_private_nat_gateway_address::AssignPrivateNatGatewayAddressInput;
/// A builder for [`AssignPrivateNatGatewayAddressInput`](crate::operation::assign_private_nat_gateway_address::AssignPrivateNatGatewayAddressInput).
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
pub struct AssignPrivateNatGatewayAddressInputBuilder {
    pub(crate) nat_gateway_id: std::option::Option<std::string::String>,
    pub(crate) private_ip_addresses: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) private_ip_address_count: std::option::Option<i32>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl AssignPrivateNatGatewayAddressInputBuilder {
    /// <p>The NAT gateway ID.</p>
    pub fn nat_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.nat_gateway_id = Some(input.into());
        self
    }
    /// <p>The NAT gateway ID.</p>
    pub fn set_nat_gateway_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.nat_gateway_id = input;
        self
    }
    /// Appends an item to `private_ip_addresses`.
    ///
    /// To override the contents of this collection use [`set_private_ip_addresses`](Self::set_private_ip_addresses).
    ///
    /// <p>The private IPv4 addresses you want to assign to the private NAT gateway.</p>
    pub fn private_ip_addresses(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.private_ip_addresses.unwrap_or_default();
        v.push(input.into());
        self.private_ip_addresses = Some(v);
        self
    }
    /// <p>The private IPv4 addresses you want to assign to the private NAT gateway.</p>
    pub fn set_private_ip_addresses(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.private_ip_addresses = input;
        self
    }
    /// <p>The number of private IP addresses to assign to the NAT gateway. You can't specify this parameter when also specifying private IP addresses.</p>
    pub fn private_ip_address_count(mut self, input: i32) -> Self {
        self.private_ip_address_count = Some(input);
        self
    }
    /// <p>The number of private IP addresses to assign to the NAT gateway. You can't specify this parameter when also specifying private IP addresses.</p>
    pub fn set_private_ip_address_count(mut self, input: std::option::Option<i32>) -> Self {
        self.private_ip_address_count = input;
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
    /// Consumes the builder and constructs a [`AssignPrivateNatGatewayAddressInput`](crate::operation::assign_private_nat_gateway_address::AssignPrivateNatGatewayAddressInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::assign_private_nat_gateway_address::AssignPrivateNatGatewayAddressInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::assign_private_nat_gateway_address::AssignPrivateNatGatewayAddressInput {
                nat_gateway_id: self.nat_gateway_id
                ,
                private_ip_addresses: self.private_ip_addresses
                ,
                private_ip_address_count: self.private_ip_address_count
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}