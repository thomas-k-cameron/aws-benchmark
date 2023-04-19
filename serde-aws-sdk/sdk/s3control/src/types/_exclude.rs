// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container for what Amazon S3 Storage Lens will exclude.</p>
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
pub struct Exclude {
    /// <p>A container for the S3 Storage Lens bucket excludes.</p>
    #[doc(hidden)]
    pub buckets: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>A container for the S3 Storage Lens Region excludes.</p>
    #[doc(hidden)]
    pub regions: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl Exclude {
    /// <p>A container for the S3 Storage Lens bucket excludes.</p>
    pub fn buckets(&self) -> std::option::Option<&[std::string::String]> {
        self.buckets.as_deref()
    }
    /// <p>A container for the S3 Storage Lens Region excludes.</p>
    pub fn regions(&self) -> std::option::Option<&[std::string::String]> {
        self.regions.as_deref()
    }
}
impl Exclude {
    /// Creates a new builder-style object to manufacture [`Exclude`](crate::types::Exclude).
    pub fn builder() -> crate::types::builders::ExcludeBuilder {
        crate::types::builders::ExcludeBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Exclude;
/// A builder for [`Exclude`](crate::types::Exclude).
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
pub struct ExcludeBuilder {
    pub(crate) buckets: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) regions: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl ExcludeBuilder {
    /// Appends an item to `buckets`.
    ///
    /// To override the contents of this collection use [`set_buckets`](Self::set_buckets).
    ///
    /// <p>A container for the S3 Storage Lens bucket excludes.</p>
    pub fn buckets(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.buckets.unwrap_or_default();
        v.push(input.into());
        self.buckets = Some(v);
        self
    }
    /// <p>A container for the S3 Storage Lens bucket excludes.</p>
    pub fn set_buckets(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.buckets = input;
        self
    }
    /// Appends an item to `regions`.
    ///
    /// To override the contents of this collection use [`set_regions`](Self::set_regions).
    ///
    /// <p>A container for the S3 Storage Lens Region excludes.</p>
    pub fn regions(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.regions.unwrap_or_default();
        v.push(input.into());
        self.regions = Some(v);
        self
    }
    /// <p>A container for the S3 Storage Lens Region excludes.</p>
    pub fn set_regions(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.regions = input;
        self
    }
    /// Consumes the builder and constructs a [`Exclude`](crate::types::Exclude).
    pub fn build(self) -> crate::types::Exclude {
        crate::types::Exclude {
            buckets: self.buckets,
            regions: self.regions,
        }
    }
}