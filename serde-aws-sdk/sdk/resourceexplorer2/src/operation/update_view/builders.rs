// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_view::_update_view_output::UpdateViewOutputBuilder;

pub use crate::operation::update_view::_update_view_input::UpdateViewInputBuilder;

/// Fluent builder constructing a request to `UpdateView`.
///
/// <p>Modifies some of the details of a view. You can change the filter string and the list of included properties. You can't change the name of the view.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateViewFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_view::builders::UpdateViewInputBuilder,
}
impl UpdateViewFluentBuilder {
    /// Creates a new `UpdateView`.
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
            crate::operation::update_view::UpdateView,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_view::UpdateViewError>,
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
        crate::operation::update_view::UpdateViewOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_view::UpdateViewError>,
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
    ///     let deserialized_parameters: crate::operation::update_view::builders::UpdateViewInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.update_view().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::update_view::builders::UpdateViewInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that you want to modify.</p>
    pub fn view_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.view_arn(input.into());
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that you want to modify.</p>
    pub fn set_view_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_view_arn(input);
        self
    }
    /// Appends an item to `IncludedProperties`.
    ///
    /// To override the contents of this collection use [`set_included_properties`](Self::set_included_properties).
    ///
    /// <p>Specifies optional fields that you want included in search results from this view. It is a list of objects that each describe a field to include.</p>
    /// <p>The default is an empty list, with no optional fields included in the results.</p>
    pub fn included_properties(mut self, input: crate::types::IncludedProperty) -> Self {
        self.inner = self.inner.included_properties(input);
        self
    }
    /// <p>Specifies optional fields that you want included in search results from this view. It is a list of objects that each describe a field to include.</p>
    /// <p>The default is an empty list, with no optional fields included in the results.</p>
    pub fn set_included_properties(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::IncludedProperty>>,
    ) -> Self {
        self.inner = self.inner.set_included_properties(input);
        self
    }
    /// <p>An array of strings that specify which resources are included in the results of queries made using this view. When you use this view in a <code>Search</code> operation, the filter string is combined with the search's <code>QueryString</code> parameter using a logical <code>AND</code> operator.</p>
    /// <p>For information about the supported syntax, see <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html">Search query reference for Resource Explorer</a> in the <i>Amazon Web Services Resource Explorer User Guide</i>.</p> <important>
    /// <p>This query string in the context of this operation supports only <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html#query-syntax-filters">filter prefixes</a> with optional <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html#query-syntax-operators">operators</a>. It doesn't support free-form text. For example, the string <code>region:us* service:ec2 -tag:stage=prod</code> includes all Amazon EC2 resources in any Amazon Web Services Region that begins with the letters <code>us</code> and is <i>not</i> tagged with a key <code>Stage</code> that has the value <code>prod</code>.</p>
    /// </important>
    pub fn filters(mut self, input: crate::types::SearchFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>An array of strings that specify which resources are included in the results of queries made using this view. When you use this view in a <code>Search</code> operation, the filter string is combined with the search's <code>QueryString</code> parameter using a logical <code>AND</code> operator.</p>
    /// <p>For information about the supported syntax, see <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html">Search query reference for Resource Explorer</a> in the <i>Amazon Web Services Resource Explorer User Guide</i>.</p> <important>
    /// <p>This query string in the context of this operation supports only <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html#query-syntax-filters">filter prefixes</a> with optional <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/using-search-query-syntax.html#query-syntax-operators">operators</a>. It doesn't support free-form text. For example, the string <code>region:us* service:ec2 -tag:stage=prod</code> includes all Amazon EC2 resources in any Amazon Web Services Region that begins with the letters <code>us</code> and is <i>not</i> tagged with a key <code>Stage</code> that has the value <code>prod</code>.</p>
    /// </important>
    pub fn set_filters(mut self, input: std::option::Option<crate::types::SearchFilter>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
}