// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides timing details for the job.</p>
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
pub struct JobTimers {
    /// <p>Indicates the elapsed time in seconds the job has been in the Active job state.</p>
    #[doc(hidden)]
    pub elapsed_time_in_active_seconds: std::option::Option<i64>,
}
impl JobTimers {
    /// <p>Indicates the elapsed time in seconds the job has been in the Active job state.</p>
    pub fn elapsed_time_in_active_seconds(&self) -> std::option::Option<i64> {
        self.elapsed_time_in_active_seconds
    }
}
impl JobTimers {
    /// Creates a new builder-style object to manufacture [`JobTimers`](crate::types::JobTimers).
    pub fn builder() -> crate::types::builders::JobTimersBuilder {
        crate::types::builders::JobTimersBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::JobTimers;
/// A builder for [`JobTimers`](crate::types::JobTimers).
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
pub struct JobTimersBuilder {
    pub(crate) elapsed_time_in_active_seconds: std::option::Option<i64>,
}
impl JobTimersBuilder {
    /// <p>Indicates the elapsed time in seconds the job has been in the Active job state.</p>
    pub fn elapsed_time_in_active_seconds(mut self, input: i64) -> Self {
        self.elapsed_time_in_active_seconds = Some(input);
        self
    }
    /// <p>Indicates the elapsed time in seconds the job has been in the Active job state.</p>
    pub fn set_elapsed_time_in_active_seconds(mut self, input: std::option::Option<i64>) -> Self {
        self.elapsed_time_in_active_seconds = input;
        self
    }
    /// Consumes the builder and constructs a [`JobTimers`](crate::types::JobTimers).
    pub fn build(self) -> crate::types::JobTimers {
        crate::types::JobTimers {
            elapsed_time_in_active_seconds: self.elapsed_time_in_active_seconds,
        }
    }
}
