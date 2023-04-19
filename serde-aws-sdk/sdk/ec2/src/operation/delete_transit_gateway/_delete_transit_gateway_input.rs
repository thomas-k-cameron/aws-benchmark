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
pub struct DeleteTransitGatewayInput {
    /// <p>The ID of the transit gateway.</p>
    #[doc(hidden)]
    pub transit_gateway_id: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DeleteTransitGatewayInput {
    /// <p>The ID of the transit gateway.</p>
    pub fn transit_gateway_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DeleteTransitGatewayInput {
    /// Creates a new builder-style object to manufacture [`DeleteTransitGatewayInput`](crate::operation::delete_transit_gateway::DeleteTransitGatewayInput).
    pub fn builder(
    ) -> crate::operation::delete_transit_gateway::builders::DeleteTransitGatewayInputBuilder {
        crate::operation::delete_transit_gateway::builders::DeleteTransitGatewayInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_transit_gateway::DeleteTransitGatewayInput;
/// A builder for [`DeleteTransitGatewayInput`](crate::operation::delete_transit_gateway::DeleteTransitGatewayInput).
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
pub struct DeleteTransitGatewayInputBuilder {
    pub(crate) transit_gateway_id: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DeleteTransitGatewayInputBuilder {
    /// <p>The ID of the transit gateway.</p>
    pub fn transit_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.transit_gateway_id = Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn set_transit_gateway_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transit_gateway_id = input;
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
    /// Consumes the builder and constructs a [`DeleteTransitGatewayInput`](crate::operation::delete_transit_gateway::DeleteTransitGatewayInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_transit_gateway::DeleteTransitGatewayInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_transit_gateway::DeleteTransitGatewayInput {
                transit_gateway_id: self.transit_gateway_id,
                dry_run: self.dry_run,
            },
        )
    }
}