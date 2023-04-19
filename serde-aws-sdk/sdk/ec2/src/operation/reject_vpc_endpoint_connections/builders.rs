// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::reject_vpc_endpoint_connections::_reject_vpc_endpoint_connections_output::RejectVpcEndpointConnectionsOutputBuilder;

pub use crate::operation::reject_vpc_endpoint_connections::_reject_vpc_endpoint_connections_input::RejectVpcEndpointConnectionsInputBuilder;

/// Fluent builder constructing a request to `RejectVpcEndpointConnections`.
///
/// <p>Rejects VPC endpoint connection requests to your VPC endpoint service.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RejectVpcEndpointConnectionsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::reject_vpc_endpoint_connections::builders::RejectVpcEndpointConnectionsInputBuilder
            }
impl RejectVpcEndpointConnectionsFluentBuilder {
    /// Creates a new `RejectVpcEndpointConnections`.
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
            crate::operation::reject_vpc_endpoint_connections::RejectVpcEndpointConnections,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::reject_vpc_endpoint_connections::RejectVpcEndpointConnectionsError,
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
        crate::operation::reject_vpc_endpoint_connections::RejectVpcEndpointConnectionsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::reject_vpc_endpoint_connections::RejectVpcEndpointConnectionsError,
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
    ///     let deserialized_parameters: crate::operation::reject_vpc_endpoint_connections::builders::RejectVpcEndpointConnectionsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.reject_vpc_endpoint_connections().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::reject_vpc_endpoint_connections::builders::RejectVpcEndpointConnectionsInputBuilder,
    ) -> Self {
        self.inner = data;
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
    /// <p>The ID of the service.</p>
    pub fn service_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.service_id(input.into());
        self
    }
    /// <p>The ID of the service.</p>
    pub fn set_service_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_service_id(input);
        self
    }
    /// Appends an item to `VpcEndpointIds`.
    ///
    /// To override the contents of this collection use [`set_vpc_endpoint_ids`](Self::set_vpc_endpoint_ids).
    ///
    /// <p>The IDs of the VPC endpoints.</p>
    pub fn vpc_endpoint_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.vpc_endpoint_ids(input.into());
        self
    }
    /// <p>The IDs of the VPC endpoints.</p>
    pub fn set_vpc_endpoint_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_vpc_endpoint_ids(input);
        self
    }
}