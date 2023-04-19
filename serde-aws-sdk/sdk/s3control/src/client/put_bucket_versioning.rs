// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutBucketVersioning`](crate::operation::put_bucket_versioning::builders::PutBucketVersioningFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::put_bucket_versioning::builders::PutBucketVersioningFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::put_bucket_versioning::builders::PutBucketVersioningFluentBuilder::set_account_id): <p>The Amazon Web Services account ID of the S3 on Outposts bucket.</p>
    ///   - [`bucket(impl Into<String>)`](crate::operation::put_bucket_versioning::builders::PutBucketVersioningFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::put_bucket_versioning::builders::PutBucketVersioningFluentBuilder::set_bucket): <p>The S3 on Outposts bucket to set the versioning state for.</p>
    ///   - [`mfa(impl Into<String>)`](crate::operation::put_bucket_versioning::builders::PutBucketVersioningFluentBuilder::mfa) / [`set_mfa(Option<String>)`](crate::operation::put_bucket_versioning::builders::PutBucketVersioningFluentBuilder::set_mfa): <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    ///   - [`versioning_configuration(VersioningConfiguration)`](crate::operation::put_bucket_versioning::builders::PutBucketVersioningFluentBuilder::versioning_configuration) / [`set_versioning_configuration(Option<VersioningConfiguration>)`](crate::operation::put_bucket_versioning::builders::PutBucketVersioningFluentBuilder::set_versioning_configuration): <p>The root-level tag for the <code>VersioningConfiguration</code> parameters.</p>
    /// - On success, responds with [`PutBucketVersioningOutput`](crate::operation::put_bucket_versioning::PutBucketVersioningOutput)
    /// - On failure, responds with [`SdkError<PutBucketVersioningError>`](crate::operation::put_bucket_versioning::PutBucketVersioningError)
    pub fn put_bucket_versioning(
        &self,
    ) -> crate::operation::put_bucket_versioning::builders::PutBucketVersioningFluentBuilder {
        crate::operation::put_bucket_versioning::builders::PutBucketVersioningFluentBuilder::new(
            self.handle.clone(),
        )
    }
}