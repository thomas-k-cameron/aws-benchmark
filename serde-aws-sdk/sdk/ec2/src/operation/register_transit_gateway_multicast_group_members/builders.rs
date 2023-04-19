// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::register_transit_gateway_multicast_group_members::_register_transit_gateway_multicast_group_members_output::RegisterTransitGatewayMulticastGroupMembersOutputBuilder;

pub use crate::operation::register_transit_gateway_multicast_group_members::_register_transit_gateway_multicast_group_members_input::RegisterTransitGatewayMulticastGroupMembersInputBuilder;

/// Fluent builder constructing a request to `RegisterTransitGatewayMulticastGroupMembers`.
///
/// <p>Registers members (network interfaces) with the transit gateway multicast group. A member is a network interface associated with a supported EC2 instance that receives multicast traffic. For information about supported instances, see <a href="https://docs.aws.amazon.com/vpc/latest/tgw/transit-gateway-limits.html#multicast-limits">Multicast Consideration</a> in <i>Amazon VPC Transit Gateways</i>.</p>
/// <p>After you add the members, use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_SearchTransitGatewayMulticastGroups.html">SearchTransitGatewayMulticastGroups</a> to verify that the members were added to the transit gateway multicast group.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RegisterTransitGatewayMulticastGroupMembersFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::register_transit_gateway_multicast_group_members::builders::RegisterTransitGatewayMulticastGroupMembersInputBuilder
            }
impl RegisterTransitGatewayMulticastGroupMembersFluentBuilder {
    /// Creates a new `RegisterTransitGatewayMulticastGroupMembers`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembers, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembersError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembersOutput, aws_smithy_http::result::SdkError<crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembersError>>
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
    ///     let deserialized_parameters: crate::operation::register_transit_gateway_multicast_group_members::builders::RegisterTransitGatewayMulticastGroupMembersInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.register_transit_gateway_multicast_group_members().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::register_transit_gateway_multicast_group_members::builders::RegisterTransitGatewayMulticastGroupMembersInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.transit_gateway_multicast_domain_id(input.into());
        self
    }
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn set_transit_gateway_multicast_domain_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_transit_gateway_multicast_domain_id(input);
        self
    }
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    pub fn group_ip_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.group_ip_address(input.into());
        self
    }
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    pub fn set_group_ip_address(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_group_ip_address(input);
        self
    }
    /// Appends an item to `NetworkInterfaceIds`.
    ///
    /// To override the contents of this collection use [`set_network_interface_ids`](Self::set_network_interface_ids).
    ///
    /// <p>The group members' network interface IDs to register with the transit gateway multicast group.</p>
    pub fn network_interface_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.network_interface_ids(input.into());
        self
    }
    /// <p>The group members' network interface IDs to register with the transit gateway multicast group.</p>
    pub fn set_network_interface_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_network_interface_ids(input);
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