// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_speech_synthesis_task::_get_speech_synthesis_task_output::GetSpeechSynthesisTaskOutputBuilder;

pub use crate::operation::get_speech_synthesis_task::_get_speech_synthesis_task_input::GetSpeechSynthesisTaskInputBuilder;

/// Fluent builder constructing a request to `GetSpeechSynthesisTask`.
///
/// <p>Retrieves a specific SpeechSynthesisTask object based on its TaskID. This object contains information about the given speech synthesis task, including the status of the task, and a link to the S3 bucket containing the output of the task.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetSpeechSynthesisTaskFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::get_speech_synthesis_task::builders::GetSpeechSynthesisTaskInputBuilder,
}
impl GetSpeechSynthesisTaskFluentBuilder {
    /// Creates a new `GetSpeechSynthesisTask`.
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
            crate::operation::get_speech_synthesis_task::GetSpeechSynthesisTask,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_speech_synthesis_task::GetSpeechSynthesisTaskError,
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
        crate::operation::get_speech_synthesis_task::GetSpeechSynthesisTaskOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_speech_synthesis_task::GetSpeechSynthesisTaskError,
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
    ///     let deserialized_parameters: crate::operation::get_speech_synthesis_task::builders::GetSpeechSynthesisTaskInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_speech_synthesis_task().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_speech_synthesis_task::builders::GetSpeechSynthesisTaskInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The Amazon Polly generated identifier for a speech synthesis task.</p>
    pub fn task_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.task_id(input.into());
        self
    }
    /// <p>The Amazon Polly generated identifier for a speech synthesis task.</p>
    pub fn set_task_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_task_id(input);
        self
    }
}