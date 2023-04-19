// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the details of the features enabled on the table when the backup was created. For example, LSIs, GSIs, streams, TTL. </p>
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
pub struct SourceTableFeatureDetails {
    /// <p>Represents the LSI properties for the table when the backup was created. It includes the IndexName, KeySchema and Projection for the LSIs on the table at the time of backup. </p>
    #[doc(hidden)]
    pub local_secondary_indexes:
        std::option::Option<std::vec::Vec<crate::types::LocalSecondaryIndexInfo>>,
    /// <p>Represents the GSI properties for the table when the backup was created. It includes the IndexName, KeySchema, Projection, and ProvisionedThroughput for the GSIs on the table at the time of backup. </p>
    #[doc(hidden)]
    pub global_secondary_indexes:
        std::option::Option<std::vec::Vec<crate::types::GlobalSecondaryIndexInfo>>,
    /// <p>Stream settings on the table when the backup was created.</p>
    #[doc(hidden)]
    pub stream_description: std::option::Option<crate::types::StreamSpecification>,
    /// <p>Time to Live settings on the table when the backup was created.</p>
    #[doc(hidden)]
    pub time_to_live_description: std::option::Option<crate::types::TimeToLiveDescription>,
    /// <p>The description of the server-side encryption status on the table when the backup was created.</p>
    #[doc(hidden)]
    pub sse_description: std::option::Option<crate::types::SseDescription>,
}
impl SourceTableFeatureDetails {
    /// <p>Represents the LSI properties for the table when the backup was created. It includes the IndexName, KeySchema and Projection for the LSIs on the table at the time of backup. </p>
    pub fn local_secondary_indexes(
        &self,
    ) -> std::option::Option<&[crate::types::LocalSecondaryIndexInfo]> {
        self.local_secondary_indexes.as_deref()
    }
    /// <p>Represents the GSI properties for the table when the backup was created. It includes the IndexName, KeySchema, Projection, and ProvisionedThroughput for the GSIs on the table at the time of backup. </p>
    pub fn global_secondary_indexes(
        &self,
    ) -> std::option::Option<&[crate::types::GlobalSecondaryIndexInfo]> {
        self.global_secondary_indexes.as_deref()
    }
    /// <p>Stream settings on the table when the backup was created.</p>
    pub fn stream_description(&self) -> std::option::Option<&crate::types::StreamSpecification> {
        self.stream_description.as_ref()
    }
    /// <p>Time to Live settings on the table when the backup was created.</p>
    pub fn time_to_live_description(
        &self,
    ) -> std::option::Option<&crate::types::TimeToLiveDescription> {
        self.time_to_live_description.as_ref()
    }
    /// <p>The description of the server-side encryption status on the table when the backup was created.</p>
    pub fn sse_description(&self) -> std::option::Option<&crate::types::SseDescription> {
        self.sse_description.as_ref()
    }
}
impl SourceTableFeatureDetails {
    /// Creates a new builder-style object to manufacture [`SourceTableFeatureDetails`](crate::types::SourceTableFeatureDetails).
    pub fn builder() -> crate::types::builders::SourceTableFeatureDetailsBuilder {
        crate::types::builders::SourceTableFeatureDetailsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::SourceTableFeatureDetails;
/// A builder for [`SourceTableFeatureDetails`](crate::types::SourceTableFeatureDetails).
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
pub struct SourceTableFeatureDetailsBuilder {
    pub(crate) local_secondary_indexes:
        std::option::Option<std::vec::Vec<crate::types::LocalSecondaryIndexInfo>>,
    pub(crate) global_secondary_indexes:
        std::option::Option<std::vec::Vec<crate::types::GlobalSecondaryIndexInfo>>,
    pub(crate) stream_description: std::option::Option<crate::types::StreamSpecification>,
    pub(crate) time_to_live_description: std::option::Option<crate::types::TimeToLiveDescription>,
    pub(crate) sse_description: std::option::Option<crate::types::SseDescription>,
}
impl SourceTableFeatureDetailsBuilder {
    /// Appends an item to `local_secondary_indexes`.
    ///
    /// To override the contents of this collection use [`set_local_secondary_indexes`](Self::set_local_secondary_indexes).
    ///
    /// <p>Represents the LSI properties for the table when the backup was created. It includes the IndexName, KeySchema and Projection for the LSIs on the table at the time of backup. </p>
    pub fn local_secondary_indexes(mut self, input: crate::types::LocalSecondaryIndexInfo) -> Self {
        let mut v = self.local_secondary_indexes.unwrap_or_default();
        v.push(input);
        self.local_secondary_indexes = Some(v);
        self
    }
    /// <p>Represents the LSI properties for the table when the backup was created. It includes the IndexName, KeySchema and Projection for the LSIs on the table at the time of backup. </p>
    pub fn set_local_secondary_indexes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::LocalSecondaryIndexInfo>>,
    ) -> Self {
        self.local_secondary_indexes = input;
        self
    }
    /// Appends an item to `global_secondary_indexes`.
    ///
    /// To override the contents of this collection use [`set_global_secondary_indexes`](Self::set_global_secondary_indexes).
    ///
    /// <p>Represents the GSI properties for the table when the backup was created. It includes the IndexName, KeySchema, Projection, and ProvisionedThroughput for the GSIs on the table at the time of backup. </p>
    pub fn global_secondary_indexes(
        mut self,
        input: crate::types::GlobalSecondaryIndexInfo,
    ) -> Self {
        let mut v = self.global_secondary_indexes.unwrap_or_default();
        v.push(input);
        self.global_secondary_indexes = Some(v);
        self
    }
    /// <p>Represents the GSI properties for the table when the backup was created. It includes the IndexName, KeySchema, Projection, and ProvisionedThroughput for the GSIs on the table at the time of backup. </p>
    pub fn set_global_secondary_indexes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::GlobalSecondaryIndexInfo>>,
    ) -> Self {
        self.global_secondary_indexes = input;
        self
    }
    /// <p>Stream settings on the table when the backup was created.</p>
    pub fn stream_description(mut self, input: crate::types::StreamSpecification) -> Self {
        self.stream_description = Some(input);
        self
    }
    /// <p>Stream settings on the table when the backup was created.</p>
    pub fn set_stream_description(
        mut self,
        input: std::option::Option<crate::types::StreamSpecification>,
    ) -> Self {
        self.stream_description = input;
        self
    }
    /// <p>Time to Live settings on the table when the backup was created.</p>
    pub fn time_to_live_description(mut self, input: crate::types::TimeToLiveDescription) -> Self {
        self.time_to_live_description = Some(input);
        self
    }
    /// <p>Time to Live settings on the table when the backup was created.</p>
    pub fn set_time_to_live_description(
        mut self,
        input: std::option::Option<crate::types::TimeToLiveDescription>,
    ) -> Self {
        self.time_to_live_description = input;
        self
    }
    /// <p>The description of the server-side encryption status on the table when the backup was created.</p>
    pub fn sse_description(mut self, input: crate::types::SseDescription) -> Self {
        self.sse_description = Some(input);
        self
    }
    /// <p>The description of the server-side encryption status on the table when the backup was created.</p>
    pub fn set_sse_description(
        mut self,
        input: std::option::Option<crate::types::SseDescription>,
    ) -> Self {
        self.sse_description = input;
        self
    }
    /// Consumes the builder and constructs a [`SourceTableFeatureDetails`](crate::types::SourceTableFeatureDetails).
    pub fn build(self) -> crate::types::SourceTableFeatureDetails {
        crate::types::SourceTableFeatureDetails {
            local_secondary_indexes: self.local_secondary_indexes,
            global_secondary_indexes: self.global_secondary_indexes,
            stream_description: self.stream_description,
            time_to_live_description: self.time_to_live_description,
            sse_description: self.sse_description,
        }
    }
}