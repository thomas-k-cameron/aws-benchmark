// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBucketAcl`](crate::operation::get_bucket_acl::builders::GetBucketAclFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::get_bucket_acl::builders::GetBucketAclFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_bucket_acl::builders::GetBucketAclFluentBuilder::set_bucket): <p>Specifies the S3 bucket whose ACL is being requested.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::get_bucket_acl::builders::GetBucketAclFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::get_bucket_acl::builders::GetBucketAclFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`GetBucketAclOutput`](crate::operation::get_bucket_acl::GetBucketAclOutput) with field(s):
    ///   - [`owner(Option<Owner>)`](crate::operation::get_bucket_acl::GetBucketAclOutput::owner): <p>Container for the bucket owner's display name and ID.</p>
    ///   - [`grants(Option<Vec<Grant>>)`](crate::operation::get_bucket_acl::GetBucketAclOutput::grants): <p>A list of grants.</p>
    /// - On failure, responds with [`SdkError<GetBucketAclError>`](crate::operation::get_bucket_acl::GetBucketAclError)
    pub fn get_bucket_acl(
        &self,
    ) -> crate::operation::get_bucket_acl::builders::GetBucketAclFluentBuilder {
        crate::operation::get_bucket_acl::builders::GetBucketAclFluentBuilder::new(
            self.handle.clone(),
        )
    }
}