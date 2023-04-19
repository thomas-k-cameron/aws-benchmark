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
pub struct EnableIpamOrganizationAdminAccountInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The Organizations member account ID that you want to enable as the IPAM account.</p>
    #[doc(hidden)]
    pub delegated_admin_account_id: std::option::Option<std::string::String>,
}
impl EnableIpamOrganizationAdminAccountInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The Organizations member account ID that you want to enable as the IPAM account.</p>
    pub fn delegated_admin_account_id(&self) -> std::option::Option<&str> {
        self.delegated_admin_account_id.as_deref()
    }
}
impl EnableIpamOrganizationAdminAccountInput {
    /// Creates a new builder-style object to manufacture [`EnableIpamOrganizationAdminAccountInput`](crate::operation::enable_ipam_organization_admin_account::EnableIpamOrganizationAdminAccountInput).
    pub fn builder() -> crate::operation::enable_ipam_organization_admin_account::builders::EnableIpamOrganizationAdminAccountInputBuilder{
        crate::operation::enable_ipam_organization_admin_account::builders::EnableIpamOrganizationAdminAccountInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::enable_ipam_organization_admin_account::EnableIpamOrganizationAdminAccountInput;
/// A builder for [`EnableIpamOrganizationAdminAccountInput`](crate::operation::enable_ipam_organization_admin_account::EnableIpamOrganizationAdminAccountInput).
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
pub struct EnableIpamOrganizationAdminAccountInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) delegated_admin_account_id: std::option::Option<std::string::String>,
}
impl EnableIpamOrganizationAdminAccountInputBuilder {
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
    /// <p>The Organizations member account ID that you want to enable as the IPAM account.</p>
    pub fn delegated_admin_account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.delegated_admin_account_id = Some(input.into());
        self
    }
    /// <p>The Organizations member account ID that you want to enable as the IPAM account.</p>
    pub fn set_delegated_admin_account_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.delegated_admin_account_id = input;
        self
    }
    /// Consumes the builder and constructs a [`EnableIpamOrganizationAdminAccountInput`](crate::operation::enable_ipam_organization_admin_account::EnableIpamOrganizationAdminAccountInput).
    pub fn build(self) -> Result<crate::operation::enable_ipam_organization_admin_account::EnableIpamOrganizationAdminAccountInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::enable_ipam_organization_admin_account::EnableIpamOrganizationAdminAccountInput {
                dry_run: self.dry_run
                ,
                delegated_admin_account_id: self.delegated_admin_account_id
                ,
            }
        )
    }
}