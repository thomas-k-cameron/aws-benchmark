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
pub struct CreateNetworkInsightsPathInput {
    /// <p>The IP address of the Amazon Web Services resource that is the source of the path.</p>
    #[doc(hidden)]
    pub source_ip: std::option::Option<std::string::String>,
    /// <p>The IP address of the Amazon Web Services resource that is the destination of the path.</p>
    #[doc(hidden)]
    pub destination_ip: std::option::Option<std::string::String>,
    /// <p>The Amazon Web Services resource that is the source of the path.</p>
    #[doc(hidden)]
    pub source: std::option::Option<std::string::String>,
    /// <p>The Amazon Web Services resource that is the destination of the path.</p>
    #[doc(hidden)]
    pub destination: std::option::Option<std::string::String>,
    /// <p>The protocol.</p>
    #[doc(hidden)]
    pub protocol: std::option::Option<crate::types::Protocol>,
    /// <p>The destination port.</p>
    #[doc(hidden)]
    pub destination_port: std::option::Option<i32>,
    /// <p>The tags to add to the path.</p>
    #[doc(hidden)]
    pub tag_specifications: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
}
impl CreateNetworkInsightsPathInput {
    /// <p>The IP address of the Amazon Web Services resource that is the source of the path.</p>
    pub fn source_ip(&self) -> std::option::Option<&str> {
        self.source_ip.as_deref()
    }
    /// <p>The IP address of the Amazon Web Services resource that is the destination of the path.</p>
    pub fn destination_ip(&self) -> std::option::Option<&str> {
        self.destination_ip.as_deref()
    }
    /// <p>The Amazon Web Services resource that is the source of the path.</p>
    pub fn source(&self) -> std::option::Option<&str> {
        self.source.as_deref()
    }
    /// <p>The Amazon Web Services resource that is the destination of the path.</p>
    pub fn destination(&self) -> std::option::Option<&str> {
        self.destination.as_deref()
    }
    /// <p>The protocol.</p>
    pub fn protocol(&self) -> std::option::Option<&crate::types::Protocol> {
        self.protocol.as_ref()
    }
    /// <p>The destination port.</p>
    pub fn destination_port(&self) -> std::option::Option<i32> {
        self.destination_port
    }
    /// <p>The tags to add to the path.</p>
    pub fn tag_specifications(&self) -> std::option::Option<&[crate::types::TagSpecification]> {
        self.tag_specifications.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
}
impl CreateNetworkInsightsPathInput {
    /// Creates a new builder-style object to manufacture [`CreateNetworkInsightsPathInput`](crate::operation::create_network_insights_path::CreateNetworkInsightsPathInput).
    pub fn builder() -> crate::operation::create_network_insights_path::builders::CreateNetworkInsightsPathInputBuilder{
        crate::operation::create_network_insights_path::builders::CreateNetworkInsightsPathInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::create_network_insights_path::CreateNetworkInsightsPathInput;
/// A builder for [`CreateNetworkInsightsPathInput`](crate::operation::create_network_insights_path::CreateNetworkInsightsPathInput).
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
pub struct CreateNetworkInsightsPathInputBuilder {
    pub(crate) source_ip: std::option::Option<std::string::String>,
    pub(crate) destination_ip: std::option::Option<std::string::String>,
    pub(crate) source: std::option::Option<std::string::String>,
    pub(crate) destination: std::option::Option<std::string::String>,
    pub(crate) protocol: std::option::Option<crate::types::Protocol>,
    pub(crate) destination_port: std::option::Option<i32>,
    pub(crate) tag_specifications:
        std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) client_token: std::option::Option<std::string::String>,
}
impl CreateNetworkInsightsPathInputBuilder {
    /// <p>The IP address of the Amazon Web Services resource that is the source of the path.</p>
    pub fn source_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.source_ip = Some(input.into());
        self
    }
    /// <p>The IP address of the Amazon Web Services resource that is the source of the path.</p>
    pub fn set_source_ip(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.source_ip = input;
        self
    }
    /// <p>The IP address of the Amazon Web Services resource that is the destination of the path.</p>
    pub fn destination_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.destination_ip = Some(input.into());
        self
    }
    /// <p>The IP address of the Amazon Web Services resource that is the destination of the path.</p>
    pub fn set_destination_ip(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.destination_ip = input;
        self
    }
    /// <p>The Amazon Web Services resource that is the source of the path.</p>
    pub fn source(mut self, input: impl Into<std::string::String>) -> Self {
        self.source = Some(input.into());
        self
    }
    /// <p>The Amazon Web Services resource that is the source of the path.</p>
    pub fn set_source(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.source = input;
        self
    }
    /// <p>The Amazon Web Services resource that is the destination of the path.</p>
    pub fn destination(mut self, input: impl Into<std::string::String>) -> Self {
        self.destination = Some(input.into());
        self
    }
    /// <p>The Amazon Web Services resource that is the destination of the path.</p>
    pub fn set_destination(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.destination = input;
        self
    }
    /// <p>The protocol.</p>
    pub fn protocol(mut self, input: crate::types::Protocol) -> Self {
        self.protocol = Some(input);
        self
    }
    /// <p>The protocol.</p>
    pub fn set_protocol(mut self, input: std::option::Option<crate::types::Protocol>) -> Self {
        self.protocol = input;
        self
    }
    /// <p>The destination port.</p>
    pub fn destination_port(mut self, input: i32) -> Self {
        self.destination_port = Some(input);
        self
    }
    /// <p>The destination port.</p>
    pub fn set_destination_port(mut self, input: std::option::Option<i32>) -> Self {
        self.destination_port = input;
        self
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to add to the path.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = Some(v);
        self
    }
    /// <p>The tags to add to the path.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.tag_specifications = input;
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateNetworkInsightsPathInput`](crate::operation::create_network_insights_path::CreateNetworkInsightsPathInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_network_insights_path::CreateNetworkInsightsPathInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_network_insights_path::CreateNetworkInsightsPathInput {
                source_ip: self.source_ip,
                destination_ip: self.destination_ip,
                source: self.source,
                destination: self.destination,
                protocol: self.protocol,
                destination_port: self.destination_port,
                tag_specifications: self.tag_specifications,
                dry_run: self.dry_run,
                client_token: self.client_token,
            },
        )
    }
}