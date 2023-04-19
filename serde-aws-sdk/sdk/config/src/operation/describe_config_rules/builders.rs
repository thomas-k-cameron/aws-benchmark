// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_config_rules::_describe_config_rules_output::DescribeConfigRulesOutputBuilder;

pub use crate::operation::describe_config_rules::_describe_config_rules_input::DescribeConfigRulesInputBuilder;

/// Fluent builder constructing a request to `DescribeConfigRules`.
///
/// <p>Returns details about your Config rules.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeConfigRulesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_config_rules::builders::DescribeConfigRulesInputBuilder,
}
impl DescribeConfigRulesFluentBuilder {
    /// Creates a new `DescribeConfigRules`.
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
            crate::operation::describe_config_rules::DescribeConfigRules,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_config_rules::DescribeConfigRulesError,
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
        crate::operation::describe_config_rules::DescribeConfigRulesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_config_rules::DescribeConfigRulesError,
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
    ///     let deserialized_parameters: crate::operation::describe_config_rules::builders::DescribeConfigRulesInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_config_rules().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_config_rules::builders::DescribeConfigRulesInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_config_rules::paginator::DescribeConfigRulesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_config_rules::paginator::DescribeConfigRulesPaginator {
        crate::operation::describe_config_rules::paginator::DescribeConfigRulesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// Appends an item to `ConfigRuleNames`.
    ///
    /// To override the contents of this collection use [`set_config_rule_names`](Self::set_config_rule_names).
    ///
    /// <p>The names of the Config rules for which you want details. If you do not specify any names, Config returns details for all your rules.</p>
    pub fn config_rule_names(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.config_rule_names(input.into());
        self
    }
    /// <p>The names of the Config rules for which you want details. If you do not specify any names, Config returns details for all your rules.</p>
    pub fn set_config_rule_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_config_rule_names(input);
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Returns a list of Detecive or Proactive Config rules. By default, this API returns an unfiltered list.</p>
    pub fn filters(mut self, input: crate::types::DescribeConfigRulesFilters) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>Returns a list of Detecive or Proactive Config rules. By default, this API returns an unfiltered list.</p>
    pub fn set_filters(
        mut self,
        input: std::option::Option<crate::types::DescribeConfigRulesFilters>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
}