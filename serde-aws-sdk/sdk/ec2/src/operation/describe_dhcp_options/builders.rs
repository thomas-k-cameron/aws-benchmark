// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_dhcp_options::_describe_dhcp_options_output::DescribeDhcpOptionsOutputBuilder;

pub use crate::operation::describe_dhcp_options::_describe_dhcp_options_input::DescribeDhcpOptionsInputBuilder;

/// Fluent builder constructing a request to `DescribeDhcpOptions`.
///
/// <p>Describes one or more of your DHCP options sets.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_DHCP_Options.html">DHCP options sets</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeDhcpOptionsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_dhcp_options::builders::DescribeDhcpOptionsInputBuilder,
}
impl DescribeDhcpOptionsFluentBuilder {
    /// Creates a new `DescribeDhcpOptions`.
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
            crate::operation::describe_dhcp_options::DescribeDhcpOptions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_dhcp_options::DescribeDhcpOptionsError,
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
        crate::operation::describe_dhcp_options::DescribeDhcpOptionsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_dhcp_options::DescribeDhcpOptionsError,
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
    ///     let deserialized_parameters: crate::operation::describe_dhcp_options::builders::DescribeDhcpOptionsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_dhcp_options().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_dhcp_options::builders::DescribeDhcpOptionsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_dhcp_options::paginator::DescribeDhcpOptionsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_dhcp_options::paginator::DescribeDhcpOptionsPaginator {
        crate::operation::describe_dhcp_options::paginator::DescribeDhcpOptionsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// Appends an item to `DhcpOptionsIds`.
    ///
    /// To override the contents of this collection use [`set_dhcp_options_ids`](Self::set_dhcp_options_ids).
    ///
    /// <p>The IDs of one or more DHCP options sets.</p>
    /// <p>Default: Describes all your DHCP options sets.</p>
    pub fn dhcp_options_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.dhcp_options_ids(input.into());
        self
    }
    /// <p>The IDs of one or more DHCP options sets.</p>
    /// <p>Default: Describes all your DHCP options sets.</p>
    pub fn set_dhcp_options_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_dhcp_options_ids(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>dhcp-options-id</code> - The ID of a DHCP options set.</p> </li>
    /// <li> <p> <code>key</code> - The key for one of the options (for example, <code>domain-name</code>).</p> </li>
    /// <li> <p> <code>value</code> - The value for one of the options.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the DHCP options set.</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>dhcp-options-id</code> - The ID of a DHCP options set.</p> </li>
    /// <li> <p> <code>key</code> - The key for one of the options (for example, <code>domain-name</code>).</p> </li>
    /// <li> <p> <code>value</code> - The value for one of the options.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the DHCP options set.</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}