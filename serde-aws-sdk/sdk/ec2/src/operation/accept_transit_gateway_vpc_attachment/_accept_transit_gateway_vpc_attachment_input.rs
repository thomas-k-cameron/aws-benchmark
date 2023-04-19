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
pub struct AcceptTransitGatewayVpcAttachmentInput {
    /// <p>The ID of the attachment.</p>
    #[doc(hidden)]
    pub transit_gateway_attachment_id: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl AcceptTransitGatewayVpcAttachmentInput {
    /// <p>The ID of the attachment.</p>
    pub fn transit_gateway_attachment_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_attachment_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl AcceptTransitGatewayVpcAttachmentInput {
    /// Creates a new builder-style object to manufacture [`AcceptTransitGatewayVpcAttachmentInput`](crate::operation::accept_transit_gateway_vpc_attachment::AcceptTransitGatewayVpcAttachmentInput).
    pub fn builder() -> crate::operation::accept_transit_gateway_vpc_attachment::builders::AcceptTransitGatewayVpcAttachmentInputBuilder{
        crate::operation::accept_transit_gateway_vpc_attachment::builders::AcceptTransitGatewayVpcAttachmentInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::accept_transit_gateway_vpc_attachment::AcceptTransitGatewayVpcAttachmentInput;
/// A builder for [`AcceptTransitGatewayVpcAttachmentInput`](crate::operation::accept_transit_gateway_vpc_attachment::AcceptTransitGatewayVpcAttachmentInput).
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
pub struct AcceptTransitGatewayVpcAttachmentInputBuilder {
    pub(crate) transit_gateway_attachment_id: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl AcceptTransitGatewayVpcAttachmentInputBuilder {
    /// <p>The ID of the attachment.</p>
    pub fn transit_gateway_attachment_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.transit_gateway_attachment_id = Some(input.into());
        self
    }
    /// <p>The ID of the attachment.</p>
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
    /// Consumes the builder and constructs a [`AcceptTransitGatewayVpcAttachmentInput`](crate::operation::accept_transit_gateway_vpc_attachment::AcceptTransitGatewayVpcAttachmentInput).
    pub fn build(self) -> Result<crate::operation::accept_transit_gateway_vpc_attachment::AcceptTransitGatewayVpcAttachmentInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::accept_transit_gateway_vpc_attachment::AcceptTransitGatewayVpcAttachmentInput {
                transit_gateway_attachment_id: self.transit_gateway_attachment_id
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}