// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_network_interface::_create_network_interface_output::CreateNetworkInterfaceOutputBuilder;

pub use crate::operation::create_network_interface::_create_network_interface_input::CreateNetworkInterfaceInputBuilder;

/// Fluent builder constructing a request to `CreateNetworkInterface`.
///
/// <p>Creates a network interface in the specified subnet.</p>
/// <p>The number of IP addresses you can assign to a network interface varies by instance type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-eni.html#AvailableIpPerENI">IP Addresses Per ENI Per Instance Type</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
/// <p>For more information about network interfaces, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-eni.html">Elastic network interfaces</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateNetworkInterfaceFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_network_interface::builders::CreateNetworkInterfaceInputBuilder,
}
impl CreateNetworkInterfaceFluentBuilder {
    /// Creates a new `CreateNetworkInterface`.
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
            crate::operation::create_network_interface::CreateNetworkInterface,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_network_interface::CreateNetworkInterfaceError,
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
        crate::operation::create_network_interface::CreateNetworkInterfaceOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_network_interface::CreateNetworkInterfaceError,
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
    ///     let deserialized_parameters: crate::operation::create_network_interface::builders::CreateNetworkInterfaceInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_network_interface().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_network_interface::builders::CreateNetworkInterfaceInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>A description for the network interface.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for the network interface.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
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
    /// Appends an item to `Groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>The IDs of one or more security groups.</p>
    pub fn groups(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.groups(input.into());
        self
    }
    /// <p>The IDs of one or more security groups.</p>
    pub fn set_groups(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_groups(input);
        self
    }
    /// <p>The number of IPv6 addresses to assign to a network interface. Amazon EC2 automatically selects the IPv6 addresses from the subnet range.</p>
    /// <p>You can't specify a count of IPv6 addresses using this parameter if you've specified one of the following: specific IPv6 addresses, specific IPv6 prefixes, or a count of IPv6 prefixes.</p>
    /// <p>If your subnet has the <code>AssignIpv6AddressOnCreation</code> attribute set, you can override that setting by specifying 0 as the IPv6 address count.</p>
    pub fn ipv6_address_count(mut self, input: i32) -> Self {
        self.inner = self.inner.ipv6_address_count(input);
        self
    }
    /// <p>The number of IPv6 addresses to assign to a network interface. Amazon EC2 automatically selects the IPv6 addresses from the subnet range.</p>
    /// <p>You can't specify a count of IPv6 addresses using this parameter if you've specified one of the following: specific IPv6 addresses, specific IPv6 prefixes, or a count of IPv6 prefixes.</p>
    /// <p>If your subnet has the <code>AssignIpv6AddressOnCreation</code> attribute set, you can override that setting by specifying 0 as the IPv6 address count.</p>
    pub fn set_ipv6_address_count(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_ipv6_address_count(input);
        self
    }
    /// Appends an item to `Ipv6Addresses`.
    ///
    /// To override the contents of this collection use [`set_ipv6_addresses`](Self::set_ipv6_addresses).
    ///
    /// <p>The IPv6 addresses from the IPv6 CIDR block range of your subnet.</p>
    /// <p>You can't specify IPv6 addresses using this parameter if you've specified one of the following: a count of IPv6 addresses, specific IPv6 prefixes, or a count of IPv6 prefixes.</p>
    pub fn ipv6_addresses(mut self, input: crate::types::InstanceIpv6Address) -> Self {
        self.inner = self.inner.ipv6_addresses(input);
        self
    }
    /// <p>The IPv6 addresses from the IPv6 CIDR block range of your subnet.</p>
    /// <p>You can't specify IPv6 addresses using this parameter if you've specified one of the following: a count of IPv6 addresses, specific IPv6 prefixes, or a count of IPv6 prefixes.</p>
    pub fn set_ipv6_addresses(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::InstanceIpv6Address>>,
    ) -> Self {
        self.inner = self.inner.set_ipv6_addresses(input);
        self
    }
    /// <p>The primary private IPv4 address of the network interface. If you don't specify an IPv4 address, Amazon EC2 selects one for you from the subnet's IPv4 CIDR range. If you specify an IP address, you cannot indicate any IP addresses specified in <code>privateIpAddresses</code> as primary (only one IP address can be designated as primary).</p>
    pub fn private_ip_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.private_ip_address(input.into());
        self
    }
    /// <p>The primary private IPv4 address of the network interface. If you don't specify an IPv4 address, Amazon EC2 selects one for you from the subnet's IPv4 CIDR range. If you specify an IP address, you cannot indicate any IP addresses specified in <code>privateIpAddresses</code> as primary (only one IP address can be designated as primary).</p>
    pub fn set_private_ip_address(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_private_ip_address(input);
        self
    }
    /// Appends an item to `PrivateIpAddresses`.
    ///
    /// To override the contents of this collection use [`set_private_ip_addresses`](Self::set_private_ip_addresses).
    ///
    /// <p>The private IPv4 addresses.</p>
    /// <p>You can't specify private IPv4 addresses if you've specified one of the following: a count of private IPv4 addresses, specific IPv4 prefixes, or a count of IPv4 prefixes.</p>
    pub fn private_ip_addresses(
        mut self,
        input: crate::types::PrivateIpAddressSpecification,
    ) -> Self {
        self.inner = self.inner.private_ip_addresses(input);
        self
    }
    /// <p>The private IPv4 addresses.</p>
    /// <p>You can't specify private IPv4 addresses if you've specified one of the following: a count of private IPv4 addresses, specific IPv4 prefixes, or a count of IPv4 prefixes.</p>
    pub fn set_private_ip_addresses(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PrivateIpAddressSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_private_ip_addresses(input);
        self
    }
    /// <p>The number of secondary private IPv4 addresses to assign to a network interface. When you specify a number of secondary IPv4 addresses, Amazon EC2 selects these IP addresses within the subnet's IPv4 CIDR range. You can't specify this option and specify more than one private IP address using <code>privateIpAddresses</code>.</p>
    /// <p>You can't specify a count of private IPv4 addresses if you've specified one of the following: specific private IPv4 addresses, specific IPv4 prefixes, or a count of IPv4 prefixes.</p>
    pub fn secondary_private_ip_address_count(mut self, input: i32) -> Self {
        self.inner = self.inner.secondary_private_ip_address_count(input);
        self
    }
    /// <p>The number of secondary private IPv4 addresses to assign to a network interface. When you specify a number of secondary IPv4 addresses, Amazon EC2 selects these IP addresses within the subnet's IPv4 CIDR range. You can't specify this option and specify more than one private IP address using <code>privateIpAddresses</code>.</p>
    /// <p>You can't specify a count of private IPv4 addresses if you've specified one of the following: specific private IPv4 addresses, specific IPv4 prefixes, or a count of IPv4 prefixes.</p>
    pub fn set_secondary_private_ip_address_count(
        mut self,
        input: std::option::Option<i32>,
    ) -> Self {
        self.inner = self.inner.set_secondary_private_ip_address_count(input);
        self
    }
    /// Appends an item to `Ipv4Prefixes`.
    ///
    /// To override the contents of this collection use [`set_ipv4_prefixes`](Self::set_ipv4_prefixes).
    ///
    /// <p>The IPv4 prefixes assigned to the network interface.</p>
    /// <p>You can't specify IPv4 prefixes if you've specified one of the following: a count of IPv4 prefixes, specific private IPv4 addresses, or a count of private IPv4 addresses.</p>
    pub fn ipv4_prefixes(mut self, input: crate::types::Ipv4PrefixSpecificationRequest) -> Self {
        self.inner = self.inner.ipv4_prefixes(input);
        self
    }
    /// <p>The IPv4 prefixes assigned to the network interface.</p>
    /// <p>You can't specify IPv4 prefixes if you've specified one of the following: a count of IPv4 prefixes, specific private IPv4 addresses, or a count of private IPv4 addresses.</p>
    pub fn set_ipv4_prefixes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Ipv4PrefixSpecificationRequest>>,
    ) -> Self {
        self.inner = self.inner.set_ipv4_prefixes(input);
        self
    }
    /// <p>The number of IPv4 prefixes that Amazon Web Services automatically assigns to the network interface.</p>
    /// <p>You can't specify a count of IPv4 prefixes if you've specified one of the following: specific IPv4 prefixes, specific private IPv4 addresses, or a count of private IPv4 addresses.</p>
    pub fn ipv4_prefix_count(mut self, input: i32) -> Self {
        self.inner = self.inner.ipv4_prefix_count(input);
        self
    }
    /// <p>The number of IPv4 prefixes that Amazon Web Services automatically assigns to the network interface.</p>
    /// <p>You can't specify a count of IPv4 prefixes if you've specified one of the following: specific IPv4 prefixes, specific private IPv4 addresses, or a count of private IPv4 addresses.</p>
    pub fn set_ipv4_prefix_count(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_ipv4_prefix_count(input);
        self
    }
    /// Appends an item to `Ipv6Prefixes`.
    ///
    /// To override the contents of this collection use [`set_ipv6_prefixes`](Self::set_ipv6_prefixes).
    ///
    /// <p>The IPv6 prefixes assigned to the network interface.</p>
    /// <p>You can't specify IPv6 prefixes if you've specified one of the following: a count of IPv6 prefixes, specific IPv6 addresses, or a count of IPv6 addresses.</p>
    pub fn ipv6_prefixes(mut self, input: crate::types::Ipv6PrefixSpecificationRequest) -> Self {
        self.inner = self.inner.ipv6_prefixes(input);
        self
    }
    /// <p>The IPv6 prefixes assigned to the network interface.</p>
    /// <p>You can't specify IPv6 prefixes if you've specified one of the following: a count of IPv6 prefixes, specific IPv6 addresses, or a count of IPv6 addresses.</p>
    pub fn set_ipv6_prefixes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Ipv6PrefixSpecificationRequest>>,
    ) -> Self {
        self.inner = self.inner.set_ipv6_prefixes(input);
        self
    }
    /// <p>The number of IPv6 prefixes that Amazon Web Services automatically assigns to the network interface.</p>
    /// <p>You can't specify a count of IPv6 prefixes if you've specified one of the following: specific IPv6 prefixes, specific IPv6 addresses, or a count of IPv6 addresses.</p>
    pub fn ipv6_prefix_count(mut self, input: i32) -> Self {
        self.inner = self.inner.ipv6_prefix_count(input);
        self
    }
    /// <p>The number of IPv6 prefixes that Amazon Web Services automatically assigns to the network interface.</p>
    /// <p>You can't specify a count of IPv6 prefixes if you've specified one of the following: specific IPv6 prefixes, specific IPv6 addresses, or a count of IPv6 addresses.</p>
    pub fn set_ipv6_prefix_count(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_ipv6_prefix_count(input);
        self
    }
    /// <p>The type of network interface. The default is <code>interface</code>.</p>
    /// <p>The only supported values are <code>efa</code> and <code>trunk</code>.</p>
    pub fn interface_type(mut self, input: crate::types::NetworkInterfaceCreationType) -> Self {
        self.inner = self.inner.interface_type(input);
        self
    }
    /// <p>The type of network interface. The default is <code>interface</code>.</p>
    /// <p>The only supported values are <code>efa</code> and <code>trunk</code>.</p>
    pub fn set_interface_type(
        mut self,
        input: std::option::Option<crate::types::NetworkInterfaceCreationType>,
    ) -> Self {
        self.inner = self.inner.set_interface_type(input);
        self
    }
    /// <p>The ID of the subnet to associate with the network interface.</p>
    pub fn subnet_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.subnet_id(input.into());
        self
    }
    /// <p>The ID of the subnet to associate with the network interface.</p>
    pub fn set_subnet_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_subnet_id(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the new network interface.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the new network interface.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}