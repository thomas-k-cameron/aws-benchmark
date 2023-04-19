// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_organization_config_rules::_describe_organization_config_rules_output::DescribeOrganizationConfigRulesOutputBuilder;

pub use crate::operation::describe_organization_config_rules::_describe_organization_config_rules_input::DescribeOrganizationConfigRulesInputBuilder;

/// Fluent builder constructing a request to `DescribeOrganizationConfigRules`.
///
/// <p>Returns a list of organization Config rules. </p> <note>
/// <p>When you specify the limit and the next token, you receive a paginated response.</p>
/// <p>Limit and next token are not applicable if you specify organization Config rule names. It is only applicable, when you request all the organization Config rules.</p>
/// <p> <i>For accounts within an organzation</i> </p>
/// <p>If you deploy an organizational rule or conformance pack in an organization administrator account, and then establish a delegated administrator and deploy an organizational rule or conformance pack in the delegated administrator account, you won't be able to see the organizational rule or conformance pack in the organization administrator account from the delegated administrator account or see the organizational rule or conformance pack in the delegated administrator account from organization administrator account. The <code>DescribeOrganizationConfigRules</code> and <code>DescribeOrganizationConformancePacks</code> APIs can only see and interact with the organization-related resource that were deployed from within the account calling those APIs.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeOrganizationConfigRulesFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_organization_config_rules::builders::DescribeOrganizationConfigRulesInputBuilder
            }
impl DescribeOrganizationConfigRulesFluentBuilder {
    /// Creates a new `DescribeOrganizationConfigRules`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::describe_organization_config_rules::DescribeOrganizationConfigRules, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::describe_organization_config_rules::DescribeOrganizationConfigRulesError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::describe_organization_config_rules::DescribeOrganizationConfigRulesOutput, aws_smithy_http::result::SdkError<crate::operation::describe_organization_config_rules::DescribeOrganizationConfigRulesError>>
                     {
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
    ///     let deserialized_parameters: crate::operation::describe_organization_config_rules::builders::DescribeOrganizationConfigRulesInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_organization_config_rules().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_organization_config_rules::builders::DescribeOrganizationConfigRulesInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_organization_config_rules::paginator::DescribeOrganizationConfigRulesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_organization_config_rules::paginator::DescribeOrganizationConfigRulesPaginator{
        crate::operation::describe_organization_config_rules::paginator::DescribeOrganizationConfigRulesPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `OrganizationConfigRuleNames`.
    ///
    /// To override the contents of this collection use [`set_organization_config_rule_names`](Self::set_organization_config_rule_names).
    ///
    /// <p>The names of organization Config rules for which you want details. If you do not specify any names, Config returns details for all your organization Config rules.</p>
    pub fn organization_config_rule_names(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.organization_config_rule_names(input.into());
        self
    }
    /// <p>The names of organization Config rules for which you want details. If you do not specify any names, Config returns details for all your organization Config rules.</p>
    pub fn set_organization_config_rule_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_organization_config_rule_names(input);
        self
    }
    /// <p>The maximum number of organization Config rules returned on each page. If you do no specify a number, Config uses the default. The default is 100.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of organization Config rules returned on each page. If you do no specify a number, Config uses the default. The default is 100.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}