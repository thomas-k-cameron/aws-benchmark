// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p></p>
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
pub struct Tagging {
    /// <p>A collection for a set of tags.</p>
    #[doc(hidden)]
    pub tag_set: std::option::Option<std::vec::Vec<crate::types::S3Tag>>,
}
impl Tagging {
    /// <p>A collection for a set of tags.</p>
    pub fn tag_set(&self) -> std::option::Option<&[crate::types::S3Tag]> {
        self.tag_set.as_deref()
    }
}
impl Tagging {
    /// Creates a new builder-style object to manufacture [`Tagging`](crate::types::Tagging).
    pub fn builder() -> crate::types::builders::TaggingBuilder {
        crate::types::builders::TaggingBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Tagging;
/// A builder for [`Tagging`](crate::types::Tagging).
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
pub struct TaggingBuilder {
    pub(crate) tag_set: std::option::Option<std::vec::Vec<crate::types::S3Tag>>,
}
impl TaggingBuilder {
    /// Appends an item to `tag_set`.
    ///
    /// To override the contents of this collection use [`set_tag_set`](Self::set_tag_set).
    ///
    /// <p>A collection for a set of tags.</p>
    pub fn tag_set(mut self, input: crate::types::S3Tag) -> Self {
        let mut v = self.tag_set.unwrap_or_default();
        v.push(input);
        self.tag_set = Some(v);
        self
    }
    /// <p>A collection for a set of tags.</p>
    pub fn set_tag_set(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::S3Tag>>,
    ) -> Self {
        self.tag_set = input;
        self
    }
    /// Consumes the builder and constructs a [`Tagging`](crate::types::Tagging).
    pub fn build(self) -> crate::types::Tagging {
        crate::types::Tagging {
            tag_set: self.tag_set,
        }
    }
}