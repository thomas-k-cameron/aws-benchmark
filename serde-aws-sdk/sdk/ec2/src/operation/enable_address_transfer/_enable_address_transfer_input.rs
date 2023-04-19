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
pub struct EnableAddressTransferInput {
    /// <p>The allocation ID of an Elastic IP address.</p>
    #[doc(hidden)]
    pub allocation_id: std::option::Option<std::string::String>,
    /// <p>The ID of the account that you want to transfer the Elastic IP address to.</p>
    #[doc(hidden)]
    pub transfer_account_id: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl EnableAddressTransferInput {
    /// <p>The allocation ID of an Elastic IP address.</p>
    pub fn allocation_id(&self) -> std::option::Option<&str> {
        self.allocation_id.as_deref()
    }
    /// <p>The ID of the account that you want to transfer the Elastic IP address to.</p>
    pub fn transfer_account_id(&self) -> std::option::Option<&str> {
        self.transfer_account_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl EnableAddressTransferInput {
    /// Creates a new builder-style object to manufacture [`EnableAddressTransferInput`](crate::operation::enable_address_transfer::EnableAddressTransferInput).
    pub fn builder(
    ) -> crate::operation::enable_address_transfer::builders::EnableAddressTransferInputBuilder
    {
        crate::operation::enable_address_transfer::builders::EnableAddressTransferInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::enable_address_transfer::EnableAddressTransferInput;
/// A builder for [`EnableAddressTransferInput`](crate::operation::enable_address_transfer::EnableAddressTransferInput).
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
pub struct EnableAddressTransferInputBuilder {
    pub(crate) allocation_id: std::option::Option<std::string::String>,
    pub(crate) transfer_account_id: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl EnableAddressTransferInputBuilder {
    /// <p>The allocation ID of an Elastic IP address.</p>
    pub fn allocation_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.allocation_id = Some(input.into());
        self
    }
    /// <p>The allocation ID of an Elastic IP address.</p>
    pub fn set_allocation_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.allocation_id = input;
        self
    }
    /// <p>The ID of the account that you want to transfer the Elastic IP address to.</p>
    pub fn transfer_account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.transfer_account_id = Some(input.into());
        self
    }
    /// <p>The ID of the account that you want to transfer the Elastic IP address to.</p>
    pub fn set_transfer_account_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transfer_account_id = input;
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
    /// Consumes the builder and constructs a [`EnableAddressTransferInput`](crate::operation::enable_address_transfer::EnableAddressTransferInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::enable_address_transfer::EnableAddressTransferInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::enable_address_transfer::EnableAddressTransferInput {
                allocation_id: self.allocation_id,
                transfer_account_id: self.transfer_account_id,
                dry_run: self.dry_run,
            },
        )
    }
}