// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container that describes additional filters for identifying the source objects that you want to replicate. You can choose to enable or disable the replication of these objects.</p>
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
pub struct SourceSelectionCriteria {
    /// <p>A filter that you can use to select Amazon S3 objects that are encrypted with server-side encryption by using Key Management Service (KMS) keys. If you include <code>SourceSelectionCriteria</code> in the replication configuration, this element is required. </p> <note>
    /// <p>This is not supported by Amazon S3 on Outposts buckets.</p>
    /// </note>
    #[doc(hidden)]
    pub sse_kms_encrypted_objects: std::option::Option<crate::types::SseKmsEncryptedObjects>,
    /// <p>A filter that you can use to specify whether replica modification sync is enabled. S3 on Outposts replica modification sync can help you keep object metadata synchronized between replicas and source objects. By default, S3 on Outposts replicates metadata from the source objects to the replicas only. When replica modification sync is enabled, S3 on Outposts replicates metadata changes made to the replica copies back to the source object, making the replication bidirectional.</p>
    /// <p>To replicate object metadata modifications on replicas, you can specify this element and set the <code>Status</code> of this element to <code>Enabled</code>.</p> <note>
    /// <p>You must enable replica modification sync on the source and destination buckets to replicate replica metadata changes between the source and the replicas.</p>
    /// </note>
    #[doc(hidden)]
    pub replica_modifications: std::option::Option<crate::types::ReplicaModifications>,
}
impl SourceSelectionCriteria {
    /// <p>A filter that you can use to select Amazon S3 objects that are encrypted with server-side encryption by using Key Management Service (KMS) keys. If you include <code>SourceSelectionCriteria</code> in the replication configuration, this element is required. </p> <note>
    /// <p>This is not supported by Amazon S3 on Outposts buckets.</p>
    /// </note>
    pub fn sse_kms_encrypted_objects(
        &self,
    ) -> std::option::Option<&crate::types::SseKmsEncryptedObjects> {
        self.sse_kms_encrypted_objects.as_ref()
    }
    /// <p>A filter that you can use to specify whether replica modification sync is enabled. S3 on Outposts replica modification sync can help you keep object metadata synchronized between replicas and source objects. By default, S3 on Outposts replicates metadata from the source objects to the replicas only. When replica modification sync is enabled, S3 on Outposts replicates metadata changes made to the replica copies back to the source object, making the replication bidirectional.</p>
    /// <p>To replicate object metadata modifications on replicas, you can specify this element and set the <code>Status</code> of this element to <code>Enabled</code>.</p> <note>
    /// <p>You must enable replica modification sync on the source and destination buckets to replicate replica metadata changes between the source and the replicas.</p>
    /// </note>
    pub fn replica_modifications(
        &self,
    ) -> std::option::Option<&crate::types::ReplicaModifications> {
        self.replica_modifications.as_ref()
    }
}
impl SourceSelectionCriteria {
    /// Creates a new builder-style object to manufacture [`SourceSelectionCriteria`](crate::types::SourceSelectionCriteria).
    pub fn builder() -> crate::types::builders::SourceSelectionCriteriaBuilder {
        crate::types::builders::SourceSelectionCriteriaBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::SourceSelectionCriteria;
/// A builder for [`SourceSelectionCriteria`](crate::types::SourceSelectionCriteria).
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
pub struct SourceSelectionCriteriaBuilder {
    pub(crate) sse_kms_encrypted_objects: std::option::Option<crate::types::SseKmsEncryptedObjects>,
    pub(crate) replica_modifications: std::option::Option<crate::types::ReplicaModifications>,
}
impl SourceSelectionCriteriaBuilder {
    /// <p>A filter that you can use to select Amazon S3 objects that are encrypted with server-side encryption by using Key Management Service (KMS) keys. If you include <code>SourceSelectionCriteria</code> in the replication configuration, this element is required. </p> <note>
    /// <p>This is not supported by Amazon S3 on Outposts buckets.</p>
    /// </note>
    pub fn sse_kms_encrypted_objects(
        mut self,
        input: crate::types::SseKmsEncryptedObjects,
    ) -> Self {
        self.sse_kms_encrypted_objects = Some(input);
        self
    }
    /// <p>A filter that you can use to select Amazon S3 objects that are encrypted with server-side encryption by using Key Management Service (KMS) keys. If you include <code>SourceSelectionCriteria</code> in the replication configuration, this element is required. </p> <note>
    /// <p>This is not supported by Amazon S3 on Outposts buckets.</p>
    /// </note>
    pub fn set_sse_kms_encrypted_objects(
        mut self,
        input: std::option::Option<crate::types::SseKmsEncryptedObjects>,
    ) -> Self {
        self.sse_kms_encrypted_objects = input;
        self
    }
    /// <p>A filter that you can use to specify whether replica modification sync is enabled. S3 on Outposts replica modification sync can help you keep object metadata synchronized between replicas and source objects. By default, S3 on Outposts replicates metadata from the source objects to the replicas only. When replica modification sync is enabled, S3 on Outposts replicates metadata changes made to the replica copies back to the source object, making the replication bidirectional.</p>
    /// <p>To replicate object metadata modifications on replicas, you can specify this element and set the <code>Status</code> of this element to <code>Enabled</code>.</p> <note>
    /// <p>You must enable replica modification sync on the source and destination buckets to replicate replica metadata changes between the source and the replicas.</p>
    /// </note>
    pub fn replica_modifications(mut self, input: crate::types::ReplicaModifications) -> Self {
        self.replica_modifications = Some(input);
        self
    }
    /// <p>A filter that you can use to specify whether replica modification sync is enabled. S3 on Outposts replica modification sync can help you keep object metadata synchronized between replicas and source objects. By default, S3 on Outposts replicates metadata from the source objects to the replicas only. When replica modification sync is enabled, S3 on Outposts replicates metadata changes made to the replica copies back to the source object, making the replication bidirectional.</p>
    /// <p>To replicate object metadata modifications on replicas, you can specify this element and set the <code>Status</code> of this element to <code>Enabled</code>.</p> <note>
    /// <p>You must enable replica modification sync on the source and destination buckets to replicate replica metadata changes between the source and the replicas.</p>
    /// </note>
    pub fn set_replica_modifications(
        mut self,
        input: std::option::Option<crate::types::ReplicaModifications>,
    ) -> Self {
        self.replica_modifications = input;
        self
    }
    /// Consumes the builder and constructs a [`SourceSelectionCriteria`](crate::types::SourceSelectionCriteria).
    pub fn build(self) -> crate::types::SourceSelectionCriteria {
        crate::types::SourceSelectionCriteria {
            sse_kms_encrypted_objects: self.sse_kms_encrypted_objects,
            replica_modifications: self.replica_modifications,
        }
    }
}