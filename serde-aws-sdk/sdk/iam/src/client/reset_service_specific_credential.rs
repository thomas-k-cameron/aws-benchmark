// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ResetServiceSpecificCredential`](crate::operation::reset_service_specific_credential::builders::ResetServiceSpecificCredentialFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::operation::reset_service_specific_credential::builders::ResetServiceSpecificCredentialFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::reset_service_specific_credential::builders::ResetServiceSpecificCredentialFluentBuilder::set_user_name): <p>The name of the IAM user associated with the service-specific credential. If this value is not specified, then the operation assumes the user whose credentials are used to call the operation.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`service_specific_credential_id(impl Into<String>)`](crate::operation::reset_service_specific_credential::builders::ResetServiceSpecificCredentialFluentBuilder::service_specific_credential_id) / [`set_service_specific_credential_id(Option<String>)`](crate::operation::reset_service_specific_credential::builders::ResetServiceSpecificCredentialFluentBuilder::set_service_specific_credential_id): <p>The unique identifier of the service-specific credential.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that can consist of any upper or lowercased letter or digit.</p>
    /// - On success, responds with [`ResetServiceSpecificCredentialOutput`](crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialOutput) with field(s):
    ///   - [`service_specific_credential(Option<ServiceSpecificCredential>)`](crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialOutput::service_specific_credential): <p>A structure with details about the updated service-specific credential, including the new password.</p> <important>   <p>This is the <b>only</b> time that you can access the password. You cannot recover the password later, but you can reset it again.</p>  </important>
    /// - On failure, responds with [`SdkError<ResetServiceSpecificCredentialError>`](crate::operation::reset_service_specific_credential::ResetServiceSpecificCredentialError)
    pub fn reset_service_specific_credential(&self) -> crate::operation::reset_service_specific_credential::builders::ResetServiceSpecificCredentialFluentBuilder{
        crate::operation::reset_service_specific_credential::builders::ResetServiceSpecificCredentialFluentBuilder::new(self.handle.clone())
    }
}