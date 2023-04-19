// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_remediation_exceptions::_describe_remediation_exceptions_output::DescribeRemediationExceptionsOutputBuilder;

pub use crate::operation::describe_remediation_exceptions::_describe_remediation_exceptions_input::DescribeRemediationExceptionsInputBuilder;

/// Fluent builder constructing a request to `DescribeRemediationExceptions`.
///
/// <p>Returns the details of one or more remediation exceptions. A detailed view of a remediation exception for a set of resources that includes an explanation of an exception and the time when the exception will be deleted. When you specify the limit and the next token, you receive a paginated response. </p> <note>
/// <p>Config generates a remediation exception when a problem occurs executing a remediation action to a specific resource. Remediation exceptions blocks auto-remediation until the exception is cleared.</p>
/// <p>When you specify the limit and the next token, you receive a paginated response. </p>
/// <p>Limit and next token are not applicable if you request resources in batch. It is only applicable, when you request all resources.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeRemediationExceptionsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_remediation_exceptions::builders::DescribeRemediationExceptionsInputBuilder
            }
impl DescribeRemediationExceptionsFluentBuilder {
    /// Creates a new `DescribeRemediationExceptions`.
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
            crate::operation::describe_remediation_exceptions::DescribeRemediationExceptions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_remediation_exceptions::DescribeRemediationExceptionsError,
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
        crate::operation::describe_remediation_exceptions::DescribeRemediationExceptionsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_remediation_exceptions::DescribeRemediationExceptionsError,
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
    ///     let deserialized_parameters: crate::operation::describe_remediation_exceptions::builders::DescribeRemediationExceptionsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_remediation_exceptions().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_remediation_exceptions::builders::DescribeRemediationExceptionsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_remediation_exceptions::paginator::DescribeRemediationExceptionsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_remediation_exceptions::paginator::DescribeRemediationExceptionsPaginator{
        crate::operation::describe_remediation_exceptions::paginator::DescribeRemediationExceptionsPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the Config rule.</p>
    pub fn config_rule_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.config_rule_name(input.into());
        self
    }
    /// <p>The name of the Config rule.</p>
    pub fn set_config_rule_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_config_rule_name(input);
        self
    }
    /// Appends an item to `ResourceKeys`.
    ///
    /// To override the contents of this collection use [`set_resource_keys`](Self::set_resource_keys).
    ///
    /// <p>An exception list of resource exception keys to be processed with the current request. Config adds exception for each resource key. For example, Config adds 3 exceptions for 3 resource keys. </p>
    pub fn resource_keys(mut self, input: crate::types::RemediationExceptionResourceKey) -> Self {
        self.inner = self.inner.resource_keys(input);
        self
    }
    /// <p>An exception list of resource exception keys to be processed with the current request. Config adds exception for each resource key. For example, Config adds 3 exceptions for 3 resource keys. </p>
    pub fn set_resource_keys(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::RemediationExceptionResourceKey>>,
    ) -> Self {
        self.inner = self.inner.set_resource_keys(input);
        self
    }
    /// <p>The maximum number of RemediationExceptionResourceKey returned on each page. The default is 25. If you specify 0, Config uses the default.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of RemediationExceptionResourceKey returned on each page. The default is 25. If you specify 0, Config uses the default.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}