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
pub struct DeleteTransitGatewayPeeringAttachmentInput {
    /// <p>The ID of the transit gateway peering attachment.</p>
    #[doc(hidden)]
    pub transit_gateway_attachment_id: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DeleteTransitGatewayPeeringAttachmentInput {
    /// <p>The ID of the transit gateway peering attachment.</p>
    pub fn transit_gateway_attachment_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_attachment_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DeleteTransitGatewayPeeringAttachmentInput {
    /// Creates a new builder-style object to manufacture [`DeleteTransitGatewayPeeringAttachmentInput`](crate::operation::delete_transit_gateway_peering_attachment::DeleteTransitGatewayPeeringAttachmentInput).
    pub fn builder() -> crate::operation::delete_transit_gateway_peering_attachment::builders::DeleteTransitGatewayPeeringAttachmentInputBuilder{
        crate::operation::delete_transit_gateway_peering_attachment::builders::DeleteTransitGatewayPeeringAttachmentInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_transit_gateway_peering_attachment::DeleteTransitGatewayPeeringAttachmentInput;
/// A builder for [`DeleteTransitGatewayPeeringAttachmentInput`](crate::operation::delete_transit_gateway_peering_attachment::DeleteTransitGatewayPeeringAttachmentInput).
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
pub struct DeleteTransitGatewayPeeringAttachmentInputBuilder {
    pub(crate) transit_gateway_attachment_id: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DeleteTransitGatewayPeeringAttachmentInputBuilder {
    /// <p>The ID of the transit gateway peering attachment.</p>
    pub fn transit_gateway_attachment_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.transit_gateway_attachment_id = Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway peering attachment.</p>
    pub fn set_transit_gateway_attachment_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transit_gateway_attachment_id = input;
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
    /// Consumes the builder and constructs a [`DeleteTransitGatewayPeeringAttachmentInput`](crate::operation::delete_transit_gateway_peering_attachment::DeleteTransitGatewayPeeringAttachmentInput).
    pub fn build(self) -> Result<crate::operation::delete_transit_gateway_peering_attachment::DeleteTransitGatewayPeeringAttachmentInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::delete_transit_gateway_peering_attachment::DeleteTransitGatewayPeeringAttachmentInput {
                transit_gateway_attachment_id: self.transit_gateway_attachment_id
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}