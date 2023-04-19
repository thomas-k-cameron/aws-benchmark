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
pub struct AssociateTransitGatewayMulticastDomainOutput {
    /// <p>Information about the transit gateway multicast domain associations.</p>
    #[doc(hidden)]
    pub associations: std::option::Option<crate::types::TransitGatewayMulticastDomainAssociations>,
    _request_id: Option<String>,
}
impl AssociateTransitGatewayMulticastDomainOutput {
    /// <p>Information about the transit gateway multicast domain associations.</p>
    pub fn associations(
        &self,
    ) -> std::option::Option<&crate::types::TransitGatewayMulticastDomainAssociations> {
        self.associations.as_ref()
    }
}
impl aws_http::request_id::RequestId for AssociateTransitGatewayMulticastDomainOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AssociateTransitGatewayMulticastDomainOutput {
    /// Creates a new builder-style object to manufacture [`AssociateTransitGatewayMulticastDomainOutput`](crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainOutput).
    pub fn builder() -> crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainOutputBuilder{
        crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainOutput;
/// A builder for [`AssociateTransitGatewayMulticastDomainOutput`](crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainOutput).
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
pub struct AssociateTransitGatewayMulticastDomainOutputBuilder {
    pub(crate) associations:
        std::option::Option<crate::types::TransitGatewayMulticastDomainAssociations>,
    _request_id: Option<String>,
}
impl AssociateTransitGatewayMulticastDomainOutputBuilder {
    /// <p>Information about the transit gateway multicast domain associations.</p>
    pub fn associations(
        mut self,
        input: crate::types::TransitGatewayMulticastDomainAssociations,
    ) -> Self {
        self.associations = Some(input);
        self
    }
    /// <p>Information about the transit gateway multicast domain associations.</p>
    pub fn set_associations(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayMulticastDomainAssociations>,
    ) -> Self {
        self.associations = input;
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
    /// Consumes the builder and constructs a [`AssociateTransitGatewayMulticastDomainOutput`](crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainOutput).
    pub fn build(self) -> crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainOutput{
        crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainOutput {
            associations: self.associations
            ,
            _request_id: self._request_id,
        }
    }
}