// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_object::_get_object_output::GetObjectOutputBuilder;

pub use crate::operation::get_object::_get_object_input::GetObjectInputBuilder;

/// Fluent builder constructing a request to `GetObject`.
///
/// <p>Retrieves objects from Amazon S3. To use <code>GET</code>, you must have <code>READ</code> access to the object. If you grant <code>READ</code> access to the anonymous user, you can return the object without using an authorization header.</p>
/// <p>An Amazon S3 bucket has no directory hierarchy such as you would find in a typical computer file system. You can, however, create a logical hierarchy by using object key names that imply a folder structure. For example, instead of naming an object <code>sample.jpg</code>, you can name it <code>photos/2006/February/sample.jpg</code>.</p>
/// <p>To get an object from such a logical hierarchy, specify the full key name for the object in the <code>GET</code> operation. For a virtual hosted-style request example, if you have the object <code>photos/2006/February/sample.jpg</code>, specify the resource as <code>/photos/2006/February/sample.jpg</code>. For a path-style request example, if you have the object <code>photos/2006/February/sample.jpg</code> in the bucket named <code>examplebucket</code>, specify the resource as <code>/examplebucket/photos/2006/February/sample.jpg</code>. For more information about request types, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/VirtualHosting.html#VirtualHostingSpecifyBucket">HTTP Host Header Bucket Specification</a>.</p>
/// <p>For more information about returning the ACL of an object, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAcl.html">GetObjectAcl</a>.</p>
/// <p>If the object you are retrieving is stored in the S3 Glacier or S3 Glacier Deep Archive storage class, or S3 Intelligent-Tiering Archive or S3 Intelligent-Tiering Deep Archive tiers, before you can retrieve the object you must first restore a copy using <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_RestoreObject.html">RestoreObject</a>. Otherwise, this action returns an <code>InvalidObjectStateError</code> error. For information about restoring archived objects, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/restoring-objects.html">Restoring Archived Objects</a>.</p>
/// <p>Encryption request headers, like <code>x-amz-server-side-encryption</code>, should not be sent for GET requests if your object uses server-side encryption with KMS keys (SSE-KMS) or server-side encryption with Amazon S3–managed encryption keys (SSE-S3). If your object does use these types of keys, you’ll get an HTTP 400 BadRequest error.</p>
/// <p>If you encrypt an object by using server-side encryption with customer-provided encryption keys (SSE-C) when you store the object in Amazon S3, then when you GET the object, you must use the following headers:</p>
/// <ul>
/// <li> <p>x-amz-server-side-encryption-customer-algorithm</p> </li>
/// <li> <p>x-amz-server-side-encryption-customer-key</p> </li>
/// <li> <p>x-amz-server-side-encryption-customer-key-MD5</p> </li>
/// </ul>
/// <p>For more information about SSE-C, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Server-Side Encryption (Using Customer-Provided Encryption Keys)</a>.</p>
/// <p>Assuming you have the relevant permission to read object tags, the response also returns the <code>x-amz-tagging-count</code> header that provides the count of number of tags associated with the object. You can use <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectTagging.html">GetObjectTagging</a> to retrieve the tag set associated with an object.</p>
/// <p> <b>Permissions</b> </p>
/// <p>You need the relevant read object (or version) permission for this operation. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html">Specifying Permissions in a Policy</a>. If the object you request does not exist, the error Amazon S3 returns depends on whether you also have the <code>s3:ListBucket</code> permission.</p>
/// <ul>
/// <li> <p>If you have the <code>s3:ListBucket</code> permission on the bucket, Amazon S3 will return an HTTP status code 404 ("no such key") error.</p> </li>
/// <li> <p>If you don’t have the <code>s3:ListBucket</code> permission, Amazon S3 will return an HTTP status code 403 ("access denied") error.</p> </li>
/// </ul>
/// <p> <b>Versioning</b> </p>
/// <p>By default, the GET action returns the current version of an object. To return a different version, use the <code>versionId</code> subresource.</p> <note>
/// <ul>
/// <li> <p> If you supply a <code>versionId</code>, you need the <code>s3:GetObjectVersion</code> permission to access a specific version of an object. If you request a specific version, you do not need to have the <code>s3:GetObject</code> permission. </p> </li>
/// <li> <p>If the current version of the object is a delete marker, Amazon S3 behaves as if the object was deleted and includes <code>x-amz-delete-marker: true</code> in the response.</p> </li>
/// </ul>
/// </note>
/// <p>For more information about versioning, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketVersioning.html">PutBucketVersioning</a>. </p>
/// <p> <b>Overriding Response Header Values</b> </p>
/// <p>There are times when you want to override certain response header values in a GET response. For example, you might override the <code>Content-Disposition</code> response header value in your GET request.</p>
/// <p>You can override values for a set of response headers using the following query parameters. These response header values are sent only on a successful request, that is, when status code 200 OK is returned. The set of headers you can override using these parameters is a subset of the headers that Amazon S3 accepts when you create an object. The response headers that you can override for the GET response are <code>Content-Type</code>, <code>Content-Language</code>, <code>Expires</code>, <code>Cache-Control</code>, <code>Content-Disposition</code>, and <code>Content-Encoding</code>. To override these header values in the GET response, you use the following request parameters.</p> <note>
/// <p>You must sign the request, either using an Authorization header or a presigned URL, when using these parameters. They cannot be used with an unsigned (anonymous) request.</p>
/// </note>
/// <ul>
/// <li> <p> <code>response-content-type</code> </p> </li>
/// <li> <p> <code>response-content-language</code> </p> </li>
/// <li> <p> <code>response-expires</code> </p> </li>
/// <li> <p> <code>response-cache-control</code> </p> </li>
/// <li> <p> <code>response-content-disposition</code> </p> </li>
/// <li> <p> <code>response-content-encoding</code> </p> </li>
/// </ul>
/// <p> <b>Additional Considerations about Request Headers</b> </p>
/// <p>If both of the <code>If-Match</code> and <code>If-Unmodified-Since</code> headers are present in the request as follows: <code>If-Match</code> condition evaluates to <code>true</code>, and; <code>If-Unmodified-Since</code> condition evaluates to <code>false</code>; then, S3 returns 200 OK and the data requested. </p>
/// <p>If both of the <code>If-None-Match</code> and <code>If-Modified-Since</code> headers are present in the request as follows:<code> If-None-Match</code> condition evaluates to <code>false</code>, and; <code>If-Modified-Since</code> condition evaluates to <code>true</code>; then, S3 returns 304 Not Modified response code.</p>
/// <p>For more information about conditional requests, see <a href="https://tools.ietf.org/html/rfc7232">RFC 7232</a>.</p>
/// <p>The following operations are related to <code>GetObject</code>:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBuckets.html">ListBuckets</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAcl.html">GetObjectAcl</a> </p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetObjectFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_object::builders::GetObjectInputBuilder,
}
impl GetObjectFluentBuilder {
    /// Creates a new `GetObject`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::get_object::GetObject,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::get_object::GetObjectError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::get_object::GetObjectOutput,
        aws_smithy_http::result::SdkError<crate::operation::get_object::GetObjectError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::get_object::builders::GetObjectInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_object().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_object::builders::GetObjectInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    ///
    /// Creates a presigned request for this operation.
    ///
    /// The `presigning_config` provides additional presigning-specific config values, such as the
    /// amount of time the request should be valid for after creation.
    ///
    /// Presigned requests can be given to other users or applications to access a resource or perform
    /// an operation without having access to the AWS security credentials.
    ///
    pub async fn presigned(
        self,
        presigning_config: crate::presigning::PresigningConfig,
    ) -> Result<
        crate::presigning::PresignedRequest,
        aws_smithy_http::result::SdkError<crate::operation::get_object::GetObjectError>,
    > {
        let input = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        input.presigned(&self.handle.conf, presigning_config).await
    }
    /// <p>The bucket name containing the object. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using an Object Lambda access point the hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-object-lambda.<i>Region</i>.amazonaws.com.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The bucket name containing the object. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using an Object Lambda access point the hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-object-lambda.<i>Region</i>.amazonaws.com.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>Return the object only if its entity tag (ETag) is the same as the one specified; otherwise, return a 412 (precondition failed) error.</p>
    pub fn if_match(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.if_match(input.into());
        self
    }
    /// <p>Return the object only if its entity tag (ETag) is the same as the one specified; otherwise, return a 412 (precondition failed) error.</p>
    pub fn set_if_match(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_if_match(input);
        self
    }
    /// <p>Return the object only if it has been modified since the specified time; otherwise, return a 304 (not modified) error.</p>
    pub fn if_modified_since(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.if_modified_since(input);
        self
    }
    /// <p>Return the object only if it has been modified since the specified time; otherwise, return a 304 (not modified) error.</p>
    pub fn set_if_modified_since(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_if_modified_since(input);
        self
    }
    /// <p>Return the object only if its entity tag (ETag) is different from the one specified; otherwise, return a 304 (not modified) error.</p>
    pub fn if_none_match(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.if_none_match(input.into());
        self
    }
    /// <p>Return the object only if its entity tag (ETag) is different from the one specified; otherwise, return a 304 (not modified) error.</p>
    pub fn set_if_none_match(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_if_none_match(input);
        self
    }
    /// <p>Return the object only if it has not been modified since the specified time; otherwise, return a 412 (precondition failed) error.</p>
    pub fn if_unmodified_since(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.if_unmodified_since(input);
        self
    }
    /// <p>Return the object only if it has not been modified since the specified time; otherwise, return a 412 (precondition failed) error.</p>
    pub fn set_if_unmodified_since(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_if_unmodified_since(input);
        self
    }
    /// <p>Key of the object to get.</p>
    pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.key(input.into());
        self
    }
    /// <p>Key of the object to get.</p>
    pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_key(input);
        self
    }
    /// <p>Downloads the specified range bytes of an object. For more information about the HTTP Range header, see <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35</a>.</p> <note>
    /// <p>Amazon S3 doesn't support retrieving multiple ranges of data per <code>GET</code> request.</p>
    /// </note>
    pub fn range(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.range(input.into());
        self
    }
    /// <p>Downloads the specified range bytes of an object. For more information about the HTTP Range header, see <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35</a>.</p> <note>
    /// <p>Amazon S3 doesn't support retrieving multiple ranges of data per <code>GET</code> request.</p>
    /// </note>
    pub fn set_range(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_range(input);
        self
    }
    /// <p>Sets the <code>Cache-Control</code> header of the response.</p>
    pub fn response_cache_control(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.response_cache_control(input.into());
        self
    }
    /// <p>Sets the <code>Cache-Control</code> header of the response.</p>
    pub fn set_response_cache_control(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_response_cache_control(input);
        self
    }
    /// <p>Sets the <code>Content-Disposition</code> header of the response</p>
    pub fn response_content_disposition(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.response_content_disposition(input.into());
        self
    }
    /// <p>Sets the <code>Content-Disposition</code> header of the response</p>
    pub fn set_response_content_disposition(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_response_content_disposition(input);
        self
    }
    /// <p>Sets the <code>Content-Encoding</code> header of the response.</p>
    pub fn response_content_encoding(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.response_content_encoding(input.into());
        self
    }
    /// <p>Sets the <code>Content-Encoding</code> header of the response.</p>
    pub fn set_response_content_encoding(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_response_content_encoding(input);
        self
    }
    /// <p>Sets the <code>Content-Language</code> header of the response.</p>
    pub fn response_content_language(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.response_content_language(input.into());
        self
    }
    /// <p>Sets the <code>Content-Language</code> header of the response.</p>
    pub fn set_response_content_language(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_response_content_language(input);
        self
    }
    /// <p>Sets the <code>Content-Type</code> header of the response.</p>
    pub fn response_content_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.response_content_type(input.into());
        self
    }
    /// <p>Sets the <code>Content-Type</code> header of the response.</p>
    pub fn set_response_content_type(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_response_content_type(input);
        self
    }
    /// <p>Sets the <code>Expires</code> header of the response.</p>
    pub fn response_expires(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.response_expires(input);
        self
    }
    /// <p>Sets the <code>Expires</code> header of the response.</p>
    pub fn set_response_expires(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_response_expires(input);
        self
    }
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub fn version_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.version_id(input.into());
        self
    }
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub fn set_version_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_version_id(input);
        self
    }
    /// <p>Specifies the algorithm to use to when decrypting the object (for example, AES256).</p>
    pub fn sse_customer_algorithm(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_algorithm(input.into());
        self
    }
    /// <p>Specifies the algorithm to use to when decrypting the object (for example, AES256).</p>
    pub fn set_sse_customer_algorithm(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_sse_customer_algorithm(input);
        self
    }
    /// <p>Specifies the customer-provided encryption key for Amazon S3 used to encrypt the data. This value is used to decrypt the object when recovering it and must match the one used when storing the data. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    pub fn sse_customer_key(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_key(input.into());
        self
    }
    /// <p>Specifies the customer-provided encryption key for Amazon S3 used to encrypt the data. This value is used to decrypt the object when recovering it and must match the one used when storing the data. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    pub fn set_sse_customer_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_sse_customer_key(input);
        self
    }
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    pub fn sse_customer_key_md5(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_key_md5(input.into());
        self
    }
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    pub fn set_sse_customer_key_md5(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_sse_customer_key_md5(input);
        self
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn request_payer(mut self, input: crate::types::RequestPayer) -> Self {
        self.inner = self.inner.request_payer(input);
        self
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_request_payer(
        mut self,
        input: std::option::Option<crate::types::RequestPayer>,
    ) -> Self {
        self.inner = self.inner.set_request_payer(input);
        self
    }
    /// <p>Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' GET request for the part specified. Useful for downloading just a part of an object.</p>
    pub fn part_number(mut self, input: i32) -> Self {
        self.inner = self.inner.part_number(input);
        self
    }
    /// <p>Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' GET request for the part specified. Useful for downloading just a part of an object.</p>
    pub fn set_part_number(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_part_number(input);
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.expected_bucket_owner(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_expected_bucket_owner(input);
        self
    }
    /// <p>To retrieve the checksum, this mode must be enabled.</p>
    pub fn checksum_mode(mut self, input: crate::types::ChecksumMode) -> Self {
        self.inner = self.inner.checksum_mode(input);
        self
    }
    /// <p>To retrieve the checksum, this mode must be enabled.</p>
    pub fn set_checksum_mode(
        mut self,
        input: std::option::Option<crate::types::ChecksumMode>,
    ) -> Self {
        self.inner = self.inner.set_checksum_mode(input);
        self
    }
}