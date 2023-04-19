// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::provision_ipam_pool_cidr::_provision_ipam_pool_cidr_output::ProvisionIpamPoolCidrOutputBuilder;

pub use crate::operation::provision_ipam_pool_cidr::_provision_ipam_pool_cidr_input::ProvisionIpamPoolCidrInputBuilder;

/// Fluent builder constructing a request to `ProvisionIpamPoolCidr`.
///
/// <p>Provision a CIDR to an IPAM pool. You can use this action to provision new CIDRs to a top-level pool or to transfer a CIDR from a top-level pool to a pool within it.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/prov-cidr-ipam.html">Provision CIDRs to pools</a> in the <i>Amazon VPC IPAM User Guide</i>. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ProvisionIpamPoolCidrFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::provision_ipam_pool_cidr::builders::ProvisionIpamPoolCidrInputBuilder,
}
impl ProvisionIpamPoolCidrFluentBuilder {
    /// Creates a new `ProvisionIpamPoolCidr`.
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
            crate::operation::provision_ipam_pool_cidr::ProvisionIpamPoolCidr,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::provision_ipam_pool_cidr::ProvisionIpamPoolCidrError,
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
        crate::operation::provision_ipam_pool_cidr::ProvisionIpamPoolCidrOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::provision_ipam_pool_cidr::ProvisionIpamPoolCidrError,
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
    ///     let deserialized_parameters: crate::operation::provision_ipam_pool_cidr::builders::ProvisionIpamPoolCidrInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.provision_ipam_pool_cidr().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::provision_ipam_pool_cidr::builders::ProvisionIpamPoolCidrInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>The ID of the IPAM pool to which you want to assign a CIDR.</p>
    pub fn ipam_pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.ipam_pool_id(input.into());
        self
    }
    /// <p>The ID of the IPAM pool to which you want to assign a CIDR.</p>
    pub fn set_ipam_pool_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_ipam_pool_id(input);
        self
    }
    /// <p>The CIDR you want to assign to the IPAM pool. Either "NetmaskLength" or "Cidr" is required. This value will be null if you specify "NetmaskLength" and will be filled in during the provisioning process.</p>
    pub fn cidr(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.cidr(input.into());
        self
    }
    /// <p>The CIDR you want to assign to the IPAM pool. Either "NetmaskLength" or "Cidr" is required. This value will be null if you specify "NetmaskLength" and will be filled in during the provisioning process.</p>
    pub fn set_cidr(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_cidr(input);
        self
    }
    /// <p>A signed document that proves that you are authorized to bring a specified IP address range to Amazon using BYOIP. This option applies to public pools only.</p>
    pub fn cidr_authorization_context(
        mut self,
        input: crate::types::IpamCidrAuthorizationContext,
    ) -> Self {
        self.inner = self.inner.cidr_authorization_context(input);
        self
    }
    /// <p>A signed document that proves that you are authorized to bring a specified IP address range to Amazon using BYOIP. This option applies to public pools only.</p>
    pub fn set_cidr_authorization_context(
        mut self,
        input: std::option::Option<crate::types::IpamCidrAuthorizationContext>,
    ) -> Self {
        self.inner = self.inner.set_cidr_authorization_context(input);
        self
    }
    /// <p>The netmask length of the CIDR you'd like to provision to a pool. Can be used for provisioning Amazon-provided IPv6 CIDRs to top-level pools and for provisioning CIDRs to pools with source pools. Cannot be used to provision BYOIP CIDRs to top-level pools. Either "NetmaskLength" or "Cidr" is required.</p>
    pub fn netmask_length(mut self, input: i32) -> Self {
        self.inner = self.inner.netmask_length(input);
        self
    }
    /// <p>The netmask length of the CIDR you'd like to provision to a pool. Can be used for provisioning Amazon-provided IPv6 CIDRs to top-level pools and for provisioning CIDRs to pools with source pools. Cannot be used to provision BYOIP CIDRs to top-level pools. Either "NetmaskLength" or "Cidr" is required.</p>
    pub fn set_netmask_length(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_netmask_length(input);
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}