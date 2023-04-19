// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_contributor_insights::_describe_contributor_insights_output::DescribeContributorInsightsOutputBuilder;

pub use crate::operation::describe_contributor_insights::_describe_contributor_insights_input::DescribeContributorInsightsInputBuilder;

/// Fluent builder constructing a request to `DescribeContributorInsights`.
///
/// <p>Returns information about contributor insights for a given table or global secondary index.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeContributorInsightsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsInputBuilder
            }
impl DescribeContributorInsightsFluentBuilder {
    /// Creates a new `DescribeContributorInsights`.
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
            crate::operation::describe_contributor_insights::DescribeContributorInsights,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_contributor_insights::DescribeContributorInsightsError,
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
        crate::operation::describe_contributor_insights::DescribeContributorInsightsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_contributor_insights::DescribeContributorInsightsError,
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
    ///     let deserialized_parameters: crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_contributor_insights().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_contributor_insights::builders::DescribeContributorInsightsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the table to describe.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table to describe.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>The name of the global secondary index to describe, if applicable.</p>
    pub fn index_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.index_name(input.into());
        self
    }
    /// <p>The name of the global secondary index to describe, if applicable.</p>
    pub fn set_index_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_index_name(input);
        self
    }
}