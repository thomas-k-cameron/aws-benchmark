// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::replace_transit_gateway_route::_replace_transit_gateway_route_output::ReplaceTransitGatewayRouteOutputBuilder;

pub use crate::operation::replace_transit_gateway_route::_replace_transit_gateway_route_input::ReplaceTransitGatewayRouteInputBuilder;

/// Fluent builder constructing a request to `ReplaceTransitGatewayRoute`.
///
/// <p>Replaces the specified route in the specified transit gateway route table.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ReplaceTransitGatewayRouteFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::replace_transit_gateway_route::builders::ReplaceTransitGatewayRouteInputBuilder
            }
impl ReplaceTransitGatewayRouteFluentBuilder {
    /// Creates a new `ReplaceTransitGatewayRoute`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::replace_transit_gateway_route::ReplaceTransitGatewayRoute,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::replace_transit_gateway_route::ReplaceTransitGatewayRouteError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::replace_transit_gateway_route::ReplaceTransitGatewayRouteOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::replace_transit_gateway_route::ReplaceTransitGatewayRouteError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::replace_transit_gateway_route::builders::ReplaceTransitGatewayRouteInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.replace_transit_gateway_route().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::replace_transit_gateway_route::builders::ReplaceTransitGatewayRouteInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The CIDR range used for the destination match. Routing decisions are based on the most specific match.</p>
    pub fn destination_cidr_block(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.destination_cidr_block(input.into());
        self
    }
    /// <p>The CIDR range used for the destination match. Routing decisions are based on the most specific match.</p>
    pub fn set_destination_cidr_block(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_destination_cidr_block(input);
        self
    }
    /// <p>The ID of the route table.</p>
    pub fn transit_gateway_route_table_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.transit_gateway_route_table_id(input.into());
        self
    }
    /// <p>The ID of the route table.</p>
    pub fn set_transit_gateway_route_table_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_transit_gateway_route_table_id(input);
        self
    }
    /// <p>The ID of the attachment.</p>
    pub fn transit_gateway_attachment_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.transit_gateway_attachment_id(input.into());
        self
    }
    /// <p>The ID of the attachment.</p>
    pub fn set_transit_gateway_attachment_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_transit_gateway_attachment_id(input);
        self
    }
    /// <p>Indicates whether traffic matching this route is to be dropped.</p>
    pub fn blackhole(mut self, input: bool) -> Self {
        self.inner = self.inner.blackhole(input);
        self
    }
    /// <p>Indicates whether traffic matching this route is to be dropped.</p>
    pub fn set_blackhole(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_blackhole(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
}