// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_cidr_blocks::_list_cidr_blocks_output::ListCidrBlocksOutputBuilder;

pub use crate::operation::list_cidr_blocks::_list_cidr_blocks_input::ListCidrBlocksInputBuilder;

/// Fluent builder constructing a request to `ListCidrBlocks`.
///
/// <p>Returns a paginated list of location objects and their CIDR blocks.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListCidrBlocksFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_cidr_blocks::builders::ListCidrBlocksInputBuilder,
}
impl ListCidrBlocksFluentBuilder {
    /// Creates a new `ListCidrBlocks`.
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
            crate::operation::list_cidr_blocks::ListCidrBlocks,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::list_cidr_blocks::ListCidrBlocksError>,
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
        crate::operation::list_cidr_blocks::ListCidrBlocksOutput,
        aws_smithy_http::result::SdkError<crate::operation::list_cidr_blocks::ListCidrBlocksError>,
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
    ///     let deserialized_parameters: crate::operation::list_cidr_blocks::builders::ListCidrBlocksInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.list_cidr_blocks().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::list_cidr_blocks::builders::ListCidrBlocksInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_cidr_blocks::paginator::ListCidrBlocksPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_cidr_blocks::paginator::ListCidrBlocksPaginator {
        crate::operation::list_cidr_blocks::paginator::ListCidrBlocksPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The UUID of the CIDR collection.</p>
    pub fn collection_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.collection_id(input.into());
        self
    }
    /// <p>The UUID of the CIDR collection.</p>
    pub fn set_collection_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_collection_id(input);
        self
    }
    /// <p>The name of the CIDR collection location.</p>
    pub fn location_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.location_name(input.into());
        self
    }
    /// <p>The name of the CIDR collection location.</p>
    pub fn set_location_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_location_name(input);
        self
    }
    /// <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Maximum number of results you want returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Maximum number of results you want returned.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}