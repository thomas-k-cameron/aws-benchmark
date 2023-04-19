// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetLoginProfile`](crate::operation::get_login_profile::builders::GetLoginProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::operation::get_login_profile::builders::GetLoginProfileFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::get_login_profile::builders::GetLoginProfileFluentBuilder::set_user_name): <p>The name of the user whose login profile you want to retrieve.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    /// - On success, responds with [`GetLoginProfileOutput`](crate::operation::get_login_profile::GetLoginProfileOutput) with field(s):
    ///   - [`login_profile(Option<LoginProfile>)`](crate::operation::get_login_profile::GetLoginProfileOutput::login_profile): <p>A structure containing the user name and the profile creation date for the user.</p>
    /// - On failure, responds with [`SdkError<GetLoginProfileError>`](crate::operation::get_login_profile::GetLoginProfileError)
    pub fn get_login_profile(
        &self,
    ) -> crate::operation::get_login_profile::builders::GetLoginProfileFluentBuilder {
        crate::operation::get_login_profile::builders::GetLoginProfileFluentBuilder::new(
            self.handle.clone(),
        )
    }
}