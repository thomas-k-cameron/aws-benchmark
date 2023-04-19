// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateTransitGatewayRoute`](crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`destination_cidr_block(impl Into<String>)`](crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteFluentBuilder::destination_cidr_block) / [`set_destination_cidr_block(Option<String>)`](crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteFluentBuilder::set_destination_cidr_block): <p>The CIDR range used for destination matches. Routing decisions are based on the most specific match.</p>
    ///   - [`transit_gateway_route_table_id(impl Into<String>)`](crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteFluentBuilder::transit_gateway_route_table_id) / [`set_transit_gateway_route_table_id(Option<String>)`](crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteFluentBuilder::set_transit_gateway_route_table_id): <p>The ID of the transit gateway route table.</p>
    ///   - [`transit_gateway_attachment_id(impl Into<String>)`](crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteFluentBuilder::transit_gateway_attachment_id) / [`set_transit_gateway_attachment_id(Option<String>)`](crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteFluentBuilder::set_transit_gateway_attachment_id): <p>The ID of the attachment.</p>
    ///   - [`blackhole(bool)`](crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteFluentBuilder::blackhole) / [`set_blackhole(Option<bool>)`](crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteFluentBuilder::set_blackhole): <p>Indicates whether to drop traffic that matches this route.</p>
    ///   - [`dry_run(bool)`](crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`CreateTransitGatewayRouteOutput`](crate::operation::create_transit_gateway_route::CreateTransitGatewayRouteOutput) with field(s):
    ///   - [`route(Option<TransitGatewayRoute>)`](crate::operation::create_transit_gateway_route::CreateTransitGatewayRouteOutput::route): <p>Information about the route.</p>
    /// - On failure, responds with [`SdkError<CreateTransitGatewayRouteError>`](crate::operation::create_transit_gateway_route::CreateTransitGatewayRouteError)
    pub fn create_transit_gateway_route(&self) -> crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteFluentBuilder{
        crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteFluentBuilder::new(self.handle.clone())
    }
}