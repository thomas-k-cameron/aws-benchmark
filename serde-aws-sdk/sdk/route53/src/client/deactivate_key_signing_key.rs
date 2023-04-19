// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeactivateKeySigningKey`](crate::operation::deactivate_key_signing_key::builders::DeactivateKeySigningKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`hosted_zone_id(impl Into<String>)`](crate::operation::deactivate_key_signing_key::builders::DeactivateKeySigningKeyFluentBuilder::hosted_zone_id) / [`set_hosted_zone_id(Option<String>)`](crate::operation::deactivate_key_signing_key::builders::DeactivateKeySigningKeyFluentBuilder::set_hosted_zone_id): <p>A unique string used to identify a hosted zone.</p>
    ///   - [`name(impl Into<String>)`](crate::operation::deactivate_key_signing_key::builders::DeactivateKeySigningKeyFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::deactivate_key_signing_key::builders::DeactivateKeySigningKeyFluentBuilder::set_name): <p>A string used to identify a key-signing key (KSK).</p>
    /// - On success, responds with [`DeactivateKeySigningKeyOutput`](crate::operation::deactivate_key_signing_key::DeactivateKeySigningKeyOutput) with field(s):
    ///   - [`change_info(Option<ChangeInfo>)`](crate::operation::deactivate_key_signing_key::DeactivateKeySigningKeyOutput::change_info): <p>A complex type that describes change information about changes made to your hosted zone.</p>
    /// - On failure, responds with [`SdkError<DeactivateKeySigningKeyError>`](crate::operation::deactivate_key_signing_key::DeactivateKeySigningKeyError)
    pub fn deactivate_key_signing_key(
        &self,
    ) -> crate::operation::deactivate_key_signing_key::builders::DeactivateKeySigningKeyFluentBuilder
    {
        crate::operation::deactivate_key_signing_key::builders::DeactivateKeySigningKeyFluentBuilder::new(self.handle.clone())
    }
}