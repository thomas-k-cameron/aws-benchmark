// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A key-value pair. A tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p>
/// <p>For information about the rules that apply to tag keys and tag values, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>Amazon Web Services Billing and Cost Management User Guide</i>.</p>
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
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[doc(hidden)]
    pub tag_key: std::option::Option<std::string::String>,
    /// <p>The value of the tag.</p>
    #[doc(hidden)]
    pub tag_value: std::option::Option<std::string::String>,
}
impl Tag {
    /// <p>The key of the tag.</p>
    pub fn tag_key(&self) -> std::option::Option<&str> {
        self.tag_key.as_deref()
    }
    /// <p>The value of the tag.</p>
    pub fn tag_value(&self) -> std::option::Option<&str> {
        self.tag_value.as_deref()
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::types::Tag).
    pub fn builder() -> crate::types::builders::TagBuilder {
        crate::types::builders::TagBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Tag;
/// A builder for [`Tag`](crate::types::Tag).
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
pub struct TagBuilder {
    pub(crate) tag_key: std::option::Option<std::string::String>,
    pub(crate) tag_value: std::option::Option<std::string::String>,
}
impl TagBuilder {
    /// <p>The key of the tag.</p>
    pub fn tag_key(mut self, input: impl Into<std::string::String>) -> Self {
        self.tag_key = Some(input.into());
        self
    }
    /// <p>The key of the tag.</p>
    pub fn set_tag_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.tag_key = input;
        self
    }
    /// <p>The value of the tag.</p>
    pub fn tag_value(mut self, input: impl Into<std::string::String>) -> Self {
        self.tag_value = Some(input.into());
        self
    }
    /// <p>The value of the tag.</p>
    pub fn set_tag_value(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.tag_value = input;
        self
    }
    /// Consumes the builder and constructs a [`Tag`](crate::types::Tag).
    pub fn build(self) -> crate::types::Tag {
        crate::types::Tag {
            tag_key: self.tag_key,
            tag_value: self.tag_value,
        }
    }
}