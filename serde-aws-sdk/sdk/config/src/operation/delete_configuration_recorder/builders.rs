// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_configuration_recorder::_delete_configuration_recorder_output::DeleteConfigurationRecorderOutputBuilder;

pub use crate::operation::delete_configuration_recorder::_delete_configuration_recorder_input::DeleteConfigurationRecorderInputBuilder;

/// Fluent builder constructing a request to `DeleteConfigurationRecorder`.
///
/// <p>Deletes the configuration recorder.</p>
/// <p>After the configuration recorder is deleted, Config will not record resource configuration changes until you create a new configuration recorder.</p>
/// <p>This action does not delete the configuration information that was previously recorded. You will be able to access the previously recorded information by using the <code>GetResourceConfigHistory</code> action, but you will not be able to access this information in the Config console until you create a new configuration recorder.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteConfigurationRecorderFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::delete_configuration_recorder::builders::DeleteConfigurationRecorderInputBuilder
            }
impl DeleteConfigurationRecorderFluentBuilder {
    /// Creates a new `DeleteConfigurationRecorder`.
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
            crate::operation::delete_configuration_recorder::DeleteConfigurationRecorder,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderError,
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
        crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderError,
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
    ///     let deserialized_parameters: crate::operation::delete_configuration_recorder::builders::DeleteConfigurationRecorderInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_configuration_recorder().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_configuration_recorder::builders::DeleteConfigurationRecorderInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the configuration recorder to be deleted. You can retrieve the name of your configuration recorder by using the <code>DescribeConfigurationRecorders</code> action.</p>
    pub fn configuration_recorder_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.configuration_recorder_name(input.into());
        self
    }
    /// <p>The name of the configuration recorder to be deleted. You can retrieve the name of your configuration recorder by using the <code>DescribeConfigurationRecorders</code> action.</p>
    pub fn set_configuration_recorder_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_configuration_recorder_name(input);
        self
    }
}