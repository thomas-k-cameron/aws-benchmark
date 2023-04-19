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
pub struct GetSshPublicKeyInput {
    /// <p>The name of the IAM user associated with the SSH public key.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    #[doc(hidden)]
    pub user_name: std::option::Option<std::string::String>,
    /// <p>The unique identifier for the SSH public key.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that can consist of any upper or lowercased letter or digit.</p>
    #[doc(hidden)]
    pub ssh_public_key_id: std::option::Option<std::string::String>,
    /// <p>Specifies the public key encoding format to use in the response. To retrieve the public key in ssh-rsa format, use <code>SSH</code>. To retrieve the public key in PEM format, use <code>PEM</code>.</p>
    #[doc(hidden)]
    pub encoding: std::option::Option<crate::types::EncodingType>,
}
impl GetSshPublicKeyInput {
    /// <p>The name of the IAM user associated with the SSH public key.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn user_name(&self) -> std::option::Option<&str> {
        self.user_name.as_deref()
    }
    /// <p>The unique identifier for the SSH public key.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that can consist of any upper or lowercased letter or digit.</p>
    pub fn ssh_public_key_id(&self) -> std::option::Option<&str> {
        self.ssh_public_key_id.as_deref()
    }
    /// <p>Specifies the public key encoding format to use in the response. To retrieve the public key in ssh-rsa format, use <code>SSH</code>. To retrieve the public key in PEM format, use <code>PEM</code>.</p>
    pub fn encoding(&self) -> std::option::Option<&crate::types::EncodingType> {
        self.encoding.as_ref()
    }
}
impl GetSshPublicKeyInput {
    /// Creates a new builder-style object to manufacture [`GetSshPublicKeyInput`](crate::operation::get_ssh_public_key::GetSshPublicKeyInput).
    pub fn builder() -> crate::operation::get_ssh_public_key::builders::GetSshPublicKeyInputBuilder
    {
        crate::operation::get_ssh_public_key::builders::GetSshPublicKeyInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_ssh_public_key::GetSshPublicKeyInput;
/// A builder for [`GetSshPublicKeyInput`](crate::operation::get_ssh_public_key::GetSshPublicKeyInput).
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
pub struct GetSshPublicKeyInputBuilder {
    pub(crate) user_name: std::option::Option<std::string::String>,
    pub(crate) ssh_public_key_id: std::option::Option<std::string::String>,
    pub(crate) encoding: std::option::Option<crate::types::EncodingType>,
}
impl GetSshPublicKeyInputBuilder {
    /// <p>The name of the IAM user associated with the SSH public key.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn user_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.user_name = Some(input.into());
        self
    }
    /// <p>The name of the IAM user associated with the SSH public key.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_user_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.user_name = input;
        self
    }
    /// <p>The unique identifier for the SSH public key.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that can consist of any upper or lowercased letter or digit.</p>
    pub fn ssh_public_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.ssh_public_key_id = Some(input.into());
        self
    }
    /// <p>The unique identifier for the SSH public key.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters that can consist of any upper or lowercased letter or digit.</p>
    pub fn set_ssh_public_key_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.ssh_public_key_id = input;
        self
    }
    /// <p>Specifies the public key encoding format to use in the response. To retrieve the public key in ssh-rsa format, use <code>SSH</code>. To retrieve the public key in PEM format, use <code>PEM</code>.</p>
    pub fn encoding(mut self, input: crate::types::EncodingType) -> Self {
        self.encoding = Some(input);
        self
    }
    /// <p>Specifies the public key encoding format to use in the response. To retrieve the public key in ssh-rsa format, use <code>SSH</code>. To retrieve the public key in PEM format, use <code>PEM</code>.</p>
    pub fn set_encoding(mut self, input: std::option::Option<crate::types::EncodingType>) -> Self {
        self.encoding = input;
        self
    }
    /// Consumes the builder and constructs a [`GetSshPublicKeyInput`](crate::operation::get_ssh_public_key::GetSshPublicKeyInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_ssh_public_key::GetSshPublicKeyInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::get_ssh_public_key::GetSshPublicKeyInput {
            user_name: self.user_name,
            ssh_public_key_id: self.ssh_public_key_id,
            encoding: self.encoding,
        })
    }
}