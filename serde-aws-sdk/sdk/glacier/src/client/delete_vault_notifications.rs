// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteVaultNotifications`](crate::operation::delete_vault_notifications::builders::DeleteVaultNotificationsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::delete_vault_notifications::builders::DeleteVaultNotificationsFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::delete_vault_notifications::builders::DeleteVaultNotificationsFluentBuilder::set_account_id): <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    ///   - [`vault_name(impl Into<String>)`](crate::operation::delete_vault_notifications::builders::DeleteVaultNotificationsFluentBuilder::vault_name) / [`set_vault_name(Option<String>)`](crate::operation::delete_vault_notifications::builders::DeleteVaultNotificationsFluentBuilder::set_vault_name): <p>The name of the vault.</p>
    /// - On success, responds with [`DeleteVaultNotificationsOutput`](crate::operation::delete_vault_notifications::DeleteVaultNotificationsOutput)
    /// - On failure, responds with [`SdkError<DeleteVaultNotificationsError>`](crate::operation::delete_vault_notifications::DeleteVaultNotificationsError)
    pub fn delete_vault_notifications(
        &self,
    ) -> crate::operation::delete_vault_notifications::builders::DeleteVaultNotificationsFluentBuilder
    {
        crate::operation::delete_vault_notifications::builders::DeleteVaultNotificationsFluentBuilder::new(self.handle.clone())
    }
}