// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for DeleteNetworkInterface.</p>
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
pub struct DeleteNetworkInterfaceInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The ID of the network interface.</p>
    #[doc(hidden)]
    pub network_interface_id: std::option::Option<std::string::String>,
}
impl DeleteNetworkInterfaceInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(&self) -> std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
}
impl DeleteNetworkInterfaceInput {
    /// Creates a new builder-style object to manufacture [`DeleteNetworkInterfaceInput`](crate::operation::delete_network_interface::DeleteNetworkInterfaceInput).
    pub fn builder(
    ) -> crate::operation::delete_network_interface::builders::DeleteNetworkInterfaceInputBuilder
    {
        crate::operation::delete_network_interface::builders::DeleteNetworkInterfaceInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_network_interface::DeleteNetworkInterfaceInput;
/// A builder for [`DeleteNetworkInterfaceInput`](crate::operation::delete_network_interface::DeleteNetworkInterfaceInput).
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
pub struct DeleteNetworkInterfaceInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) network_interface_id: std::option::Option<std::string::String>,
}
impl DeleteNetworkInterfaceInputBuilder {
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
    /// Consumes the builder and constructs a [`DeleteNetworkInterfaceInput`](crate::operation::delete_network_interface::DeleteNetworkInterfaceInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_network_interface::DeleteNetworkInterfaceInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_network_interface::DeleteNetworkInterfaceInput {
                dry_run: self.dry_run,
                network_interface_id: self.network_interface_id,
            },
        )
    }
}