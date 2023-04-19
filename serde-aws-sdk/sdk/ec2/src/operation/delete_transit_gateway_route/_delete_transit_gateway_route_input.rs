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
pub struct DeleteTransitGatewayRouteInput {
    /// <p>The ID of the transit gateway route table.</p>
    #[doc(hidden)]
    pub transit_gateway_route_table_id: std::option::Option<std::string::String>,
    /// <p>The CIDR range for the route. This must match the CIDR for the route exactly.</p>
    #[doc(hidden)]
    pub destination_cidr_block: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DeleteTransitGatewayRouteInput {
    /// <p>The ID of the transit gateway route table.</p>
    pub fn transit_gateway_route_table_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_route_table_id.as_deref()
    }
    /// <p>The CIDR range for the route. This must match the CIDR for the route exactly.</p>
    pub fn destination_cidr_block(&self) -> std::option::Option<&str> {
        self.destination_cidr_block.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DeleteTransitGatewayRouteInput {
    /// Creates a new builder-style object to manufacture [`DeleteTransitGatewayRouteInput`](crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteInput).
    pub fn builder() -> crate::operation::delete_transit_gateway_route::builders::DeleteTransitGatewayRouteInputBuilder{
        crate::operation::delete_transit_gateway_route::builders::DeleteTransitGatewayRouteInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteInput;
/// A builder for [`DeleteTransitGatewayRouteInput`](crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteInput).
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
pub struct DeleteTransitGatewayRouteInputBuilder {
    pub(crate) transit_gateway_route_table_id: std::option::Option<std::string::String>,
    pub(crate) destination_cidr_block: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DeleteTransitGatewayRouteInputBuilder {
    /// <p>The ID of the transit gateway route table.</p>
    pub fn transit_gateway_route_table_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.transit_gateway_route_table_id = Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway route table.</p>
    pub fn set_transit_gateway_route_table_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transit_gateway_route_table_id = input;
        self
    }
    /// <p>The CIDR range for the route. This must match the CIDR for the route exactly.</p>
    pub fn destination_cidr_block(mut self, input: impl Into<std::string::String>) -> Self {
        self.destination_cidr_block = Some(input.into());
        self
    }
    /// <p>The CIDR range for the route. This must match the CIDR for the route exactly.</p>
    pub fn set_destination_cidr_block(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.destination_cidr_block = input;
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
    /// Consumes the builder and constructs a [`DeleteTransitGatewayRouteInput`](crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteInput {
                transit_gateway_route_table_id: self.transit_gateway_route_table_id,
                destination_cidr_block: self.destination_cidr_block,
                dry_run: self.dry_run,
            },
        )
    }
}