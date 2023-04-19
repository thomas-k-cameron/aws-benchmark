// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a transit gateway Connect attachment.</p>
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
pub struct TransitGatewayConnect {
    /// <p>The ID of the Connect attachment.</p>
    #[doc(hidden)]
    pub transit_gateway_attachment_id: std::option::Option<std::string::String>,
    /// <p>The ID of the attachment from which the Connect attachment was created.</p>
    #[doc(hidden)]
    pub transport_transit_gateway_attachment_id: std::option::Option<std::string::String>,
    /// <p>The ID of the transit gateway.</p>
    #[doc(hidden)]
    pub transit_gateway_id: std::option::Option<std::string::String>,
    /// <p>The state of the attachment.</p>
    #[doc(hidden)]
    pub state: std::option::Option<crate::types::TransitGatewayAttachmentState>,
    /// <p>The creation time.</p>
    #[doc(hidden)]
    pub creation_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The Connect attachment options.</p>
    #[doc(hidden)]
    pub options: std::option::Option<crate::types::TransitGatewayConnectOptions>,
    /// <p>The tags for the attachment.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl TransitGatewayConnect {
    /// <p>The ID of the Connect attachment.</p>
    pub fn transit_gateway_attachment_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_attachment_id.as_deref()
    }
    /// <p>The ID of the attachment from which the Connect attachment was created.</p>
    pub fn transport_transit_gateway_attachment_id(&self) -> std::option::Option<&str> {
        self.transport_transit_gateway_attachment_id.as_deref()
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn transit_gateway_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_id.as_deref()
    }
    /// <p>The state of the attachment.</p>
    pub fn state(&self) -> std::option::Option<&crate::types::TransitGatewayAttachmentState> {
        self.state.as_ref()
    }
    /// <p>The creation time.</p>
    pub fn creation_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The Connect attachment options.</p>
    pub fn options(&self) -> std::option::Option<&crate::types::TransitGatewayConnectOptions> {
        self.options.as_ref()
    }
    /// <p>The tags for the attachment.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl TransitGatewayConnect {
    /// Creates a new builder-style object to manufacture [`TransitGatewayConnect`](crate::types::TransitGatewayConnect).
    pub fn builder() -> crate::types::builders::TransitGatewayConnectBuilder {
        crate::types::builders::TransitGatewayConnectBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TransitGatewayConnect;
/// A builder for [`TransitGatewayConnect`](crate::types::TransitGatewayConnect).
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
pub struct TransitGatewayConnectBuilder {
    pub(crate) transit_gateway_attachment_id: std::option::Option<std::string::String>,
    pub(crate) transport_transit_gateway_attachment_id: std::option::Option<std::string::String>,
    pub(crate) transit_gateway_id: std::option::Option<std::string::String>,
    pub(crate) state: std::option::Option<crate::types::TransitGatewayAttachmentState>,
    pub(crate) creation_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) options: std::option::Option<crate::types::TransitGatewayConnectOptions>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl TransitGatewayConnectBuilder {
    /// <p>The ID of the Connect attachment.</p>
    pub fn transit_gateway_attachment_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.transit_gateway_attachment_id = Some(input.into());
        self
    }
    /// <p>The ID of the Connect attachment.</p>
    pub fn set_transit_gateway_attachment_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transit_gateway_attachment_id = input;
        self
    }
    /// <p>The ID of the attachment from which the Connect attachment was created.</p>
    pub fn transport_transit_gateway_attachment_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.transport_transit_gateway_attachment_id = Some(input.into());
        self
    }
    /// <p>The ID of the attachment from which the Connect attachment was created.</p>
    pub fn set_transport_transit_gateway_attachment_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transport_transit_gateway_attachment_id = input;
        self
    }
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
    /// <p>The state of the attachment.</p>
    pub fn state(mut self, input: crate::types::TransitGatewayAttachmentState) -> Self {
        self.state = Some(input);
        self
    }
    /// <p>The state of the attachment.</p>
    pub fn set_state(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayAttachmentState>,
    ) -> Self {
        self.state = input;
        self
    }
    /// <p>The creation time.</p>
    pub fn creation_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.creation_time = Some(input);
        self
    }
    /// <p>The creation time.</p>
    pub fn set_creation_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The Connect attachment options.</p>
    pub fn options(mut self, input: crate::types::TransitGatewayConnectOptions) -> Self {
        self.options = Some(input);
        self
    }
    /// <p>The Connect attachment options.</p>
    pub fn set_options(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayConnectOptions>,
    ) -> Self {
        self.options = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags for the attachment.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>The tags for the attachment.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`TransitGatewayConnect`](crate::types::TransitGatewayConnect).
    pub fn build(self) -> crate::types::TransitGatewayConnect {
        crate::types::TransitGatewayConnect {
            transit_gateway_attachment_id: self.transit_gateway_attachment_id,
            transport_transit_gateway_attachment_id: self.transport_transit_gateway_attachment_id,
            transit_gateway_id: self.transit_gateway_id,
            state: self.state,
            creation_time: self.creation_time,
            options: self.options,
            tags: self.tags,
        }
    }
}