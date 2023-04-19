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
pub struct CreateMultipartUploadOutput {
    /// <p>If the bucket has a lifecycle rule configured with an action to abort incomplete multipart uploads and the prefix in the lifecycle rule matches the object name in the request, the response includes this header. The header indicates when the initiated multipart upload becomes eligible for an abort operation. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Policy</a>.</p>
    /// <p>The response also includes the <code>x-amz-abort-rule-id</code> header that provides the ID of the lifecycle configuration rule that defines this action.</p>
    #[doc(hidden)]
    pub abort_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>This header is returned along with the <code>x-amz-abort-date</code> header. It identifies the applicable lifecycle configuration rule that defines the action to abort incomplete multipart uploads.</p>
    #[doc(hidden)]
    pub abort_rule_id: std::option::Option<std::string::String>,
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>Object key for which the multipart upload was initiated.</p>
    #[doc(hidden)]
    pub key: std::option::Option<std::string::String>,
    /// <p>ID for the initiated multipart upload.</p>
    #[doc(hidden)]
    pub upload_id: std::option::Option<std::string::String>,
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms).</p>
    #[doc(hidden)]
    pub server_side_encryption: std::option::Option<crate::types::ServerSideEncryption>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    #[doc(hidden)]
    pub sse_customer_algorithm: std::option::Option<std::string::String>,
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round-trip message integrity verification of the customer-provided encryption key.</p>
    #[doc(hidden)]
    pub sse_customer_key_md5: std::option::Option<std::string::String>,
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key that was used for the object.</p>
    #[doc(hidden)]
    pub ssekms_key_id: std::option::Option<std::string::String>,
    /// <p>If present, specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p>
    #[doc(hidden)]
    pub ssekms_encryption_context: std::option::Option<std::string::String>,
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    #[doc(hidden)]
    pub bucket_key_enabled: bool,
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    #[doc(hidden)]
    pub request_charged: std::option::Option<crate::types::RequestCharged>,
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    #[doc(hidden)]
    pub checksum_algorithm: std::option::Option<crate::types::ChecksumAlgorithm>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl CreateMultipartUploadOutput {
    /// <p>If the bucket has a lifecycle rule configured with an action to abort incomplete multipart uploads and the prefix in the lifecycle rule matches the object name in the request, the response includes this header. The header indicates when the initiated multipart upload becomes eligible for an abort operation. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Policy</a>.</p>
    /// <p>The response also includes the <code>x-amz-abort-rule-id</code> header that provides the ID of the lifecycle configuration rule that defines this action.</p>
    pub fn abort_date(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.abort_date.as_ref()
    }
    /// <p>This header is returned along with the <code>x-amz-abort-date</code> header. It identifies the applicable lifecycle configuration rule that defines the action to abort incomplete multipart uploads.</p>
    pub fn abort_rule_id(&self) -> std::option::Option<&str> {
        self.abort_rule_id.as_deref()
    }
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn key(&self) -> std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>ID for the initiated multipart upload.</p>
    pub fn upload_id(&self) -> std::option::Option<&str> {
        self.upload_id.as_deref()
    }
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms).</p>
    pub fn server_side_encryption(
        &self,
    ) -> std::option::Option<&crate::types::ServerSideEncryption> {
        self.server_side_encryption.as_ref()
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub fn sse_customer_algorithm(&self) -> std::option::Option<&str> {
        self.sse_customer_algorithm.as_deref()
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round-trip message integrity verification of the customer-provided encryption key.</p>
    pub fn sse_customer_key_md5(&self) -> std::option::Option<&str> {
        self.sse_customer_key_md5.as_deref()
    }
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key that was used for the object.</p>
    pub fn ssekms_key_id(&self) -> std::option::Option<&str> {
        self.ssekms_key_id.as_deref()
    }
    /// <p>If present, specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p>
    pub fn ssekms_encryption_context(&self) -> std::option::Option<&str> {
        self.ssekms_encryption_context.as_deref()
    }
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub fn bucket_key_enabled(&self) -> bool {
        self.bucket_key_enabled
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn request_charged(&self) -> std::option::Option<&crate::types::RequestCharged> {
        self.request_charged.as_ref()
    }
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn checksum_algorithm(&self) -> std::option::Option<&crate::types::ChecksumAlgorithm> {
        self.checksum_algorithm.as_ref()
    }
}
impl std::fmt::Debug for CreateMultipartUploadOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateMultipartUploadOutput");
        formatter.field("abort_date", &self.abort_date);
        formatter.field("abort_rule_id", &self.abort_rule_id);
        formatter.field("bucket", &self.bucket);
        formatter.field("key", &self.key);
        formatter.field("upload_id", &self.upload_id);
        formatter.field("server_side_encryption", &self.server_side_encryption);
        formatter.field("sse_customer_algorithm", &self.sse_customer_algorithm);
        formatter.field("sse_customer_key_md5", &self.sse_customer_key_md5);
        formatter.field("ssekms_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field(
            "ssekms_encryption_context",
            &"*** Sensitive Data Redacted ***",
        );
        formatter.field("bucket_key_enabled", &self.bucket_key_enabled);
        formatter.field("request_charged", &self.request_charged);
        formatter.field("checksum_algorithm", &self.checksum_algorithm);
        formatter.field("_extended_request_id", &self._extended_request_id);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl crate::s3_request_id::RequestIdExt for CreateMultipartUploadOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for CreateMultipartUploadOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateMultipartUploadOutput {
    /// Creates a new builder-style object to manufacture [`CreateMultipartUploadOutput`](crate::operation::create_multipart_upload::CreateMultipartUploadOutput).
    pub fn builder(
    ) -> crate::operation::create_multipart_upload::builders::CreateMultipartUploadOutputBuilder
    {
        crate::operation::create_multipart_upload::builders::CreateMultipartUploadOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_multipart_upload::CreateMultipartUploadOutput;
/// A builder for [`CreateMultipartUploadOutput`](crate::operation::create_multipart_upload::CreateMultipartUploadOutput).
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
pub struct CreateMultipartUploadOutputBuilder {
    pub(crate) abort_date: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) abort_rule_id: std::option::Option<std::string::String>,
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) key: std::option::Option<std::string::String>,
    pub(crate) upload_id: std::option::Option<std::string::String>,
    pub(crate) server_side_encryption: std::option::Option<crate::types::ServerSideEncryption>,
    pub(crate) sse_customer_algorithm: std::option::Option<std::string::String>,
    pub(crate) sse_customer_key_md5: std::option::Option<std::string::String>,
    pub(crate) ssekms_key_id: std::option::Option<std::string::String>,
    pub(crate) ssekms_encryption_context: std::option::Option<std::string::String>,
    pub(crate) bucket_key_enabled: std::option::Option<bool>,
    pub(crate) request_charged: std::option::Option<crate::types::RequestCharged>,
    pub(crate) checksum_algorithm: std::option::Option<crate::types::ChecksumAlgorithm>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl CreateMultipartUploadOutputBuilder {
    /// <p>If the bucket has a lifecycle rule configured with an action to abort incomplete multipart uploads and the prefix in the lifecycle rule matches the object name in the request, the response includes this header. The header indicates when the initiated multipart upload becomes eligible for an abort operation. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Policy</a>.</p>
    /// <p>The response also includes the <code>x-amz-abort-rule-id</code> header that provides the ID of the lifecycle configuration rule that defines this action.</p>
    pub fn abort_date(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.abort_date = Some(input);
        self
    }
    /// <p>If the bucket has a lifecycle rule configured with an action to abort incomplete multipart uploads and the prefix in the lifecycle rule matches the object name in the request, the response includes this header. The header indicates when the initiated multipart upload becomes eligible for an abort operation. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html#mpu-abort-incomplete-mpu-lifecycle-config"> Aborting Incomplete Multipart Uploads Using a Bucket Lifecycle Policy</a>.</p>
    /// <p>The response also includes the <code>x-amz-abort-rule-id</code> header that provides the ID of the lifecycle configuration rule that defines this action.</p>
    pub fn set_abort_date(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.abort_date = input;
        self
    }
    /// <p>This header is returned along with the <code>x-amz-abort-date</code> header. It identifies the applicable lifecycle configuration rule that defines the action to abort incomplete multipart uploads.</p>
    pub fn abort_rule_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.abort_rule_id = Some(input.into());
        self
    }
    /// <p>This header is returned along with the <code>x-amz-abort-date</code> header. It identifies the applicable lifecycle configuration rule that defines the action to abort incomplete multipart uploads.</p>
    pub fn set_abort_rule_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.abort_rule_id = input;
        self
    }
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
        self.key = Some(input.into());
        self
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>ID for the initiated multipart upload.</p>
    pub fn upload_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.upload_id = Some(input.into());
        self
    }
    /// <p>ID for the initiated multipart upload.</p>
    pub fn set_upload_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.upload_id = input;
        self
    }
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms).</p>
    pub fn server_side_encryption(mut self, input: crate::types::ServerSideEncryption) -> Self {
        self.server_side_encryption = Some(input);
        self
    }
    /// <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms).</p>
    pub fn set_server_side_encryption(
        mut self,
        input: std::option::Option<crate::types::ServerSideEncryption>,
    ) -> Self {
        self.server_side_encryption = input;
        self
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub fn sse_customer_algorithm(mut self, input: impl Into<std::string::String>) -> Self {
        self.sse_customer_algorithm = Some(input.into());
        self
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    pub fn set_sse_customer_algorithm(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.sse_customer_algorithm = input;
        self
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round-trip message integrity verification of the customer-provided encryption key.</p>
    pub fn sse_customer_key_md5(mut self, input: impl Into<std::string::String>) -> Self {
        self.sse_customer_key_md5 = Some(input.into());
        self
    }
    /// <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round-trip message integrity verification of the customer-provided encryption key.</p>
    pub fn set_sse_customer_key_md5(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.sse_customer_key_md5 = input;
        self
    }
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key that was used for the object.</p>
    pub fn ssekms_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.ssekms_key_id = Some(input.into());
        self
    }
    /// <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key that was used for the object.</p>
    pub fn set_ssekms_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ssekms_key_id = input;
        self
    }
    /// <p>If present, specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p>
    pub fn ssekms_encryption_context(mut self, input: impl Into<std::string::String>) -> Self {
        self.ssekms_encryption_context = Some(input.into());
        self
    }
    /// <p>If present, specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p>
    pub fn set_ssekms_encryption_context(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.ssekms_encryption_context = input;
        self
    }
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub fn bucket_key_enabled(mut self, input: bool) -> Self {
        self.bucket_key_enabled = Some(input);
        self
    }
    /// <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    pub fn set_bucket_key_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.bucket_key_enabled = input;
        self
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn request_charged(mut self, input: crate::types::RequestCharged) -> Self {
        self.request_charged = Some(input);
        self
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn set_request_charged(
        mut self,
        input: std::option::Option<crate::types::RequestCharged>,
    ) -> Self {
        self.request_charged = input;
        self
    }
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn checksum_algorithm(mut self, input: crate::types::ChecksumAlgorithm) -> Self {
        self.checksum_algorithm = Some(input);
        self
    }
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn set_checksum_algorithm(
        mut self,
        input: std::option::Option<crate::types::ChecksumAlgorithm>,
    ) -> Self {
        self.checksum_algorithm = input;
        self
    }
    pub(crate) fn _extended_request_id(mut self, extended_request_id: impl Into<String>) -> Self {
        self._extended_request_id = Some(extended_request_id.into());
        self
    }

    pub(crate) fn _set_extended_request_id(
        &mut self,
        extended_request_id: Option<String>,
    ) -> &mut Self {
        self._extended_request_id = extended_request_id;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateMultipartUploadOutput`](crate::operation::create_multipart_upload::CreateMultipartUploadOutput).
    pub fn build(self) -> crate::operation::create_multipart_upload::CreateMultipartUploadOutput {
        crate::operation::create_multipart_upload::CreateMultipartUploadOutput {
            abort_date: self.abort_date,
            abort_rule_id: self.abort_rule_id,
            bucket: self.bucket,
            key: self.key,
            upload_id: self.upload_id,
            server_side_encryption: self.server_side_encryption,
            sse_customer_algorithm: self.sse_customer_algorithm,
            sse_customer_key_md5: self.sse_customer_key_md5,
            ssekms_key_id: self.ssekms_key_id,
            ssekms_encryption_context: self.ssekms_encryption_context,
            bucket_key_enabled: self.bucket_key_enabled.unwrap_or_default(),
            request_charged: self.request_charged,
            checksum_algorithm: self.checksum_algorithm,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
impl std::fmt::Debug for CreateMultipartUploadOutputBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateMultipartUploadOutputBuilder");
        formatter.field("abort_date", &self.abort_date);
        formatter.field("abort_rule_id", &self.abort_rule_id);
        formatter.field("bucket", &self.bucket);
        formatter.field("key", &self.key);
        formatter.field("upload_id", &self.upload_id);
        formatter.field("server_side_encryption", &self.server_side_encryption);
        formatter.field("sse_customer_algorithm", &self.sse_customer_algorithm);
        formatter.field("sse_customer_key_md5", &self.sse_customer_key_md5);
        formatter.field("ssekms_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field(
            "ssekms_encryption_context",
            &"*** Sensitive Data Redacted ***",
        );
        formatter.field("bucket_key_enabled", &self.bucket_key_enabled);
        formatter.field("request_charged", &self.request_charged);
        formatter.field("checksum_algorithm", &self.checksum_algorithm);
        formatter.field("_extended_request_id", &self._extended_request_id);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}