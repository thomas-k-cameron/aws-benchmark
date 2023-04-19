// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetEbsEncryptionByDefault`](crate::operation::get_ebs_encryption_by_default::builders::GetEbsEncryptionByDefaultFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::get_ebs_encryption_by_default::builders::GetEbsEncryptionByDefaultFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_ebs_encryption_by_default::builders::GetEbsEncryptionByDefaultFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`GetEbsEncryptionByDefaultOutput`](crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultOutput) with field(s):
    ///   - [`ebs_encryption_by_default(Option<bool>)`](crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultOutput::ebs_encryption_by_default): <p>Indicates whether encryption by default is enabled.</p>
    /// - On failure, responds with [`SdkError<GetEbsEncryptionByDefaultError>`](crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultError)
    pub fn get_ebs_encryption_by_default(&self) -> crate::operation::get_ebs_encryption_by_default::builders::GetEbsEncryptionByDefaultFluentBuilder{
        crate::operation::get_ebs_encryption_by_default::builders::GetEbsEncryptionByDefaultFluentBuilder::new(self.handle.clone())
    }
}