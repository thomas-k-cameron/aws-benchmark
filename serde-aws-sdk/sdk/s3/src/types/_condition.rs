// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container for describing a condition that must be met for the specified redirect to apply. For example, 1. If request is for pages in the <code>/docs</code> folder, redirect to the <code>/documents</code> folder. 2. If request results in HTTP error 4xx, redirect request to another host where you might process the error.</p>
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
pub struct Condition {
    /// <p>The HTTP error code when the redirect is applied. In the event of an error, if the error code equals this value, then the specified redirect is applied. Required when parent element <code>Condition</code> is specified and sibling <code>KeyPrefixEquals</code> is not specified. If both are specified, then both must be true for the redirect to be applied.</p>
    #[doc(hidden)]
    pub http_error_code_returned_equals: std::option::Option<std::string::String>,
    /// <p>The object key name prefix when the redirect is applied. For example, to redirect requests for <code>ExamplePage.html</code>, the key prefix will be <code>ExamplePage.html</code>. To redirect request for all pages with the prefix <code>docs/</code>, the key prefix will be <code>/docs</code>, which identifies all objects in the <code>docs/</code> folder. Required when the parent element <code>Condition</code> is specified and sibling <code>HttpErrorCodeReturnedEquals</code> is not specified. If both conditions are specified, both must be true for the redirect to be applied.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    #[doc(hidden)]
    pub key_prefix_equals: std::option::Option<std::string::String>,
}
impl Condition {
    /// <p>The HTTP error code when the redirect is applied. In the event of an error, if the error code equals this value, then the specified redirect is applied. Required when parent element <code>Condition</code> is specified and sibling <code>KeyPrefixEquals</code> is not specified. If both are specified, then both must be true for the redirect to be applied.</p>
    pub fn http_error_code_returned_equals(&self) -> std::option::Option<&str> {
        self.http_error_code_returned_equals.as_deref()
    }
    /// <p>The object key name prefix when the redirect is applied. For example, to redirect requests for <code>ExamplePage.html</code>, the key prefix will be <code>ExamplePage.html</code>. To redirect request for all pages with the prefix <code>docs/</code>, the key prefix will be <code>/docs</code>, which identifies all objects in the <code>docs/</code> folder. Required when the parent element <code>Condition</code> is specified and sibling <code>HttpErrorCodeReturnedEquals</code> is not specified. If both conditions are specified, both must be true for the redirect to be applied.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn key_prefix_equals(&self) -> std::option::Option<&str> {
        self.key_prefix_equals.as_deref()
    }
}
impl Condition {
    /// Creates a new builder-style object to manufacture [`Condition`](crate::types::Condition).
    pub fn builder() -> crate::types::builders::ConditionBuilder {
        crate::types::builders::ConditionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Condition;
/// A builder for [`Condition`](crate::types::Condition).
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
pub struct ConditionBuilder {
    pub(crate) http_error_code_returned_equals: std::option::Option<std::string::String>,
    pub(crate) key_prefix_equals: std::option::Option<std::string::String>,
}
impl ConditionBuilder {
    /// <p>The HTTP error code when the redirect is applied. In the event of an error, if the error code equals this value, then the specified redirect is applied. Required when parent element <code>Condition</code> is specified and sibling <code>KeyPrefixEquals</code> is not specified. If both are specified, then both must be true for the redirect to be applied.</p>
    pub fn http_error_code_returned_equals(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.http_error_code_returned_equals = Some(input.into());
        self
    }
    /// <p>The HTTP error code when the redirect is applied. In the event of an error, if the error code equals this value, then the specified redirect is applied. Required when parent element <code>Condition</code> is specified and sibling <code>KeyPrefixEquals</code> is not specified. If both are specified, then both must be true for the redirect to be applied.</p>
    pub fn set_http_error_code_returned_equals(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.http_error_code_returned_equals = input;
        self
    }
    /// <p>The object key name prefix when the redirect is applied. For example, to redirect requests for <code>ExamplePage.html</code>, the key prefix will be <code>ExamplePage.html</code>. To redirect request for all pages with the prefix <code>docs/</code>, the key prefix will be <code>/docs</code>, which identifies all objects in the <code>docs/</code> folder. Required when the parent element <code>Condition</code> is specified and sibling <code>HttpErrorCodeReturnedEquals</code> is not specified. If both conditions are specified, both must be true for the redirect to be applied.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn key_prefix_equals(mut self, input: impl Into<std::string::String>) -> Self {
        self.key_prefix_equals = Some(input.into());
        self
    }
    /// <p>The object key name prefix when the redirect is applied. For example, to redirect requests for <code>ExamplePage.html</code>, the key prefix will be <code>ExamplePage.html</code>. To redirect request for all pages with the prefix <code>docs/</code>, the key prefix will be <code>/docs</code>, which identifies all objects in the <code>docs/</code> folder. Required when the parent element <code>Condition</code> is specified and sibling <code>HttpErrorCodeReturnedEquals</code> is not specified. If both conditions are specified, both must be true for the redirect to be applied.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn set_key_prefix_equals(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.key_prefix_equals = input;
        self
    }
    /// Consumes the builder and constructs a [`Condition`](crate::types::Condition).
    pub fn build(self) -> crate::types::Condition {
        crate::types::Condition {
            http_error_code_returned_equals: self.http_error_code_returned_equals,
            key_prefix_equals: self.key_prefix_equals,
        }
    }
}