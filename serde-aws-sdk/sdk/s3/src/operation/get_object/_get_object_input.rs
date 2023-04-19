// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetObjectInput {
    /// <p>The bucket name containing the object. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using an Object Lambda access point the hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-object-lambda.<i>Region</i>.amazonaws.com.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>Return the object only if its entity tag (ETag) is the same as the one specified; otherwise, return a 412 (precondition failed) error.</p>
    #[doc(hidden)]
    pub if_match: std::option::Option<std::string::String>,
    /// <p>Return the object only if it has been modified since the specified time; otherwise, return a 304 (not modified) error.</p>
    #[doc(hidden)]
    pub if_modified_since: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Return the object only if its entity tag (ETag) is different from the one specified; otherwise, return a 304 (not modified) error.</p>
    #[doc(hidden)]
    pub if_none_match: std::option::Option<std::string::String>,
    /// <p>Return the object only if it has not been modified since the specified time; otherwise, return a 412 (precondition failed) error.</p>
    #[doc(hidden)]
    pub if_unmodified_since: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Key of the object to get.</p>
    #[doc(hidden)]
    pub key: std::option::Option<std::string::String>,
    /// <p>Downloads the specified range bytes of an object. For more information about the HTTP Range header, see <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35</a>.</p> <note>
    /// <p>Amazon S3 doesn't support retrieving multiple ranges of data per <code>GET</code> request.</p>
    /// </note>
    #[doc(hidden)]
    pub range: std::option::Option<std::string::String>,
    /// <p>Sets the <code>Cache-Control</code> header of the response.</p>
    #[doc(hidden)]
    pub response_cache_control: std::option::Option<std::string::String>,
    /// <p>Sets the <code>Content-Disposition</code> header of the response</p>
    #[doc(hidden)]
    pub response_content_disposition: std::option::Option<std::string::String>,
    /// <p>Sets the <code>Content-Encoding</code> header of the response.</p>
    #[doc(hidden)]
    pub response_content_encoding: std::option::Option<std::string::String>,
    /// <p>Sets the <code>Content-Language</code> header of the response.</p>
    #[doc(hidden)]
    pub response_content_language: std::option::Option<std::string::String>,
    /// <p>Sets the <code>Content-Type</code> header of the response.</p>
    #[doc(hidden)]
    pub response_content_type: std::option::Option<std::string::String>,
    /// <p>Sets the <code>Expires</code> header of the response.</p>
    #[doc(hidden)]
    pub response_expires: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>VersionId used to reference a specific version of the object.</p>
    #[doc(hidden)]
    pub version_id: std::option::Option<std::string::String>,
    /// <p>Specifies the algorithm to use to when decrypting the object (for example, AES256).</p>
    #[doc(hidden)]
    pub sse_customer_algorithm: std::option::Option<std::string::String>,
    /// <p>Specifies the customer-provided encryption key for Amazon S3 used to encrypt the data. This value is used to decrypt the object when recovering it and must match the one used when storing the data. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    #[doc(hidden)]
    pub sse_customer_key: std::option::Option<std::string::String>,
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    #[doc(hidden)]
    pub sse_customer_key_md5: std::option::Option<std::string::String>,
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub request_payer: std::option::Option<crate::types::RequestPayer>,
    /// <p>Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' GET request for the part specified. Useful for downloading just a part of an object.</p>
    #[doc(hidden)]
    pub part_number: std::option::Option<i32>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    #[doc(hidden)]
    pub expected_bucket_owner: std::option::Option<std::string::String>,
    /// <p>To retrieve the checksum, this mode must be enabled.</p>
    #[doc(hidden)]
    pub checksum_mode: std::option::Option<crate::types::ChecksumMode>,
}
impl GetObjectInput {
    /// <p>The bucket name containing the object. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using an Object Lambda access point the hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-object-lambda.<i>Region</i>.amazonaws.com.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>Return the object only if its entity tag (ETag) is the same as the one specified; otherwise, return a 412 (precondition failed) error.</p>
    pub fn if_match(&self) -> std::option::Option<&str> {
        self.if_match.as_deref()
    }
    /// <p>Return the object only if it has been modified since the specified time; otherwise, return a 304 (not modified) error.</p>
    pub fn if_modified_since(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.if_modified_since.as_ref()
    }
    /// <p>Return the object only if its entity tag (ETag) is different from the one specified; otherwise, return a 304 (not modified) error.</p>
    pub fn if_none_match(&self) -> std::option::Option<&str> {
        self.if_none_match.as_deref()
    }
    /// <p>Return the object only if it has not been modified since the specified time; otherwise, return a 412 (precondition failed) error.</p>
    pub fn if_unmodified_since(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.if_unmodified_since.as_ref()
    }
    /// <p>Key of the object to get.</p>
    pub fn key(&self) -> std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>Downloads the specified range bytes of an object. For more information about the HTTP Range header, see <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35</a>.</p> <note>
    /// <p>Amazon S3 doesn't support retrieving multiple ranges of data per <code>GET</code> request.</p>
    /// </note>
    pub fn range(&self) -> std::option::Option<&str> {
        self.range.as_deref()
    }
    /// <p>Sets the <code>Cache-Control</code> header of the response.</p>
    pub fn response_cache_control(&self) -> std::option::Option<&str> {
        self.response_cache_control.as_deref()
    }
    /// <p>Sets the <code>Content-Disposition</code> header of the response</p>
    pub fn response_content_disposition(&self) -> std::option::Option<&str> {
        self.response_content_disposition.as_deref()
    }
    /// <p>Sets the <code>Content-Encoding</code> header of the response.</p>
    pub fn response_content_encoding(&self) -> std::option::Option<&str> {
        self.response_content_encoding.as_deref()
    }
    /// <p>Sets the <code>Content-Language</code> header of the response.</p>
    pub fn response_content_language(&self) -> std::option::Option<&str> {
        self.response_content_language.as_deref()
    }
    /// <p>Sets the <code>Content-Type</code> header of the response.</p>
    pub fn response_content_type(&self) -> std::option::Option<&str> {
        self.response_content_type.as_deref()
    }
    /// <p>Sets the <code>Expires</code> header of the response.</p>
    pub fn response_expires(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.response_expires.as_ref()
    }
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub fn version_id(&self) -> std::option::Option<&str> {
        self.version_id.as_deref()
    }
    /// <p>Specifies the algorithm to use to when decrypting the object (for example, AES256).</p>
    pub fn sse_customer_algorithm(&self) -> std::option::Option<&str> {
        self.sse_customer_algorithm.as_deref()
    }
    /// <p>Specifies the customer-provided encryption key for Amazon S3 used to encrypt the data. This value is used to decrypt the object when recovering it and must match the one used when storing the data. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    pub fn sse_customer_key(&self) -> std::option::Option<&str> {
        self.sse_customer_key.as_deref()
    }
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    pub fn sse_customer_key_md5(&self) -> std::option::Option<&str> {
        self.sse_customer_key_md5.as_deref()
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn request_payer(&self) -> std::option::Option<&crate::types::RequestPayer> {
        self.request_payer.as_ref()
    }
    /// <p>Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' GET request for the part specified. Useful for downloading just a part of an object.</p>
    pub fn part_number(&self) -> std::option::Option<i32> {
        self.part_number
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
    /// <p>To retrieve the checksum, this mode must be enabled.</p>
    pub fn checksum_mode(&self) -> std::option::Option<&crate::types::ChecksumMode> {
        self.checksum_mode.as_ref()
    }
}
impl std::fmt::Debug for GetObjectInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetObjectInput");
        formatter.field("bucket", &self.bucket);
        formatter.field("if_match", &self.if_match);
        formatter.field("if_modified_since", &self.if_modified_since);
        formatter.field("if_none_match", &self.if_none_match);
        formatter.field("if_unmodified_since", &self.if_unmodified_since);
        formatter.field("key", &self.key);
        formatter.field("range", &self.range);
        formatter.field("response_cache_control", &self.response_cache_control);
        formatter.field(
            "response_content_disposition",
            &self.response_content_disposition,
        );
        formatter.field("response_content_encoding", &self.response_content_encoding);
        formatter.field("response_content_language", &self.response_content_language);
        formatter.field("response_content_type", &self.response_content_type);
        formatter.field("response_expires", &self.response_expires);
        formatter.field("version_id", &self.version_id);
        formatter.field("sse_customer_algorithm", &self.sse_customer_algorithm);
        formatter.field("sse_customer_key", &"*** Sensitive Data Redacted ***");
        formatter.field("sse_customer_key_md5", &self.sse_customer_key_md5);
        formatter.field("request_payer", &self.request_payer);
        formatter.field("part_number", &self.part_number);
        formatter.field("expected_bucket_owner", &self.expected_bucket_owner);
        formatter.field("checksum_mode", &self.checksum_mode);
        formatter.finish()
    }
}
impl GetObjectInput {
    /// Creates a new builder-style object to manufacture [`GetObjectInput`](crate::operation::get_object::GetObjectInput).
    pub fn builder() -> crate::operation::get_object::builders::GetObjectInputBuilder {
        crate::operation::get_object::builders::GetObjectInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_object::GetObjectInput;
/// A builder for [`GetObjectInput`](crate::operation::get_object::GetObjectInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct GetObjectInputBuilder {
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) if_match: std::option::Option<std::string::String>,
    pub(crate) if_modified_since: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) if_none_match: std::option::Option<std::string::String>,
    pub(crate) if_unmodified_since: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) key: std::option::Option<std::string::String>,
    pub(crate) range: std::option::Option<std::string::String>,
    pub(crate) response_cache_control: std::option::Option<std::string::String>,
    pub(crate) response_content_disposition: std::option::Option<std::string::String>,
    pub(crate) response_content_encoding: std::option::Option<std::string::String>,
    pub(crate) response_content_language: std::option::Option<std::string::String>,
    pub(crate) response_content_type: std::option::Option<std::string::String>,
    pub(crate) response_expires: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) version_id: std::option::Option<std::string::String>,
    pub(crate) sse_customer_algorithm: std::option::Option<std::string::String>,
    pub(crate) sse_customer_key: std::option::Option<std::string::String>,
    pub(crate) sse_customer_key_md5: std::option::Option<std::string::String>,
    pub(crate) request_payer: std::option::Option<crate::types::RequestPayer>,
    pub(crate) part_number: std::option::Option<i32>,
    pub(crate) expected_bucket_owner: std::option::Option<std::string::String>,
    pub(crate) checksum_mode: std::option::Option<crate::types::ChecksumMode>,
}
impl GetObjectInputBuilder {
    /// <p>The bucket name containing the object. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using an Object Lambda access point the hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-object-lambda.<i>Region</i>.amazonaws.com.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>The bucket name containing the object. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using an Object Lambda access point the hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-object-lambda.<i>Region</i>.amazonaws.com.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>Return the object only if its entity tag (ETag) is the same as the one specified; otherwise, return a 412 (precondition failed) error.</p>
    pub fn if_match(mut self, input: impl Into<std::string::String>) -> Self {
        self.if_match = Some(input.into());
        self
    }
    /// <p>Return the object only if its entity tag (ETag) is the same as the one specified; otherwise, return a 412 (precondition failed) error.</p>
    pub fn set_if_match(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.if_match = input;
        self
    }
    /// <p>Return the object only if it has been modified since the specified time; otherwise, return a 304 (not modified) error.</p>
    pub fn if_modified_since(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.if_modified_since = Some(input);
        self
    }
    /// <p>Return the object only if it has been modified since the specified time; otherwise, return a 304 (not modified) error.</p>
    pub fn set_if_modified_since(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.if_modified_since = input;
        self
    }
    /// <p>Return the object only if its entity tag (ETag) is different from the one specified; otherwise, return a 304 (not modified) error.</p>
    pub fn if_none_match(mut self, input: impl Into<std::string::String>) -> Self {
        self.if_none_match = Some(input.into());
        self
    }
    /// <p>Return the object only if its entity tag (ETag) is different from the one specified; otherwise, return a 304 (not modified) error.</p>
    pub fn set_if_none_match(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.if_none_match = input;
        self
    }
    /// <p>Return the object only if it has not been modified since the specified time; otherwise, return a 412 (precondition failed) error.</p>
    pub fn if_unmodified_since(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.if_unmodified_since = Some(input);
        self
    }
    /// <p>Return the object only if it has not been modified since the specified time; otherwise, return a 412 (precondition failed) error.</p>
    pub fn set_if_unmodified_since(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.if_unmodified_since = input;
        self
    }
    /// <p>Key of the object to get.</p>
    pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
        self.key = Some(input.into());
        self
    }
    /// <p>Key of the object to get.</p>
    pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>Downloads the specified range bytes of an object. For more information about the HTTP Range header, see <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35</a>.</p> <note>
    /// <p>Amazon S3 doesn't support retrieving multiple ranges of data per <code>GET</code> request.</p>
    /// </note>
    pub fn range(mut self, input: impl Into<std::string::String>) -> Self {
        self.range = Some(input.into());
        self
    }
    /// <p>Downloads the specified range bytes of an object. For more information about the HTTP Range header, see <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35</a>.</p> <note>
    /// <p>Amazon S3 doesn't support retrieving multiple ranges of data per <code>GET</code> request.</p>
    /// </note>
    pub fn set_range(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.range = input;
        self
    }
    /// <p>Sets the <code>Cache-Control</code> header of the response.</p>
    pub fn response_cache_control(mut self, input: impl Into<std::string::String>) -> Self {
        self.response_cache_control = Some(input.into());
        self
    }
    /// <p>Sets the <code>Cache-Control</code> header of the response.</p>
    pub fn set_response_cache_control(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.response_cache_control = input;
        self
    }
    /// <p>Sets the <code>Content-Disposition</code> header of the response</p>
    pub fn response_content_disposition(mut self, input: impl Into<std::string::String>) -> Self {
        self.response_content_disposition = Some(input.into());
        self
    }
    /// <p>Sets the <code>Content-Disposition</code> header of the response</p>
    pub fn set_response_content_disposition(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.response_content_disposition = input;
        self
    }
    /// <p>Sets the <code>Content-Encoding</code> header of the response.</p>
    pub fn response_content_encoding(mut self, input: impl Into<std::string::String>) -> Self {
        self.response_content_encoding = Some(input.into());
        self
    }
    /// <p>Sets the <code>Content-Encoding</code> header of the response.</p>
    pub fn set_response_content_encoding(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.response_content_encoding = input;
        self
    }
    /// <p>Sets the <code>Content-Language</code> header of the response.</p>
    pub fn response_content_language(mut self, input: impl Into<std::string::String>) -> Self {
        self.response_content_language = Some(input.into());
        self
    }
    /// <p>Sets the <code>Content-Language</code> header of the response.</p>
    pub fn set_response_content_language(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.response_content_language = input;
        self
    }
    /// <p>Sets the <code>Content-Type</code> header of the response.</p>
    pub fn response_content_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.response_content_type = Some(input.into());
        self
    }
    /// <p>Sets the <code>Content-Type</code> header of the response.</p>
    pub fn set_response_content_type(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.response_content_type = input;
        self
    }
    /// <p>Sets the <code>Expires</code> header of the response.</p>
    pub fn response_expires(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.response_expires = Some(input);
        self
    }
    /// <p>Sets the <code>Expires</code> header of the response.</p>
    pub fn set_response_expires(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.response_expires = input;
        self
    }
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub fn version_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.version_id = Some(input.into());
        self
    }
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub fn set_version_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.version_id = input;
        self
    }
    /// <p>Specifies the algorithm to use to when decrypting the object (for example, AES256).</p>
    pub fn sse_customer_algorithm(mut self, input: impl Into<std::string::String>) -> Self {
        self.sse_customer_algorithm = Some(input.into());
        self
    }
    /// <p>Specifies the algorithm to use to when decrypting the object (for example, AES256).</p>
    pub fn set_sse_customer_algorithm(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.sse_customer_algorithm = input;
        self
    }
    /// <p>Specifies the customer-provided encryption key for Amazon S3 used to encrypt the data. This value is used to decrypt the object when recovering it and must match the one used when storing the data. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    pub fn sse_customer_key(mut self, input: impl Into<std::string::String>) -> Self {
        self.sse_customer_key = Some(input.into());
        self
    }
    /// <p>Specifies the customer-provided encryption key for Amazon S3 used to encrypt the data. This value is used to decrypt the object when recovering it and must match the one used when storing the data. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    pub fn set_sse_customer_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.sse_customer_key = input;
        self
    }
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    pub fn sse_customer_key_md5(mut self, input: impl Into<std::string::String>) -> Self {
        self.sse_customer_key_md5 = Some(input.into());
        self
    }
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    pub fn set_sse_customer_key_md5(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.sse_customer_key_md5 = input;
        self
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn request_payer(mut self, input: crate::types::RequestPayer) -> Self {
        self.request_payer = Some(input);
        self
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_request_payer(
        mut self,
        input: std::option::Option<crate::types::RequestPayer>,
    ) -> Self {
        self.request_payer = input;
        self
    }
    /// <p>Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' GET request for the part specified. Useful for downloading just a part of an object.</p>
    pub fn part_number(mut self, input: i32) -> Self {
        self.part_number = Some(input);
        self
    }
    /// <p>Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' GET request for the part specified. Useful for downloading just a part of an object.</p>
    pub fn set_part_number(mut self, input: std::option::Option<i32>) -> Self {
        self.part_number = input;
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl Into<std::string::String>) -> Self {
        self.expected_bucket_owner = Some(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.expected_bucket_owner = input;
        self
    }
    /// <p>To retrieve the checksum, this mode must be enabled.</p>
    pub fn checksum_mode(mut self, input: crate::types::ChecksumMode) -> Self {
        self.checksum_mode = Some(input);
        self
    }
    /// <p>To retrieve the checksum, this mode must be enabled.</p>
    pub fn set_checksum_mode(
        mut self,
        input: std::option::Option<crate::types::ChecksumMode>,
    ) -> Self {
        self.checksum_mode = input;
        self
    }
    /// Consumes the builder and constructs a [`GetObjectInput`](crate::operation::get_object::GetObjectInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_object::GetObjectInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::get_object::GetObjectInput {
            bucket: self.bucket,
            if_match: self.if_match,
            if_modified_since: self.if_modified_since,
            if_none_match: self.if_none_match,
            if_unmodified_since: self.if_unmodified_since,
            key: self.key,
            range: self.range,
            response_cache_control: self.response_cache_control,
            response_content_disposition: self.response_content_disposition,
            response_content_encoding: self.response_content_encoding,
            response_content_language: self.response_content_language,
            response_content_type: self.response_content_type,
            response_expires: self.response_expires,
            version_id: self.version_id,
            sse_customer_algorithm: self.sse_customer_algorithm,
            sse_customer_key: self.sse_customer_key,
            sse_customer_key_md5: self.sse_customer_key_md5,
            request_payer: self.request_payer,
            part_number: self.part_number,
            expected_bucket_owner: self.expected_bucket_owner,
            checksum_mode: self.checksum_mode,
        })
    }
}
impl std::fmt::Debug for GetObjectInputBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetObjectInputBuilder");
        formatter.field("bucket", &self.bucket);
        formatter.field("if_match", &self.if_match);
        formatter.field("if_modified_since", &self.if_modified_since);
        formatter.field("if_none_match", &self.if_none_match);
        formatter.field("if_unmodified_since", &self.if_unmodified_since);
        formatter.field("key", &self.key);
        formatter.field("range", &self.range);
        formatter.field("response_cache_control", &self.response_cache_control);
        formatter.field(
            "response_content_disposition",
            &self.response_content_disposition,
        );
        formatter.field("response_content_encoding", &self.response_content_encoding);
        formatter.field("response_content_language", &self.response_content_language);
        formatter.field("response_content_type", &self.response_content_type);
        formatter.field("response_expires", &self.response_expires);
        formatter.field("version_id", &self.version_id);
        formatter.field("sse_customer_algorithm", &self.sse_customer_algorithm);
        formatter.field("sse_customer_key", &"*** Sensitive Data Redacted ***");
        formatter.field("sse_customer_key_md5", &self.sse_customer_key_md5);
        formatter.field("request_payer", &self.request_payer);
        formatter.field("part_number", &self.part_number);
        formatter.field("expected_bucket_owner", &self.expected_bucket_owner);
        formatter.field("checksum_mode", &self.checksum_mode);
        formatter.finish()
    }
}