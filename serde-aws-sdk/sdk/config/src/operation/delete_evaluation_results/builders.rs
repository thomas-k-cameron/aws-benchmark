// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_evaluation_results::_delete_evaluation_results_output::DeleteEvaluationResultsOutputBuilder;

pub use crate::operation::delete_evaluation_results::_delete_evaluation_results_input::DeleteEvaluationResultsInputBuilder;

/// Fluent builder constructing a request to `DeleteEvaluationResults`.
///
/// <p>Deletes the evaluation results for the specified Config rule. You can specify one Config rule per request. After you delete the evaluation results, you can call the <code>StartConfigRulesEvaluation</code> API to start evaluating your Amazon Web Services resources against the rule.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteEvaluationResultsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::delete_evaluation_results::builders::DeleteEvaluationResultsInputBuilder,
}
impl DeleteEvaluationResultsFluentBuilder {
    /// Creates a new `DeleteEvaluationResults`.
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
            crate::operation::delete_evaluation_results::DeleteEvaluationResults,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_evaluation_results::DeleteEvaluationResultsError,
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
        crate::operation::delete_evaluation_results::DeleteEvaluationResultsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_evaluation_results::DeleteEvaluationResultsError,
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
    ///     let deserialized_parameters: crate::operation::delete_evaluation_results::builders::DeleteEvaluationResultsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_evaluation_results().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_evaluation_results::builders::DeleteEvaluationResultsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the Config rule for which you want to delete the evaluation results.</p>
    pub fn config_rule_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.config_rule_name(input.into());
        self
    }
    /// <p>The name of the Config rule for which you want to delete the evaluation results.</p>
    pub fn set_config_rule_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_config_rule_name(input);
        self
    }
}