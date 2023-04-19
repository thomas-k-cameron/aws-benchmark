// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_health_check::_delete_health_check_output::DeleteHealthCheckOutputBuilder;

pub use crate::operation::delete_health_check::_delete_health_check_input::DeleteHealthCheckInputBuilder;

/// Fluent builder constructing a request to `DeleteHealthCheck`.
///
/// <p>Deletes a health check.</p> <important>
/// <p>Amazon Route 53 does not prevent you from deleting a health check even if the health check is associated with one or more resource record sets. If you delete a health check and you don't update the associated resource record sets, the future status of the health check can't be predicted and may change. This will affect the routing of DNS queries for your DNS failover configuration. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/health-checks-creating-deleting.html#health-checks-deleting.html">Replacing and Deleting Health Checks</a> in the <i>Amazon Route 53 Developer Guide</i>.</p>
/// </important>
/// <p>If you're using Cloud Map and you configured Cloud Map to create a Route 53 health check when you register an instance, you can't use the Route 53 <code>DeleteHealthCheck</code> command to delete the health check. The health check is deleted automatically when you deregister the instance; there can be a delay of several hours before the health check is deleted from Route 53. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteHealthCheckFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_health_check::builders::DeleteHealthCheckInputBuilder,
}
impl DeleteHealthCheckFluentBuilder {
    /// Creates a new `DeleteHealthCheck`.
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
            crate::operation::delete_health_check::DeleteHealthCheck,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_health_check::DeleteHealthCheckError,
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
        crate::operation::delete_health_check::DeleteHealthCheckOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_health_check::DeleteHealthCheckError,
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
    ///     let deserialized_parameters: crate::operation::delete_health_check::builders::DeleteHealthCheckInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_health_check().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_health_check::builders::DeleteHealthCheckInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the health check that you want to delete.</p>
    pub fn health_check_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.health_check_id(input.into());
        self
    }
    /// <p>The ID of the health check that you want to delete.</p>
    pub fn set_health_check_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_health_check_id(input);
        self
    }
}