// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes Amazon Kinesis Data Firehose logging options.</p>
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
pub struct VerifiedAccessLogKinesisDataFirehoseDestinationOptions {
    /// <p>Indicates whether logging is enabled.</p>
    #[doc(hidden)]
    pub enabled: std::option::Option<bool>,
    /// <p>The ID of the delivery stream.</p>
    #[doc(hidden)]
    pub delivery_stream: std::option::Option<std::string::String>,
}
impl VerifiedAccessLogKinesisDataFirehoseDestinationOptions {
    /// <p>Indicates whether logging is enabled.</p>
    pub fn enabled(&self) -> std::option::Option<bool> {
        self.enabled
    }
    /// <p>The ID of the delivery stream.</p>
    pub fn delivery_stream(&self) -> std::option::Option<&str> {
        self.delivery_stream.as_deref()
    }
}
impl VerifiedAccessLogKinesisDataFirehoseDestinationOptions {
    /// Creates a new builder-style object to manufacture [`VerifiedAccessLogKinesisDataFirehoseDestinationOptions`](crate::types::VerifiedAccessLogKinesisDataFirehoseDestinationOptions).
    pub fn builder(
    ) -> crate::types::builders::VerifiedAccessLogKinesisDataFirehoseDestinationOptionsBuilder {
        crate::types::builders::VerifiedAccessLogKinesisDataFirehoseDestinationOptionsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::VerifiedAccessLogKinesisDataFirehoseDestinationOptions;
/// A builder for [`VerifiedAccessLogKinesisDataFirehoseDestinationOptions`](crate::types::VerifiedAccessLogKinesisDataFirehoseDestinationOptions).
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
pub struct VerifiedAccessLogKinesisDataFirehoseDestinationOptionsBuilder {
    pub(crate) enabled: std::option::Option<bool>,
    pub(crate) delivery_stream: std::option::Option<std::string::String>,
}
impl VerifiedAccessLogKinesisDataFirehoseDestinationOptionsBuilder {
    /// <p>Indicates whether logging is enabled.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = Some(input);
        self
    }
    /// <p>Indicates whether logging is enabled.</p>
    pub fn set_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>The ID of the delivery stream.</p>
    pub fn delivery_stream(mut self, input: impl Into<std::string::String>) -> Self {
        self.delivery_stream = Some(input.into());
        self
    }
    /// <p>The ID of the delivery stream.</p>
    pub fn set_delivery_stream(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.delivery_stream = input;
        self
    }
    /// Consumes the builder and constructs a [`VerifiedAccessLogKinesisDataFirehoseDestinationOptions`](crate::types::VerifiedAccessLogKinesisDataFirehoseDestinationOptions).
    pub fn build(self) -> crate::types::VerifiedAccessLogKinesisDataFirehoseDestinationOptions {
        crate::types::VerifiedAccessLogKinesisDataFirehoseDestinationOptions {
            enabled: self.enabled,
            delivery_stream: self.delivery_stream,
        }
    }
}