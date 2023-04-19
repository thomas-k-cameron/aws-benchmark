// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::unassign_private_nat_gateway_address::_unassign_private_nat_gateway_address_output::UnassignPrivateNatGatewayAddressOutputBuilder;

pub use crate::operation::unassign_private_nat_gateway_address::_unassign_private_nat_gateway_address_input::UnassignPrivateNatGatewayAddressInputBuilder;

/// Fluent builder constructing a request to `UnassignPrivateNatGatewayAddress`.
///
/// <p>Unassigns secondary private IPv4 addresses from a private NAT gateway. You cannot unassign your primary private IP. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/vpc-nat-gateway.html#nat-gateway-edit-secondary">Edit secondary IP address associations</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
/// <p>While unassigning is in progress, you cannot assign/unassign additional IP addresses while the connections are being drained. You are, however, allowed to delete the NAT gateway.</p>
/// <p>A private IP address will only be released at the end of MaxDrainDurationSeconds. The private IP addresses stay associated and support the existing connections but do not support any new connections (new connections are distributed across the remaining assigned private IP address). After the existing connections drain out, the private IP addresses get released. </p>
/// <p></p>
/// <p></p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UnassignPrivateNatGatewayAddressFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::unassign_private_nat_gateway_address::builders::UnassignPrivateNatGatewayAddressInputBuilder
            }
impl UnassignPrivateNatGatewayAddressFluentBuilder {
    /// Creates a new `UnassignPrivateNatGatewayAddress`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::unassign_private_nat_gateway_address::UnassignPrivateNatGatewayAddress, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::unassign_private_nat_gateway_address::UnassignPrivateNatGatewayAddressError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::unassign_private_nat_gateway_address::UnassignPrivateNatGatewayAddressOutput, aws_smithy_http::result::SdkError<crate::operation::unassign_private_nat_gateway_address::UnassignPrivateNatGatewayAddressError>>
                     {
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
    ///     let deserialized_parameters: crate::operation::unassign_private_nat_gateway_address::builders::UnassignPrivateNatGatewayAddressInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.unassign_private_nat_gateway_address().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::unassign_private_nat_gateway_address::builders::UnassignPrivateNatGatewayAddressInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The NAT gateway ID.</p>
    pub fn nat_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.nat_gateway_id(input.into());
        self
    }
    /// <p>The NAT gateway ID.</p>
    pub fn set_nat_gateway_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_nat_gateway_id(input);
        self
    }
    /// Appends an item to `PrivateIpAddresses`.
    ///
    /// To override the contents of this collection use [`set_private_ip_addresses`](Self::set_private_ip_addresses).
    ///
    /// <p>The private IPv4 addresses you want to unassign.</p>
    pub fn private_ip_addresses(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.private_ip_addresses(input.into());
        self
    }
    /// <p>The private IPv4 addresses you want to unassign.</p>
    pub fn set_private_ip_addresses(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_private_ip_addresses(input);
        self
    }
    /// <p>The maximum amount of time to wait (in seconds) before forcibly releasing the IP addresses if connections are still in progress. Default value is 350 seconds.</p>
    pub fn max_drain_duration_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.max_drain_duration_seconds(input);
        self
    }
    /// <p>The maximum amount of time to wait (in seconds) before forcibly releasing the IP addresses if connections are still in progress. Default value is 350 seconds.</p>
    pub fn set_max_drain_duration_seconds(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_drain_duration_seconds(input);
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