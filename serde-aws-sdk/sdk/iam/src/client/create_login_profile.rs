// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateLoginProfile`](crate::operation::create_login_profile::builders::CreateLoginProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::operation::create_login_profile::builders::CreateLoginProfileFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::create_login_profile::builders::CreateLoginProfileFluentBuilder::set_user_name): <p>The name of the IAM user to create a password for. The user must already exist.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`password(impl Into<String>)`](crate::operation::create_login_profile::builders::CreateLoginProfileFluentBuilder::password) / [`set_password(Option<String>)`](crate::operation::create_login_profile::builders::CreateLoginProfileFluentBuilder::set_password): <p>The new password for the user.</p>  <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of characters. That string can include almost any printable ASCII character from the space (<code>\u0020</code>) through the end of the ASCII character range (<code>\u00FF</code>). You can also include the tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) characters. Any of these characters are valid in a password. However, many tools, such as the Amazon Web Services Management Console, might restrict the ability to type certain characters because they have special meaning within that tool.</p>
    ///   - [`password_reset_required(bool)`](crate::operation::create_login_profile::builders::CreateLoginProfileFluentBuilder::password_reset_required) / [`set_password_reset_required(Option<bool>)`](crate::operation::create_login_profile::builders::CreateLoginProfileFluentBuilder::set_password_reset_required): <p>Specifies whether the user is required to set a new password on next sign-in.</p>
    /// - On success, responds with [`CreateLoginProfileOutput`](crate::operation::create_login_profile::CreateLoginProfileOutput) with field(s):
    ///   - [`login_profile(Option<LoginProfile>)`](crate::operation::create_login_profile::CreateLoginProfileOutput::login_profile): <p>A structure containing the user name and password create date.</p>
    /// - On failure, responds with [`SdkError<CreateLoginProfileError>`](crate::operation::create_login_profile::CreateLoginProfileError)
    pub fn create_login_profile(
        &self,
    ) -> crate::operation::create_login_profile::builders::CreateLoginProfileFluentBuilder {
        crate::operation::create_login_profile::builders::CreateLoginProfileFluentBuilder::new(
            self.handle.clone(),
        )
    }
}