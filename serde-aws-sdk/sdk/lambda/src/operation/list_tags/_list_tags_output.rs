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
pub struct ListTagsOutput {
    /// <p>The function's tags.</p>
    #[doc(hidden)]
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    _request_id: Option<String>,
}
impl ListTagsOutput {
    /// <p>The function's tags.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
impl aws_http::request_id::RequestId for ListTagsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListTagsOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsOutput`](crate::operation::list_tags::ListTagsOutput).
    pub fn builder() -> crate::operation::list_tags::builders::ListTagsOutputBuilder {
        crate::operation::list_tags::builders::ListTagsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_tags::ListTagsOutput;
/// A builder for [`ListTagsOutput`](crate::operation::list_tags::ListTagsOutput).
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
pub struct ListTagsOutputBuilder {
    pub(crate) tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    _request_id: Option<String>,
}
impl ListTagsOutputBuilder {
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The function's tags.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = Some(hash_map);
        self
    }
    /// <p>The function's tags.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
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
    /// Consumes the builder and constructs a [`ListTagsOutput`](crate::operation::list_tags::ListTagsOutput).
    pub fn build(self) -> crate::operation::list_tags::ListTagsOutput {
        crate::operation::list_tags::ListTagsOutput {
            tags: self.tags,
            _request_id: self._request_id,
        }
    }
}