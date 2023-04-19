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
pub struct DeleteIpamInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The ID of the IPAM to delete.</p>
    #[doc(hidden)]
    pub ipam_id: std::option::Option<std::string::String>,
    /// <p>Enables you to quickly delete an IPAM, private scopes, pools in private scopes, and any allocations in the pools in private scopes. You cannot delete the IPAM with this option if there is a pool in your public scope. If you use this option, IPAM does the following:</p>
    /// <ul>
    /// <li> <p>Deallocates any CIDRs allocated to VPC resources (such as VPCs) in pools in private scopes.</p> <note>
    /// <p>No VPC resources are deleted as a result of enabling this option. The CIDR associated with the resource will no longer be allocated from an IPAM pool, but the CIDR itself will remain unchanged.</p>
    /// </note> </li>
    /// <li> <p>Deprovisions all IPv4 CIDRs provisioned to IPAM pools in private scopes.</p> </li>
    /// <li> <p>Deletes all IPAM pools in private scopes.</p> </li>
    /// <li> <p>Deletes all non-default private scopes in the IPAM.</p> </li>
    /// <li> <p>Deletes the default public and private scopes and the IPAM.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub cascade: std::option::Option<bool>,
}
impl DeleteIpamInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the IPAM to delete.</p>
    pub fn ipam_id(&self) -> std::option::Option<&str> {
        self.ipam_id.as_deref()
    }
    /// <p>Enables you to quickly delete an IPAM, private scopes, pools in private scopes, and any allocations in the pools in private scopes. You cannot delete the IPAM with this option if there is a pool in your public scope. If you use this option, IPAM does the following:</p>
    /// <ul>
    /// <li> <p>Deallocates any CIDRs allocated to VPC resources (such as VPCs) in pools in private scopes.</p> <note>
    /// <p>No VPC resources are deleted as a result of enabling this option. The CIDR associated with the resource will no longer be allocated from an IPAM pool, but the CIDR itself will remain unchanged.</p>
    /// </note> </li>
    /// <li> <p>Deprovisions all IPv4 CIDRs provisioned to IPAM pools in private scopes.</p> </li>
    /// <li> <p>Deletes all IPAM pools in private scopes.</p> </li>
    /// <li> <p>Deletes all non-default private scopes in the IPAM.</p> </li>
    /// <li> <p>Deletes the default public and private scopes and the IPAM.</p> </li>
    /// </ul>
    pub fn cascade(&self) -> std::option::Option<bool> {
        self.cascade
    }
}
impl DeleteIpamInput {
    /// Creates a new builder-style object to manufacture [`DeleteIpamInput`](crate::operation::delete_ipam::DeleteIpamInput).
    pub fn builder() -> crate::operation::delete_ipam::builders::DeleteIpamInputBuilder {
        crate::operation::delete_ipam::builders::DeleteIpamInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_ipam::DeleteIpamInput;
/// A builder for [`DeleteIpamInput`](crate::operation::delete_ipam::DeleteIpamInput).
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
pub struct DeleteIpamInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) ipam_id: std::option::Option<std::string::String>,
    pub(crate) cascade: std::option::Option<bool>,
}
impl DeleteIpamInputBuilder {
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
    /// <p>The ID of the IPAM to delete.</p>
    pub fn ipam_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.ipam_id = Some(input.into());
        self
    }
    /// <p>The ID of the IPAM to delete.</p>
    pub fn set_ipam_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ipam_id = input;
        self
    }
    /// <p>Enables you to quickly delete an IPAM, private scopes, pools in private scopes, and any allocations in the pools in private scopes. You cannot delete the IPAM with this option if there is a pool in your public scope. If you use this option, IPAM does the following:</p>
    /// <ul>
    /// <li> <p>Deallocates any CIDRs allocated to VPC resources (such as VPCs) in pools in private scopes.</p> <note>
    /// <p>No VPC resources are deleted as a result of enabling this option. The CIDR associated with the resource will no longer be allocated from an IPAM pool, but the CIDR itself will remain unchanged.</p>
    /// </note> </li>
    /// <li> <p>Deprovisions all IPv4 CIDRs provisioned to IPAM pools in private scopes.</p> </li>
    /// <li> <p>Deletes all IPAM pools in private scopes.</p> </li>
    /// <li> <p>Deletes all non-default private scopes in the IPAM.</p> </li>
    /// <li> <p>Deletes the default public and private scopes and the IPAM.</p> </li>
    /// </ul>
    pub fn cascade(mut self, input: bool) -> Self {
        self.cascade = Some(input);
        self
    }
    /// <p>Enables you to quickly delete an IPAM, private scopes, pools in private scopes, and any allocations in the pools in private scopes. You cannot delete the IPAM with this option if there is a pool in your public scope. If you use this option, IPAM does the following:</p>
    /// <ul>
    /// <li> <p>Deallocates any CIDRs allocated to VPC resources (such as VPCs) in pools in private scopes.</p> <note>
    /// <p>No VPC resources are deleted as a result of enabling this option. The CIDR associated with the resource will no longer be allocated from an IPAM pool, but the CIDR itself will remain unchanged.</p>
    /// </note> </li>
    /// <li> <p>Deprovisions all IPv4 CIDRs provisioned to IPAM pools in private scopes.</p> </li>
    /// <li> <p>Deletes all IPAM pools in private scopes.</p> </li>
    /// <li> <p>Deletes all non-default private scopes in the IPAM.</p> </li>
    /// <li> <p>Deletes the default public and private scopes and the IPAM.</p> </li>
    /// </ul>
    pub fn set_cascade(mut self, input: std::option::Option<bool>) -> Self {
        self.cascade = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteIpamInput`](crate::operation::delete_ipam::DeleteIpamInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_ipam::DeleteIpamInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::delete_ipam::DeleteIpamInput {
            dry_run: self.dry_run,
            ipam_id: self.ipam_id,
            cascade: self.cascade,
        })
    }
}