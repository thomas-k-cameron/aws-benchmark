// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAccessKey`](crate::operation::delete_access_key::builders::DeleteAccessKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::operation::delete_access_key::builders::DeleteAccessKeyFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::delete_access_key::builders::DeleteAccessKeyFluentBuilder::set_user_name): <p>The name of the user whose access key pair you want to delete.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`access_key_id(impl Into<String>)`](crate::operation::delete_access_key::builders::DeleteAccessKeyFluentBuilder::access_key_id) / [`set_access_key_id(Option<String>)`](crate::operation::delete_access_key::builders::DeleteAccessKeyFluentBuilder::set_access_key_id): <p>The access key ID for the access key ID and secret access key you want to delete.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that can consist of any upper or lowercased letter or digit.</p>
    /// - On success, responds with [`DeleteAccessKeyOutput`](crate::operation::delete_access_key::DeleteAccessKeyOutput)
    /// - On failure, responds with [`SdkError<DeleteAccessKeyError>`](crate::operation::delete_access_key::DeleteAccessKeyError)
    pub fn delete_access_key(
        &self,
    ) -> crate::operation::delete_access_key::builders::DeleteAccessKeyFluentBuilder {
        crate::operation::delete_access_key::builders::DeleteAccessKeyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}