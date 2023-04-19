// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_health_check_status::_get_health_check_status_output::GetHealthCheckStatusOutputBuilder;

pub use crate::operation::get_health_check_status::_get_health_check_status_input::GetHealthCheckStatusInputBuilder;

/// Fluent builder constructing a request to `GetHealthCheckStatus`.
///
/// <p>Gets status of a specified health check. </p> <important>
/// <p>This API is intended for use during development to diagnose behavior. It doesn’t support production use-cases with high query rates that require immediate and actionable responses.</p>
/// </important>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetHealthCheckStatusFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_health_check_status::builders::GetHealthCheckStatusInputBuilder,
}
impl GetHealthCheckStatusFluentBuilder {
    /// Creates a new `GetHealthCheckStatus`.
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
            crate::operation::get_health_check_status::GetHealthCheckStatus,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_health_check_status::GetHealthCheckStatusError,
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
        crate::operation::get_health_check_status::GetHealthCheckStatusOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_health_check_status::GetHealthCheckStatusError,
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
    ///     let deserialized_parameters: crate::operation::get_health_check_status::builders::GetHealthCheckStatusInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_health_check_status().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_health_check_status::builders::GetHealthCheckStatusInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID for the health check that you want the current status for. When you created the health check, <code>CreateHealthCheck</code> returned the ID in the response, in the <code>HealthCheckId</code> element.</p> <note>
    /// <p>If you want to check the status of a calculated health check, you must use the Amazon Route 53 console or the CloudWatch console. You can't use <code>GetHealthCheckStatus</code> to get the status of a calculated health check.</p>
    /// </note>
    pub fn health_check_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.health_check_id(input.into());
        self
    }
    /// <p>The ID for the health check that you want the current status for. When you created the health check, <code>CreateHealthCheck</code> returned the ID in the response, in the <code>HealthCheckId</code> element.</p> <note>
    /// <p>If you want to check the status of a calculated health check, you must use the Amazon Route 53 console or the CloudWatch console. You can't use <code>GetHealthCheckStatus</code> to get the status of a calculated health check.</p>
    /// </note>
    pub fn set_health_check_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_health_check_id(input);
        self
    }
}