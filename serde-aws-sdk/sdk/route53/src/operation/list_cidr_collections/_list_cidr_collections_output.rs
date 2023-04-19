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
pub struct ListCidrCollectionsOutput {
    /// <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>
    /// <p>If no value is provided, the listing of results starts from the beginning.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>A complex type with information about the CIDR collection.</p>
    #[doc(hidden)]
    pub cidr_collections: std::option::Option<std::vec::Vec<crate::types::CollectionSummary>>,
    _request_id: Option<String>,
}
impl ListCidrCollectionsOutput {
    /// <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>
    /// <p>If no value is provided, the listing of results starts from the beginning.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>A complex type with information about the CIDR collection.</p>
    pub fn cidr_collections(&self) -> std::option::Option<&[crate::types::CollectionSummary]> {
        self.cidr_collections.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListCidrCollectionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListCidrCollectionsOutput {
    /// Creates a new builder-style object to manufacture [`ListCidrCollectionsOutput`](crate::operation::list_cidr_collections::ListCidrCollectionsOutput).
    pub fn builder(
    ) -> crate::operation::list_cidr_collections::builders::ListCidrCollectionsOutputBuilder {
        crate::operation::list_cidr_collections::builders::ListCidrCollectionsOutputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_cidr_collections::ListCidrCollectionsOutput;
/// A builder for [`ListCidrCollectionsOutput`](crate::operation::list_cidr_collections::ListCidrCollectionsOutput).
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
pub struct ListCidrCollectionsOutputBuilder {
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) cidr_collections:
        std::option::Option<std::vec::Vec<crate::types::CollectionSummary>>,
    _request_id: Option<String>,
}
impl ListCidrCollectionsOutputBuilder {
    /// <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>
    /// <p>If no value is provided, the listing of results starts from the beginning.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>
    /// <p>If no value is provided, the listing of results starts from the beginning.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `cidr_collections`.
    ///
    /// To override the contents of this collection use [`set_cidr_collections`](Self::set_cidr_collections).
    ///
    /// <p>A complex type with information about the CIDR collection.</p>
    pub fn cidr_collections(mut self, input: crate::types::CollectionSummary) -> Self {
        let mut v = self.cidr_collections.unwrap_or_default();
        v.push(input);
        self.cidr_collections = Some(v);
        self
    }
    /// <p>A complex type with information about the CIDR collection.</p>
    pub fn set_cidr_collections(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::CollectionSummary>>,
    ) -> Self {
        self.cidr_collections = input;
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
    /// Consumes the builder and constructs a [`ListCidrCollectionsOutput`](crate::operation::list_cidr_collections::ListCidrCollectionsOutput).
    pub fn build(self) -> crate::operation::list_cidr_collections::ListCidrCollectionsOutput {
        crate::operation::list_cidr_collections::ListCidrCollectionsOutput {
            next_token: self.next_token,
            cidr_collections: self.cidr_collections,
            _request_id: self._request_id,
        }
    }
}