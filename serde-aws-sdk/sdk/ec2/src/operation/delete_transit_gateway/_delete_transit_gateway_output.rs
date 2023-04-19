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
pub struct DeleteTransitGatewayOutput {
    /// <p>Information about the deleted transit gateway.</p>
    #[doc(hidden)]
    pub transit_gateway: std::option::Option<crate::types::TransitGateway>,
    _request_id: Option<String>,
}
impl DeleteTransitGatewayOutput {
    /// <p>Information about the deleted transit gateway.</p>
    pub fn transit_gateway(&self) -> std::option::Option<&crate::types::TransitGateway> {
        self.transit_gateway.as_ref()
    }
}
impl aws_http::request_id::RequestId for DeleteTransitGatewayOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteTransitGatewayOutput {
    /// Creates a new builder-style object to manufacture [`DeleteTransitGatewayOutput`](crate::operation::delete_transit_gateway::DeleteTransitGatewayOutput).
    pub fn builder(
    ) -> crate::operation::delete_transit_gateway::builders::DeleteTransitGatewayOutputBuilder {
        crate::operation::delete_transit_gateway::builders::DeleteTransitGatewayOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_transit_gateway::DeleteTransitGatewayOutput;
/// A builder for [`DeleteTransitGatewayOutput`](crate::operation::delete_transit_gateway::DeleteTransitGatewayOutput).
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
pub struct DeleteTransitGatewayOutputBuilder {
    pub(crate) transit_gateway: std::option::Option<crate::types::TransitGateway>,
    _request_id: Option<String>,
}
impl DeleteTransitGatewayOutputBuilder {
    /// <p>Information about the deleted transit gateway.</p>
    pub fn transit_gateway(mut self, input: crate::types::TransitGateway) -> Self {
        self.transit_gateway = Some(input);
        self
    }
    /// <p>Information about the deleted transit gateway.</p>
    pub fn set_transit_gateway(
        mut self,
        input: std::option::Option<crate::types::TransitGateway>,
    ) -> Self {
        self.transit_gateway = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteTransitGatewayOutput`](crate::operation::delete_transit_gateway::DeleteTransitGatewayOutput).
    pub fn build(self) -> crate::operation::delete_transit_gateway::DeleteTransitGatewayOutput {
        crate::operation::delete_transit_gateway::DeleteTransitGatewayOutput {
            transit_gateway: self.transit_gateway,
            _request_id: self._request_id,
        }
    }
}