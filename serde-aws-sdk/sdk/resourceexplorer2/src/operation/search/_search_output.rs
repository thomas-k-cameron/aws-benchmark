// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct SearchOutput {
    /// <p>The list of structures that describe the resources that match the query.</p>
    #[doc(hidden)]
    pub resources: std::option::Option<std::vec::Vec<crate::types::Resource>>,
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that this operation used to perform the search.</p>
    #[doc(hidden)]
    pub view_arn: std::option::Option<std::string::String>,
    /// <p>The number of resources that match the query.</p>
    #[doc(hidden)]
    pub count: std::option::Option<crate::types::ResourceCount>,
    _request_id: Option<String>,
}
impl SearchOutput {
    /// <p>The list of structures that describe the resources that match the query.</p>
    pub fn resources(&self) -> std::option::Option<&[crate::types::Resource]> {
        self.resources.as_deref()
    }
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that this operation used to perform the search.</p>
    pub fn view_arn(&self) -> std::option::Option<&str> {
        self.view_arn.as_deref()
    }
    /// <p>The number of resources that match the query.</p>
    pub fn count(&self) -> std::option::Option<&crate::types::ResourceCount> {
        self.count.as_ref()
    }
}
impl aws_http::request_id::RequestId for SearchOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl SearchOutput {
    /// Creates a new builder-style object to manufacture [`SearchOutput`](crate::operation::search::SearchOutput).
    pub fn builder() -> crate::operation::search::builders::SearchOutputBuilder {
        crate::operation::search::builders::SearchOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::search::SearchOutput;
/// A builder for [`SearchOutput`](crate::operation::search::SearchOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct SearchOutputBuilder {
    pub(crate) resources: std::option::Option<std::vec::Vec<crate::types::Resource>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) view_arn: std::option::Option<std::string::String>,
    pub(crate) count: std::option::Option<crate::types::ResourceCount>,
    _request_id: Option<String>,
}
impl SearchOutputBuilder {
    /// Appends an item to `resources`.
    ///
    /// To override the contents of this collection use [`set_resources`](Self::set_resources).
    ///
    /// <p>The list of structures that describe the resources that match the query.</p>
    pub fn resources(mut self, input: crate::types::Resource) -> Self {
        let mut v = self.resources.unwrap_or_default();
        v.push(input);
        self.resources = Some(v);
        self
    }
    /// <p>The list of structures that describe the resources that match the query.</p>
    pub fn set_resources(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Resource>>,
    ) -> Self {
        self.resources = input;
        self
    }
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>If present, indicates that more output is available than is included in the current response. Use this value in the <code>NextToken</code> request parameter in a subsequent call to the operation to get the next part of the output. You should repeat this until the <code>NextToken</code> response element comes back as <code>null</code>.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that this operation used to perform the search.</p>
    pub fn view_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.view_arn = Some(input.into());
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view that this operation used to perform the search.</p>
    pub fn set_view_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.view_arn = input;
        self
    }
    /// <p>The number of resources that match the query.</p>
    pub fn count(mut self, input: crate::types::ResourceCount) -> Self {
        self.count = Some(input);
        self
    }
    /// <p>The number of resources that match the query.</p>
    pub fn set_count(mut self, input: std::option::Option<crate::types::ResourceCount>) -> Self {
        self.count = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`SearchOutput`](crate::operation::search::SearchOutput).
    pub fn build(self) -> crate::operation::search::SearchOutput {
        crate::operation::search::SearchOutput {
            resources: self.resources,
            next_token: self.next_token,
            view_arn: self.view_arn,
            count: self.count,
            _request_id: self._request_id,
        }
    }
}