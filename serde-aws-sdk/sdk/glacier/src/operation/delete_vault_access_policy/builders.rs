// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_vault_access_policy::_delete_vault_access_policy_output::DeleteVaultAccessPolicyOutputBuilder;

pub use crate::operation::delete_vault_access_policy::_delete_vault_access_policy_input::DeleteVaultAccessPolicyInputBuilder;

/// Fluent builder constructing a request to `DeleteVaultAccessPolicy`.
///
/// <p>This operation deletes the access policy associated with the specified vault. The operation is eventually consistent; that is, it might take some time for Amazon S3 Glacier to completely remove the access policy, and you might still see the effect of the policy for a short time after you send the delete request.</p>
/// <p>This operation is idempotent. You can invoke delete multiple times, even if there is no policy associated with the vault. For more information about vault access policies, see <a href="https://docs.aws.amazon.com/amazonglacier/latest/dev/vault-access-policy.html">Amazon Glacier Access Control with Vault Access Policies</a>. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteVaultAccessPolicyFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::delete_vault_access_policy::builders::DeleteVaultAccessPolicyInputBuilder,
}
impl DeleteVaultAccessPolicyFluentBuilder {
    /// Creates a new `DeleteVaultAccessPolicy`.
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
            crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicy,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError,
        >,
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
        crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError,
        >,
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
    ///     let deserialized_parameters: crate::operation::delete_vault_access_policy::builders::DeleteVaultAccessPolicyInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_vault_access_policy().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_vault_access_policy::builders::DeleteVaultAccessPolicyInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The name of the vault.</p>
    pub fn vault_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.vault_name(input.into());
        self
    }
    /// <p>The name of the vault.</p>
    pub fn set_vault_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_vault_name(input);
        self
    }
}