// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> A container specifying the time value for S3 Replication Time Control (S3 RTC) and replication metrics <code>EventThreshold</code>. </p>
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
pub struct ReplicationTimeValue {
    /// <p> Contains an integer specifying time in minutes. </p>
    /// <p> Valid value: 15</p>
    #[doc(hidden)]
    pub minutes: i32,
}
impl ReplicationTimeValue {
    /// <p> Contains an integer specifying time in minutes. </p>
    /// <p> Valid value: 15</p>
    pub fn minutes(&self) -> i32 {
        self.minutes
    }
}
impl ReplicationTimeValue {
    /// Creates a new builder-style object to manufacture [`ReplicationTimeValue`](crate::types::ReplicationTimeValue).
    pub fn builder() -> crate::types::builders::ReplicationTimeValueBuilder {
        crate::types::builders::ReplicationTimeValueBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ReplicationTimeValue;
/// A builder for [`ReplicationTimeValue`](crate::types::ReplicationTimeValue).
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
pub struct ReplicationTimeValueBuilder {
    pub(crate) minutes: std::option::Option<i32>,
}
impl ReplicationTimeValueBuilder {
    /// <p> Contains an integer specifying time in minutes. </p>
    /// <p> Valid value: 15</p>
    pub fn minutes(mut self, input: i32) -> Self {
        self.minutes = Some(input);
        self
    }
    /// <p> Contains an integer specifying time in minutes. </p>
    /// <p> Valid value: 15</p>
    pub fn set_minutes(mut self, input: std::option::Option<i32>) -> Self {
        self.minutes = input;
        self
    }
    /// Consumes the builder and constructs a [`ReplicationTimeValue`](crate::types::ReplicationTimeValue).
    pub fn build(self) -> crate::types::ReplicationTimeValue {
        crate::types::ReplicationTimeValue {
            minutes: self.minutes.unwrap_or_default(),
        }
    }
}