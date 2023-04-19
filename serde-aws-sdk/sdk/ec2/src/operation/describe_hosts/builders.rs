// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_hosts::_describe_hosts_output::DescribeHostsOutputBuilder;

pub use crate::operation::describe_hosts::_describe_hosts_input::DescribeHostsInputBuilder;

/// Fluent builder constructing a request to `DescribeHosts`.
///
/// <p>Describes the specified Dedicated Hosts or all your Dedicated Hosts.</p>
/// <p>The results describe only the Dedicated Hosts in the Region you're currently using. All listed instances consume capacity on your Dedicated Host. Dedicated Hosts that have recently been released are listed with the state <code>released</code>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeHostsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_hosts::builders::DescribeHostsInputBuilder,
}
impl DescribeHostsFluentBuilder {
    /// Creates a new `DescribeHosts`.
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
            crate::operation::describe_hosts::DescribeHosts,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::describe_hosts::DescribeHostsError>,
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
        crate::operation::describe_hosts::DescribeHostsOutput,
        aws_smithy_http::result::SdkError<crate::operation::describe_hosts::DescribeHostsError>,
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
    ///     let deserialized_parameters: crate::operation::describe_hosts::builders::DescribeHostsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_hosts().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_hosts::builders::DescribeHostsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_hosts::paginator::DescribeHostsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_hosts::paginator::DescribeHostsPaginator {
        crate::operation::describe_hosts::paginator::DescribeHostsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// Appends an item to `Filter`.
    ///
    /// To override the contents of this collection use [`set_filter`](Self::set_filter).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>auto-placement</code> - Whether auto-placement is enabled or disabled (<code>on</code> | <code>off</code>).</p> </li>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone of the host.</p> </li>
    /// <li> <p> <code>client-token</code> - The idempotency token that you provided when you allocated the host.</p> </li>
    /// <li> <p> <code>host-reservation-id</code> - The ID of the reservation assigned to this host.</p> </li>
    /// <li> <p> <code>instance-type</code> - The instance type size that the Dedicated Host is configured to support.</p> </li>
    /// <li> <p> <code>state</code> - The allocation state of the Dedicated Host (<code>available</code> | <code>under-assessment</code> | <code>permanent-failure</code> | <code>released</code> | <code>released-permanent-failure</code>).</p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// </ul>
    pub fn filter(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filter(input);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>auto-placement</code> - Whether auto-placement is enabled or disabled (<code>on</code> | <code>off</code>).</p> </li>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone of the host.</p> </li>
    /// <li> <p> <code>client-token</code> - The idempotency token that you provided when you allocated the host.</p> </li>
    /// <li> <p> <code>host-reservation-id</code> - The ID of the reservation assigned to this host.</p> </li>
    /// <li> <p> <code>instance-type</code> - The instance type size that the Dedicated Host is configured to support.</p> </li>
    /// <li> <p> <code>state</code> - The allocation state of the Dedicated Host (<code>available</code> | <code>under-assessment</code> | <code>permanent-failure</code> | <code>released</code> | <code>released-permanent-failure</code>).</p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// </ul>
    pub fn set_filter(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filter(input);
        self
    }
    /// Appends an item to `HostIds`.
    ///
    /// To override the contents of this collection use [`set_host_ids`](Self::set_host_ids).
    ///
    /// <p>The IDs of the Dedicated Hosts. The IDs are used for targeted instance launches.</p>
    pub fn host_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.host_ids(input.into());
        self
    }
    /// <p>The IDs of the Dedicated Hosts. The IDs are used for targeted instance launches.</p>
    pub fn set_host_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_host_ids(input);
        self
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    /// <p>You cannot specify this parameter and the host IDs parameter in the same request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    /// <p>You cannot specify this parameter and the host IDs parameter in the same request.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token to use to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}