// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_scheduling_policy::_delete_scheduling_policy_output::DeleteSchedulingPolicyOutputBuilder;

pub use crate::operation::delete_scheduling_policy::_delete_scheduling_policy_input::DeleteSchedulingPolicyInputBuilder;

/// Fluent builder constructing a request to `DeleteSchedulingPolicy`.
///
/// <p>Deletes the specified scheduling policy.</p>
/// <p>You can't delete a scheduling policy that's used in any job queues.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSchedulingPolicyFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_scheduling_policy::builders::DeleteSchedulingPolicyInputBuilder,
}
impl DeleteSchedulingPolicyFluentBuilder {
    /// Creates a new `DeleteSchedulingPolicy`.
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
            crate::operation::delete_scheduling_policy::DeleteSchedulingPolicy,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_scheduling_policy::DeleteSchedulingPolicyError,
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
        crate::operation::delete_scheduling_policy::DeleteSchedulingPolicyOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_scheduling_policy::DeleteSchedulingPolicyError,
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
    ///     let deserialized_parameters: crate::operation::delete_scheduling_policy::builders::DeleteSchedulingPolicyInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_scheduling_policy().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_scheduling_policy::builders::DeleteSchedulingPolicyInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy to delete.</p>
    pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy to delete.</p>
    pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
}