// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBucketVersioning`](crate::operation::get_bucket_versioning::builders::GetBucketVersioningFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::get_bucket_versioning::builders::GetBucketVersioningFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_bucket_versioning::builders::GetBucketVersioningFluentBuilder::set_bucket): <p>The name of the bucket for which to get the versioning information.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::get_bucket_versioning::builders::GetBucketVersioningFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::get_bucket_versioning::builders::GetBucketVersioningFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`GetBucketVersioningOutput`](crate::operation::get_bucket_versioning::GetBucketVersioningOutput) with field(s):
    ///   - [`status(Option<BucketVersioningStatus>)`](crate::operation::get_bucket_versioning::GetBucketVersioningOutput::status): <p>The versioning state of the bucket.</p>
    ///   - [`mfa_delete(Option<MfaDeleteStatus>)`](crate::operation::get_bucket_versioning::GetBucketVersioningOutput::mfa_delete): <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is only returned if the bucket has been configured with MFA delete. If the bucket has never been so configured, this element is not returned.</p>
    /// - On failure, responds with [`SdkError<GetBucketVersioningError>`](crate::operation::get_bucket_versioning::GetBucketVersioningError)
    pub fn get_bucket_versioning(
        &self,
    ) -> crate::operation::get_bucket_versioning::builders::GetBucketVersioningFluentBuilder {
        crate::operation::get_bucket_versioning::builders::GetBucketVersioningFluentBuilder::new(
            self.handle.clone(),
        )
    }
}