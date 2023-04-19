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
pub struct RejectTransitGatewayVpcAttachmentOutput {
    /// <p>Information about the attachment.</p>
    #[doc(hidden)]
    pub transit_gateway_vpc_attachment:
        std::option::Option<crate::types::TransitGatewayVpcAttachment>,
    _request_id: Option<String>,
}
impl RejectTransitGatewayVpcAttachmentOutput {
    /// <p>Information about the attachment.</p>
    pub fn transit_gateway_vpc_attachment(
        &self,
    ) -> std::option::Option<&crate::types::TransitGatewayVpcAttachment> {
        self.transit_gateway_vpc_attachment.as_ref()
    }
}
impl aws_http::request_id::RequestId for RejectTransitGatewayVpcAttachmentOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RejectTransitGatewayVpcAttachmentOutput {
    /// Creates a new builder-style object to manufacture [`RejectTransitGatewayVpcAttachmentOutput`](crate::operation::reject_transit_gateway_vpc_attachment::RejectTransitGatewayVpcAttachmentOutput).
    pub fn builder() -> crate::operation::reject_transit_gateway_vpc_attachment::builders::RejectTransitGatewayVpcAttachmentOutputBuilder{
        crate::operation::reject_transit_gateway_vpc_attachment::builders::RejectTransitGatewayVpcAttachmentOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::reject_transit_gateway_vpc_attachment::RejectTransitGatewayVpcAttachmentOutput;
/// A builder for [`RejectTransitGatewayVpcAttachmentOutput`](crate::operation::reject_transit_gateway_vpc_attachment::RejectTransitGatewayVpcAttachmentOutput).
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
pub struct RejectTransitGatewayVpcAttachmentOutputBuilder {
    pub(crate) transit_gateway_vpc_attachment:
        std::option::Option<crate::types::TransitGatewayVpcAttachment>,
    _request_id: Option<String>,
}
impl RejectTransitGatewayVpcAttachmentOutputBuilder {
    /// <p>Information about the attachment.</p>
    pub fn transit_gateway_vpc_attachment(
        mut self,
        input: crate::types::TransitGatewayVpcAttachment,
    ) -> Self {
        self.transit_gateway_vpc_attachment = Some(input);
        self
    }
    /// <p>Information about the attachment.</p>
    pub fn set_transit_gateway_vpc_attachment(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayVpcAttachment>,
    ) -> Self {
        self.transit_gateway_vpc_attachment = input;
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
    /// Consumes the builder and constructs a [`RejectTransitGatewayVpcAttachmentOutput`](crate::operation::reject_transit_gateway_vpc_attachment::RejectTransitGatewayVpcAttachmentOutput).
    pub fn build(self) -> crate::operation::reject_transit_gateway_vpc_attachment::RejectTransitGatewayVpcAttachmentOutput{
        crate::operation::reject_transit_gateway_vpc_attachment::RejectTransitGatewayVpcAttachmentOutput {
            transit_gateway_vpc_attachment: self.transit_gateway_vpc_attachment
            ,
            _request_id: self._request_id,
        }
    }
}