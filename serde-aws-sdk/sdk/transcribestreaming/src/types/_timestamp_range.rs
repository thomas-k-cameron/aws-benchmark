// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the timestamp range (start time through end time) of a matched category.</p>
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
pub struct TimestampRange {
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the start of the category match.</p>
    #[doc(hidden)]
    pub begin_offset_millis: std::option::Option<i64>,
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the end of the category match.</p>
    #[doc(hidden)]
    pub end_offset_millis: std::option::Option<i64>,
}
impl TimestampRange {
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the start of the category match.</p>
    pub fn begin_offset_millis(&self) -> std::option::Option<i64> {
        self.begin_offset_millis
    }
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the end of the category match.</p>
    pub fn end_offset_millis(&self) -> std::option::Option<i64> {
        self.end_offset_millis
    }
}
impl TimestampRange {
    /// Creates a new builder-style object to manufacture [`TimestampRange`](crate::types::TimestampRange).
    pub fn builder() -> crate::types::builders::TimestampRangeBuilder {
        crate::types::builders::TimestampRangeBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TimestampRange;
/// A builder for [`TimestampRange`](crate::types::TimestampRange).
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
pub struct TimestampRangeBuilder {
    pub(crate) begin_offset_millis: std::option::Option<i64>,
    pub(crate) end_offset_millis: std::option::Option<i64>,
}
impl TimestampRangeBuilder {
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the start of the category match.</p>
    pub fn begin_offset_millis(mut self, input: i64) -> Self {
        self.begin_offset_millis = Some(input);
        self
    }
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the start of the category match.</p>
    pub fn set_begin_offset_millis(mut self, input: std::option::Option<i64>) -> Self {
        self.begin_offset_millis = input;
        self
    }
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the end of the category match.</p>
    pub fn end_offset_millis(mut self, input: i64) -> Self {
        self.end_offset_millis = Some(input);
        self
    }
    /// <p>The time, in milliseconds, from the beginning of the audio stream to the end of the category match.</p>
    pub fn set_end_offset_millis(mut self, input: std::option::Option<i64>) -> Self {
        self.end_offset_millis = input;
        self
    }
    /// Consumes the builder and constructs a [`TimestampRange`](crate::types::TimestampRange).
    pub fn build(self) -> crate::types::TimestampRange {
        crate::types::TimestampRange {
            begin_offset_millis: self.begin_offset_millis,
            end_offset_millis: self.end_offset_millis,
        }
    }
}