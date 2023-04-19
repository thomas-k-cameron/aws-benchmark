// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Allows you to specify additional settings for your streaming Call Analytics post-call request, including output locations for your redacted and unredacted transcript, which IAM role to use, and, optionally, which encryption key to use.</p>
/// <p> <code>ContentRedactionOutput</code>, <code>DataAccessRoleArn</code>, and <code>OutputLocation</code> are required fields.</p>
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
pub struct PostCallAnalyticsSettings {
    /// <p>The Amazon S3 location where you want your Call Analytics post-call transcription output stored. You can use any of the following formats to specify the output location:</p>
    /// <ol>
    /// <li> <p>s3://DOC-EXAMPLE-BUCKET</p> </li>
    /// <li> <p>s3://DOC-EXAMPLE-BUCKET/my-output-folder/</p> </li>
    /// <li> <p>s3://DOC-EXAMPLE-BUCKET/my-output-folder/my-call-analytics-job.json</p> </li>
    /// </ol>
    #[doc(hidden)]
    pub output_location: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of an IAM role that has permissions to access the Amazon S3 bucket that contains your input files. If the role that you specify doesn’t have the appropriate permissions to access the specified Amazon S3 location, your request fails.</p>
    /// <p>IAM role ARNs have the format <code>arn:partition:iam::account:role/role-name-with-path</code>. For example: <code>arn:aws:iam::111122223333:role/Admin</code>. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a>.</p>
    #[doc(hidden)]
    pub data_access_role_arn: std::option::Option<std::string::String>,
    /// <p>Specify whether you want only a redacted transcript or both a redacted and an unredacted transcript. If you choose redacted and unredacted, two JSON files are generated and stored in the Amazon S3 output location you specify.</p>
    /// <p>Note that to include <code>ContentRedactionOutput</code> in your request, you must enable content redaction (<code>ContentRedactionType</code>).</p>
    #[doc(hidden)]
    pub content_redaction_output: std::option::Option<crate::types::ContentRedactionOutput>,
    /// <p>The KMS key you want to use to encrypt your Call Analytics post-call output.</p>
    /// <p>If using a key located in the <b>current</b> Amazon Web Services account, you can specify your KMS key in one of four ways:</p>
    /// <ol>
    /// <li> <p>Use the KMS key ID itself. For example, <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li>
    /// <li> <p>Use an alias for the KMS key ID. For example, <code>alias/ExampleAlias</code>.</p> </li>
    /// <li> <p>Use the Amazon Resource Name (ARN) for the KMS key ID. For example, <code>arn:aws:kms:region:account-ID:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li>
    /// <li> <p>Use the ARN for the KMS key alias. For example, <code>arn:aws:kms:region:account-ID:alias/ExampleAlias</code>.</p> </li>
    /// </ol>
    /// <p>If using a key located in a <b>different</b> Amazon Web Services account than the current Amazon Web Services account, you can specify your KMS key in one of two ways:</p>
    /// <ol>
    /// <li> <p>Use the ARN for the KMS key ID. For example, <code>arn:aws:kms:region:account-ID:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li>
    /// <li> <p>Use the ARN for the KMS key alias. For example, <code>arn:aws:kms:region:account-ID:alias/ExampleAlias</code>.</p> </li>
    /// </ol>
    /// <p>Note that the user making the request must have permission to use the specified KMS key.</p>
    #[doc(hidden)]
    pub output_encryption_kms_key_id: std::option::Option<std::string::String>,
}
impl PostCallAnalyticsSettings {
    /// <p>The Amazon S3 location where you want your Call Analytics post-call transcription output stored. You can use any of the following formats to specify the output location:</p>
    /// <ol>
    /// <li> <p>s3://DOC-EXAMPLE-BUCKET</p> </li>
    /// <li> <p>s3://DOC-EXAMPLE-BUCKET/my-output-folder/</p> </li>
    /// <li> <p>s3://DOC-EXAMPLE-BUCKET/my-output-folder/my-call-analytics-job.json</p> </li>
    /// </ol>
    pub fn output_location(&self) -> std::option::Option<&str> {
        self.output_location.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role that has permissions to access the Amazon S3 bucket that contains your input files. If the role that you specify doesn’t have the appropriate permissions to access the specified Amazon S3 location, your request fails.</p>
    /// <p>IAM role ARNs have the format <code>arn:partition:iam::account:role/role-name-with-path</code>. For example: <code>arn:aws:iam::111122223333:role/Admin</code>. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a>.</p>
    pub fn data_access_role_arn(&self) -> std::option::Option<&str> {
        self.data_access_role_arn.as_deref()
    }
    /// <p>Specify whether you want only a redacted transcript or both a redacted and an unredacted transcript. If you choose redacted and unredacted, two JSON files are generated and stored in the Amazon S3 output location you specify.</p>
    /// <p>Note that to include <code>ContentRedactionOutput</code> in your request, you must enable content redaction (<code>ContentRedactionType</code>).</p>
    pub fn content_redaction_output(
        &self,
    ) -> std::option::Option<&crate::types::ContentRedactionOutput> {
        self.content_redaction_output.as_ref()
    }
    /// <p>The KMS key you want to use to encrypt your Call Analytics post-call output.</p>
    /// <p>If using a key located in the <b>current</b> Amazon Web Services account, you can specify your KMS key in one of four ways:</p>
    /// <ol>
    /// <li> <p>Use the KMS key ID itself. For example, <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li>
    /// <li> <p>Use an alias for the KMS key ID. For example, <code>alias/ExampleAlias</code>.</p> </li>
    /// <li> <p>Use the Amazon Resource Name (ARN) for the KMS key ID. For example, <code>arn:aws:kms:region:account-ID:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li>
    /// <li> <p>Use the ARN for the KMS key alias. For example, <code>arn:aws:kms:region:account-ID:alias/ExampleAlias</code>.</p> </li>
    /// </ol>
    /// <p>If using a key located in a <b>different</b> Amazon Web Services account than the current Amazon Web Services account, you can specify your KMS key in one of two ways:</p>
    /// <ol>
    /// <li> <p>Use the ARN for the KMS key ID. For example, <code>arn:aws:kms:region:account-ID:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li>
    /// <li> <p>Use the ARN for the KMS key alias. For example, <code>arn:aws:kms:region:account-ID:alias/ExampleAlias</code>.</p> </li>
    /// </ol>
    /// <p>Note that the user making the request must have permission to use the specified KMS key.</p>
    pub fn output_encryption_kms_key_id(&self) -> std::option::Option<&str> {
        self.output_encryption_kms_key_id.as_deref()
    }
}
impl PostCallAnalyticsSettings {
    /// Creates a new builder-style object to manufacture [`PostCallAnalyticsSettings`](crate::types::PostCallAnalyticsSettings).
    pub fn builder() -> crate::types::builders::PostCallAnalyticsSettingsBuilder {
        crate::types::builders::PostCallAnalyticsSettingsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::PostCallAnalyticsSettings;
/// A builder for [`PostCallAnalyticsSettings`](crate::types::PostCallAnalyticsSettings).
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
pub struct PostCallAnalyticsSettingsBuilder {
    pub(crate) output_location: std::option::Option<std::string::String>,
    pub(crate) data_access_role_arn: std::option::Option<std::string::String>,
    pub(crate) content_redaction_output: std::option::Option<crate::types::ContentRedactionOutput>,
    pub(crate) output_encryption_kms_key_id: std::option::Option<std::string::String>,
}
impl PostCallAnalyticsSettingsBuilder {
    /// <p>The Amazon S3 location where you want your Call Analytics post-call transcription output stored. You can use any of the following formats to specify the output location:</p>
    /// <ol>
    /// <li> <p>s3://DOC-EXAMPLE-BUCKET</p> </li>
    /// <li> <p>s3://DOC-EXAMPLE-BUCKET/my-output-folder/</p> </li>
    /// <li> <p>s3://DOC-EXAMPLE-BUCKET/my-output-folder/my-call-analytics-job.json</p> </li>
    /// </ol>
    pub fn output_location(mut self, input: impl Into<std::string::String>) -> Self {
        self.output_location = Some(input.into());
        self
    }
    /// <p>The Amazon S3 location where you want your Call Analytics post-call transcription output stored. You can use any of the following formats to specify the output location:</p>
    /// <ol>
    /// <li> <p>s3://DOC-EXAMPLE-BUCKET</p> </li>
    /// <li> <p>s3://DOC-EXAMPLE-BUCKET/my-output-folder/</p> </li>
    /// <li> <p>s3://DOC-EXAMPLE-BUCKET/my-output-folder/my-call-analytics-job.json</p> </li>
    /// </ol>
    pub fn set_output_location(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.output_location = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role that has permissions to access the Amazon S3 bucket that contains your input files. If the role that you specify doesn’t have the appropriate permissions to access the specified Amazon S3 location, your request fails.</p>
    /// <p>IAM role ARNs have the format <code>arn:partition:iam::account:role/role-name-with-path</code>. For example: <code>arn:aws:iam::111122223333:role/Admin</code>. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a>.</p>
    pub fn data_access_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.data_access_role_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role that has permissions to access the Amazon S3 bucket that contains your input files. If the role that you specify doesn’t have the appropriate permissions to access the specified Amazon S3 location, your request fails.</p>
    /// <p>IAM role ARNs have the format <code>arn:partition:iam::account:role/role-name-with-path</code>. For example: <code>arn:aws:iam::111122223333:role/Admin</code>. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a>.</p>
    pub fn set_data_access_role_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.data_access_role_arn = input;
        self
    }
    /// <p>Specify whether you want only a redacted transcript or both a redacted and an unredacted transcript. If you choose redacted and unredacted, two JSON files are generated and stored in the Amazon S3 output location you specify.</p>
    /// <p>Note that to include <code>ContentRedactionOutput</code> in your request, you must enable content redaction (<code>ContentRedactionType</code>).</p>
    pub fn content_redaction_output(mut self, input: crate::types::ContentRedactionOutput) -> Self {
        self.content_redaction_output = Some(input);
        self
    }
    /// <p>Specify whether you want only a redacted transcript or both a redacted and an unredacted transcript. If you choose redacted and unredacted, two JSON files are generated and stored in the Amazon S3 output location you specify.</p>
    /// <p>Note that to include <code>ContentRedactionOutput</code> in your request, you must enable content redaction (<code>ContentRedactionType</code>).</p>
    pub fn set_content_redaction_output(
        mut self,
        input: std::option::Option<crate::types::ContentRedactionOutput>,
    ) -> Self {
        self.content_redaction_output = input;
        self
    }
    /// <p>The KMS key you want to use to encrypt your Call Analytics post-call output.</p>
    /// <p>If using a key located in the <b>current</b> Amazon Web Services account, you can specify your KMS key in one of four ways:</p>
    /// <ol>
    /// <li> <p>Use the KMS key ID itself. For example, <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li>
    /// <li> <p>Use an alias for the KMS key ID. For example, <code>alias/ExampleAlias</code>.</p> </li>
    /// <li> <p>Use the Amazon Resource Name (ARN) for the KMS key ID. For example, <code>arn:aws:kms:region:account-ID:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li>
    /// <li> <p>Use the ARN for the KMS key alias. For example, <code>arn:aws:kms:region:account-ID:alias/ExampleAlias</code>.</p> </li>
    /// </ol>
    /// <p>If using a key located in a <b>different</b> Amazon Web Services account than the current Amazon Web Services account, you can specify your KMS key in one of two ways:</p>
    /// <ol>
    /// <li> <p>Use the ARN for the KMS key ID. For example, <code>arn:aws:kms:region:account-ID:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li>
    /// <li> <p>Use the ARN for the KMS key alias. For example, <code>arn:aws:kms:region:account-ID:alias/ExampleAlias</code>.</p> </li>
    /// </ol>
    /// <p>Note that the user making the request must have permission to use the specified KMS key.</p>
    pub fn output_encryption_kms_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.output_encryption_kms_key_id = Some(input.into());
        self
    }
    /// <p>The KMS key you want to use to encrypt your Call Analytics post-call output.</p>
    /// <p>If using a key located in the <b>current</b> Amazon Web Services account, you can specify your KMS key in one of four ways:</p>
    /// <ol>
    /// <li> <p>Use the KMS key ID itself. For example, <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li>
    /// <li> <p>Use an alias for the KMS key ID. For example, <code>alias/ExampleAlias</code>.</p> </li>
    /// <li> <p>Use the Amazon Resource Name (ARN) for the KMS key ID. For example, <code>arn:aws:kms:region:account-ID:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li>
    /// <li> <p>Use the ARN for the KMS key alias. For example, <code>arn:aws:kms:region:account-ID:alias/ExampleAlias</code>.</p> </li>
    /// </ol>
    /// <p>If using a key located in a <b>different</b> Amazon Web Services account than the current Amazon Web Services account, you can specify your KMS key in one of two ways:</p>
    /// <ol>
    /// <li> <p>Use the ARN for the KMS key ID. For example, <code>arn:aws:kms:region:account-ID:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p> </li>
    /// <li> <p>Use the ARN for the KMS key alias. For example, <code>arn:aws:kms:region:account-ID:alias/ExampleAlias</code>.</p> </li>
    /// </ol>
    /// <p>Note that the user making the request must have permission to use the specified KMS key.</p>
    pub fn set_output_encryption_kms_key_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.output_encryption_kms_key_id = input;
        self
    }
    /// Consumes the builder and constructs a [`PostCallAnalyticsSettings`](crate::types::PostCallAnalyticsSettings).
    pub fn build(self) -> crate::types::PostCallAnalyticsSettings {
        crate::types::PostCallAnalyticsSettings {
            output_location: self.output_location,
            data_access_role_arn: self.data_access_role_arn,
            content_redaction_output: self.content_redaction_output,
            output_encryption_kms_key_id: self.output_encryption_kms_key_id,
        }
    }
}