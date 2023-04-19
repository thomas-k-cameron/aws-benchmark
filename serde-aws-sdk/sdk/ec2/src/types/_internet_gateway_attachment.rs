// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the attachment of a VPC to an internet gateway or an egress-only internet gateway.</p>
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
pub struct InternetGatewayAttachment {
    /// <p>The current state of the attachment. For an internet gateway, the state is <code>available</code> when attached to a VPC; otherwise, this value is not returned.</p>
    #[doc(hidden)]
    pub state: std::option::Option<crate::types::AttachmentStatus>,
    /// <p>The ID of the VPC.</p>
    #[doc(hidden)]
    pub vpc_id: std::option::Option<std::string::String>,
}
impl InternetGatewayAttachment {
    /// <p>The current state of the attachment. For an internet gateway, the state is <code>available</code> when attached to a VPC; otherwise, this value is not returned.</p>
    pub fn state(&self) -> std::option::Option<&crate::types::AttachmentStatus> {
        self.state.as_ref()
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(&self) -> std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
}
impl InternetGatewayAttachment {
    /// Creates a new builder-style object to manufacture [`InternetGatewayAttachment`](crate::types::InternetGatewayAttachment).
    pub fn builder() -> crate::types::builders::InternetGatewayAttachmentBuilder {
        crate::types::builders::InternetGatewayAttachmentBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::InternetGatewayAttachment;
/// A builder for [`InternetGatewayAttachment`](crate::types::InternetGatewayAttachment).
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
pub struct InternetGatewayAttachmentBuilder {
    pub(crate) state: std::option::Option<crate::types::AttachmentStatus>,
    pub(crate) vpc_id: std::option::Option<std::string::String>,
}
impl InternetGatewayAttachmentBuilder {
    /// <p>The current state of the attachment. For an internet gateway, the state is <code>available</code> when attached to a VPC; otherwise, this value is not returned.</p>
    pub fn state(mut self, input: crate::types::AttachmentStatus) -> Self {
        self.state = Some(input);
        self
    }
    /// <p>The current state of the attachment. For an internet gateway, the state is <code>available</code> when attached to a VPC; otherwise, this value is not returned.</p>
    pub fn set_state(mut self, input: std::option::Option<crate::types::AttachmentStatus>) -> Self {
        self.state = input;
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.vpc_id = Some(input.into());
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn set_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// Consumes the builder and constructs a [`InternetGatewayAttachment`](crate::types::InternetGatewayAttachment).
    pub fn build(self) -> crate::types::InternetGatewayAttachment {
        crate::types::InternetGatewayAttachment {
            state: self.state,
            vpc_id: self.vpc_id,
        }
    }
}