// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Configuration settings for creating and managing pre-provisioned snapshots for a fast-launch enabled Windows AMI.</p>
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
pub struct FastLaunchSnapshotConfigurationRequest {
    /// <p>The number of pre-provisioned snapshots to keep on hand for a fast-launch enabled Windows AMI.</p>
    #[doc(hidden)]
    pub target_resource_count: std::option::Option<i32>,
}
impl FastLaunchSnapshotConfigurationRequest {
    /// <p>The number of pre-provisioned snapshots to keep on hand for a fast-launch enabled Windows AMI.</p>
    pub fn target_resource_count(&self) -> std::option::Option<i32> {
        self.target_resource_count
    }
}
impl FastLaunchSnapshotConfigurationRequest {
    /// Creates a new builder-style object to manufacture [`FastLaunchSnapshotConfigurationRequest`](crate::types::FastLaunchSnapshotConfigurationRequest).
    pub fn builder() -> crate::types::builders::FastLaunchSnapshotConfigurationRequestBuilder {
        crate::types::builders::FastLaunchSnapshotConfigurationRequestBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::FastLaunchSnapshotConfigurationRequest;
/// A builder for [`FastLaunchSnapshotConfigurationRequest`](crate::types::FastLaunchSnapshotConfigurationRequest).
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
pub struct FastLaunchSnapshotConfigurationRequestBuilder {
    pub(crate) target_resource_count: std::option::Option<i32>,
}
impl FastLaunchSnapshotConfigurationRequestBuilder {
    /// <p>The number of pre-provisioned snapshots to keep on hand for a fast-launch enabled Windows AMI.</p>
    pub fn target_resource_count(mut self, input: i32) -> Self {
        self.target_resource_count = Some(input);
        self
    }
    /// <p>The number of pre-provisioned snapshots to keep on hand for a fast-launch enabled Windows AMI.</p>
    pub fn set_target_resource_count(mut self, input: std::option::Option<i32>) -> Self {
        self.target_resource_count = input;
        self
    }
    /// Consumes the builder and constructs a [`FastLaunchSnapshotConfigurationRequest`](crate::types::FastLaunchSnapshotConfigurationRequest).
    pub fn build(self) -> crate::types::FastLaunchSnapshotConfigurationRequest {
        crate::types::FastLaunchSnapshotConfigurationRequest {
            target_resource_count: self.target_resource_count,
        }
    }
}