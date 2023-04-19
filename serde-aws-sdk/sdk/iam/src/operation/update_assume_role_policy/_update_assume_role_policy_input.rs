// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct UpdateAssumeRolePolicyInput {
    /// <p>The name of the role to update with the new policy.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub role_name: std::option::Option<std::string::String>,
    /// <p>The policy that grants an entity permission to assume the role.</p>
    /// <p>You must provide policies in JSON format in IAM. However, for CloudFormation templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to IAM.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>
    /// <ul>
    /// <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>
    /// <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>
    /// <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub policy_document: std::option::Option<std::string::String>,
}
impl UpdateAssumeRolePolicyInput {
    /// <p>The name of the role to update with the new policy.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn role_name(&self) -> std::option::Option<&str> {
        self.role_name.as_deref()
    }
    /// <p>The policy that grants an entity permission to assume the role.</p>
    /// <p>You must provide policies in JSON format in IAM. However, for CloudFormation templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to IAM.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>
    /// <ul>
    /// <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>
    /// <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>
    /// <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>
    /// </ul>
    pub fn policy_document(&self) -> std::option::Option<&str> {
        self.policy_document.as_deref()
    }
}
impl UpdateAssumeRolePolicyInput {
    /// Creates a new builder-style object to manufacture [`UpdateAssumeRolePolicyInput`](crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyInput).
    pub fn builder(
    ) -> crate::operation::update_assume_role_policy::builders::UpdateAssumeRolePolicyInputBuilder
    {
        crate::operation::update_assume_role_policy::builders::UpdateAssumeRolePolicyInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyInput;
/// A builder for [`UpdateAssumeRolePolicyInput`](crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyInput).
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
pub struct UpdateAssumeRolePolicyInputBuilder {
    pub(crate) role_name: std::option::Option<std::string::String>,
    pub(crate) policy_document: std::option::Option<std::string::String>,
}
impl UpdateAssumeRolePolicyInputBuilder {
    /// <p>The name of the role to update with the new policy.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn role_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.role_name = Some(input.into());
        self
    }
    /// <p>The name of the role to update with the new policy.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_role_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.role_name = input;
        self
    }
    /// <p>The policy that grants an entity permission to assume the role.</p>
    /// <p>You must provide policies in JSON format in IAM. However, for CloudFormation templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to IAM.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>
    /// <ul>
    /// <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>
    /// <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>
    /// <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>
    /// </ul>
    pub fn policy_document(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_document = Some(input.into());
        self
    }
    /// <p>The policy that grants an entity permission to assume the role.</p>
    /// <p>You must provide policies in JSON format in IAM. However, for CloudFormation templates formatted in YAML, you can provide the policy in JSON or YAML format. CloudFormation always converts a YAML policy to JSON format before submitting it to IAM.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>
    /// <ul>
    /// <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>
    /// <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>
    /// <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>
    /// </ul>
    pub fn set_policy_document(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy_document = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateAssumeRolePolicyInput`](crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyInput {
                role_name: self.role_name,
                policy_document: self.policy_document,
            },
        )
    }
}