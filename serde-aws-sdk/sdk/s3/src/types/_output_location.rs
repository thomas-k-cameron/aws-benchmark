// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the location where the restore job's output is stored.</p>
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
pub struct OutputLocation {
    /// <p>Describes an S3 location that will receive the results of the restore request.</p>
    #[doc(hidden)]
    pub s3: std::option::Option<crate::types::S3Location>,
}
impl OutputLocation {
    /// <p>Describes an S3 location that will receive the results of the restore request.</p>
    pub fn s3(&self) -> std::option::Option<&crate::types::S3Location> {
        self.s3.as_ref()
    }
}
impl OutputLocation {
    /// Creates a new builder-style object to manufacture [`OutputLocation`](crate::types::OutputLocation).
    pub fn builder() -> crate::types::builders::OutputLocationBuilder {
        crate::types::builders::OutputLocationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::OutputLocation;
/// A builder for [`OutputLocation`](crate::types::OutputLocation).
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
pub struct OutputLocationBuilder {
    pub(crate) s3: std::option::Option<crate::types::S3Location>,
}
impl OutputLocationBuilder {
    /// <p>Describes an S3 location that will receive the results of the restore request.</p>
    pub fn s3(mut self, input: crate::types::S3Location) -> Self {
        self.s3 = Some(input);
        self
    }
    /// <p>Describes an S3 location that will receive the results of the restore request.</p>
    pub fn set_s3(mut self, input: std::option::Option<crate::types::S3Location>) -> Self {
        self.s3 = input;
        self
    }
    /// Consumes the builder and constructs a [`OutputLocation`](crate::types::OutputLocation).
    pub fn build(self) -> crate::types::OutputLocation {
        crate::types::OutputLocation { s3: self.s3 }
    }
}
