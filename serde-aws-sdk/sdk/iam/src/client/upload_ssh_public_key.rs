// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UploadSSHPublicKey`](crate::operation::upload_ssh_public_key::builders::UploadSSHPublicKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::operation::upload_ssh_public_key::builders::UploadSSHPublicKeyFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::upload_ssh_public_key::builders::UploadSSHPublicKeyFluentBuilder::set_user_name): <p>The name of the IAM user to associate the SSH public key with.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    ///   - [`ssh_public_key_body(impl Into<String>)`](crate::operation::upload_ssh_public_key::builders::UploadSSHPublicKeyFluentBuilder::ssh_public_key_body) / [`set_ssh_public_key_body(Option<String>)`](crate::operation::upload_ssh_public_key::builders::UploadSSHPublicKeyFluentBuilder::set_ssh_public_key_body): <p>The SSH public key. The public key must be encoded in ssh-rsa format or PEM format. The minimum bit-length of the public key is 2048 bits. For example, you can generate a 2048-bit key, and the resulting PEM file is 1679 bytes long.</p>  <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> used to validate this parameter is a string of characters consisting of the following:</p>  <ul>   <li> <p>Any printable ASCII character ranging from the space character (<code>\u0020</code>) through the end of the ASCII character range</p> </li>   <li> <p>The printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>)</p> </li>   <li> <p>The special characters tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>)</p> </li>  </ul>
    /// - On success, responds with [`UploadSshPublicKeyOutput`](crate::operation::upload_ssh_public_key::UploadSshPublicKeyOutput) with field(s):
    ///   - [`ssh_public_key(Option<SshPublicKey>)`](crate::operation::upload_ssh_public_key::UploadSshPublicKeyOutput::ssh_public_key): <p>Contains information about the SSH public key.</p>
    /// - On failure, responds with [`SdkError<UploadSSHPublicKeyError>`](crate::operation::upload_ssh_public_key::UploadSSHPublicKeyError)
    pub fn upload_ssh_public_key(
        &self,
    ) -> crate::operation::upload_ssh_public_key::builders::UploadSSHPublicKeyFluentBuilder {
        crate::operation::upload_ssh_public_key::builders::UploadSSHPublicKeyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}