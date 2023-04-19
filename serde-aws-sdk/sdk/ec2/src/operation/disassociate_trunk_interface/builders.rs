// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_trunk_interface::_disassociate_trunk_interface_output::DisassociateTrunkInterfaceOutputBuilder;

pub use crate::operation::disassociate_trunk_interface::_disassociate_trunk_interface_input::DisassociateTrunkInterfaceInputBuilder;

/// Fluent builder constructing a request to `DisassociateTrunkInterface`.
///
/// <note>
/// <p>This API action is currently in <b>limited preview only</b>. If you are interested in using this feature, contact your account manager.</p>
/// </note>
/// <p>Removes an association between a branch network interface with a trunk network interface.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateTrunkInterfaceFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::disassociate_trunk_interface::builders::DisassociateTrunkInterfaceInputBuilder
            }
impl DisassociateTrunkInterfaceFluentBuilder {
    /// Creates a new `DisassociateTrunkInterface`.
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
            crate::operation::disassociate_trunk_interface::DisassociateTrunkInterface,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::disassociate_trunk_interface::DisassociateTrunkInterfaceError,
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
        crate::operation::disassociate_trunk_interface::DisassociateTrunkInterfaceOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::disassociate_trunk_interface::DisassociateTrunkInterfaceError,
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
    ///     let deserialized_parameters: crate::operation::disassociate_trunk_interface::builders::DisassociateTrunkInterfaceInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.disassociate_trunk_interface().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::disassociate_trunk_interface::builders::DisassociateTrunkInterfaceInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the association</p>
    pub fn association_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.association_id(input.into());
        self
    }
    /// <p>The ID of the association</p>
    pub fn set_association_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_association_id(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
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