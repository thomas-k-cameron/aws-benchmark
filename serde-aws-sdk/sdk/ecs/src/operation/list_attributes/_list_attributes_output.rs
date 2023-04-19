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
pub struct ListAttributesOutput {
    /// <p>A list of attribute objects that meet the criteria of the request.</p>
    #[doc(hidden)]
    pub attributes: std::option::Option<std::vec::Vec<crate::types::Attribute>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListAttributes</code> request. When the results of a <code>ListAttributes</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListAttributesOutput {
    /// <p>A list of attribute objects that meet the criteria of the request.</p>
    pub fn attributes(&self) -> std::option::Option<&[crate::types::Attribute]> {
        self.attributes.as_deref()
    }
    /// <p>The <code>nextToken</code> value to include in a future <code>ListAttributes</code> request. When the results of a <code>ListAttributes</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListAttributesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListAttributesOutput {
    /// Creates a new builder-style object to manufacture [`ListAttributesOutput`](crate::operation::list_attributes::ListAttributesOutput).
    pub fn builder() -> crate::operation::list_attributes::builders::ListAttributesOutputBuilder {
        crate::operation::list_attributes::builders::ListAttributesOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_attributes::ListAttributesOutput;
/// A builder for [`ListAttributesOutput`](crate::operation::list_attributes::ListAttributesOutput).
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
pub struct ListAttributesOutputBuilder {
    pub(crate) attributes: std::option::Option<std::vec::Vec<crate::types::Attribute>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListAttributesOutputBuilder {
    /// Appends an item to `attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>A list of attribute objects that meet the criteria of the request.</p>
    pub fn attributes(mut self, input: crate::types::Attribute) -> Self {
        let mut v = self.attributes.unwrap_or_default();
        v.push(input);
        self.attributes = Some(v);
        self
    }
    /// <p>A list of attribute objects that meet the criteria of the request.</p>
    pub fn set_attributes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Attribute>>,
    ) -> Self {
        self.attributes = input;
        self
    }
    /// <p>The <code>nextToken</code> value to include in a future <code>ListAttributes</code> request. When the results of a <code>ListAttributes</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The <code>nextToken</code> value to include in a future <code>ListAttributes</code> request. When the results of a <code>ListAttributes</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`ListAttributesOutput`](crate::operation::list_attributes::ListAttributesOutput).
    pub fn build(self) -> crate::operation::list_attributes::ListAttributesOutput {
        crate::operation::list_attributes::ListAttributesOutput {
            attributes: self.attributes,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}