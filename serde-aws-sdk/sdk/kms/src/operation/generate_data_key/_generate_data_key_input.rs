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
pub struct GenerateDataKeyInput {
    /// <p>Specifies the symmetric encryption KMS key that encrypts the data key. You cannot specify an asymmetric KMS key or a KMS key in a custom key store. To get the type and origin of your KMS key, use the <code>DescribeKey</code> operation.</p>
    /// <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>
    /// <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    #[doc(hidden)]
    pub key_id: std::option::Option<std::string::String>,
    /// <p>Specifies the encryption context that will be used when encrypting the data key.</p>
    /// <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p>
    #[doc(hidden)]
    pub encryption_context:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>Specifies the length of the data key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For 128-bit (16-byte) and 256-bit (32-byte) data keys, use the <code>KeySpec</code> parameter.</p>
    /// <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p>
    #[doc(hidden)]
    pub number_of_bytes: std::option::Option<i32>,
    /// <p>Specifies the length of the data key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p>
    /// <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p>
    #[doc(hidden)]
    pub key_spec: std::option::Option<crate::types::DataKeySpec>,
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    #[doc(hidden)]
    pub grant_tokens: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl GenerateDataKeyInput {
    /// <p>Specifies the symmetric encryption KMS key that encrypts the data key. You cannot specify an asymmetric KMS key or a KMS key in a custom key store. To get the type and origin of your KMS key, use the <code>DescribeKey</code> operation.</p>
    /// <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>
    /// <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    pub fn key_id(&self) -> std::option::Option<&str> {
        self.key_id.as_deref()
    }
    /// <p>Specifies the encryption context that will be used when encrypting the data key.</p>
    /// <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn encryption_context(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.encryption_context.as_ref()
    }
    /// <p>Specifies the length of the data key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For 128-bit (16-byte) and 256-bit (32-byte) data keys, use the <code>KeySpec</code> parameter.</p>
    /// <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p>
    pub fn number_of_bytes(&self) -> std::option::Option<i32> {
        self.number_of_bytes
    }
    /// <p>Specifies the length of the data key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p>
    /// <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p>
    pub fn key_spec(&self) -> std::option::Option<&crate::types::DataKeySpec> {
        self.key_spec.as_ref()
    }
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn grant_tokens(&self) -> std::option::Option<&[std::string::String]> {
        self.grant_tokens.as_deref()
    }
}
impl GenerateDataKeyInput {
    /// Creates a new builder-style object to manufacture [`GenerateDataKeyInput`](crate::operation::generate_data_key::GenerateDataKeyInput).
    pub fn builder() -> crate::operation::generate_data_key::builders::GenerateDataKeyInputBuilder {
        crate::operation::generate_data_key::builders::GenerateDataKeyInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::generate_data_key::GenerateDataKeyInput;
/// A builder for [`GenerateDataKeyInput`](crate::operation::generate_data_key::GenerateDataKeyInput).
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
pub struct GenerateDataKeyInputBuilder {
    pub(crate) key_id: std::option::Option<std::string::String>,
    pub(crate) encryption_context:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    pub(crate) number_of_bytes: std::option::Option<i32>,
    pub(crate) key_spec: std::option::Option<crate::types::DataKeySpec>,
    pub(crate) grant_tokens: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl GenerateDataKeyInputBuilder {
    /// <p>Specifies the symmetric encryption KMS key that encrypts the data key. You cannot specify an asymmetric KMS key or a KMS key in a custom key store. To get the type and origin of your KMS key, use the <code>DescribeKey</code> operation.</p>
    /// <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>
    /// <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    pub fn key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.key_id = Some(input.into());
        self
    }
    /// <p>Specifies the symmetric encryption KMS key that encrypts the data key. You cannot specify an asymmetric KMS key or a KMS key in a custom key store. To get the type and origin of your KMS key, use the <code>DescribeKey</code> operation.</p>
    /// <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>
    /// <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    pub fn set_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.key_id = input;
        self
    }
    /// Adds a key-value pair to `encryption_context`.
    ///
    /// To override the contents of this collection use [`set_encryption_context`](Self::set_encryption_context).
    ///
    /// <p>Specifies the encryption context that will be used when encrypting the data key.</p>
    /// <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn encryption_context(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        let mut hash_map = self.encryption_context.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.encryption_context = Some(hash_map);
        self
    }
    /// <p>Specifies the encryption context that will be used when encrypting the data key.</p>
    /// <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn set_encryption_context(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.encryption_context = input;
        self
    }
    /// <p>Specifies the length of the data key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For 128-bit (16-byte) and 256-bit (32-byte) data keys, use the <code>KeySpec</code> parameter.</p>
    /// <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p>
    pub fn number_of_bytes(mut self, input: i32) -> Self {
        self.number_of_bytes = Some(input);
        self
    }
    /// <p>Specifies the length of the data key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For 128-bit (16-byte) and 256-bit (32-byte) data keys, use the <code>KeySpec</code> parameter.</p>
    /// <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p>
    pub fn set_number_of_bytes(mut self, input: std::option::Option<i32>) -> Self {
        self.number_of_bytes = input;
        self
    }
    /// <p>Specifies the length of the data key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p>
    /// <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p>
    pub fn key_spec(mut self, input: crate::types::DataKeySpec) -> Self {
        self.key_spec = Some(input);
        self
    }
    /// <p>Specifies the length of the data key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p>
    /// <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p>
    pub fn set_key_spec(mut self, input: std::option::Option<crate::types::DataKeySpec>) -> Self {
        self.key_spec = input;
        self
    }
    /// Appends an item to `grant_tokens`.
    ///
    /// To override the contents of this collection use [`set_grant_tokens`](Self::set_grant_tokens).
    ///
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn grant_tokens(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.grant_tokens.unwrap_or_default();
        v.push(input.into());
        self.grant_tokens = Some(v);
        self
    }
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn set_grant_tokens(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.grant_tokens = input;
        self
    }
    /// Consumes the builder and constructs a [`GenerateDataKeyInput`](crate::operation::generate_data_key::GenerateDataKeyInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::generate_data_key::GenerateDataKeyInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::generate_data_key::GenerateDataKeyInput {
            key_id: self.key_id,
            encryption_context: self.encryption_context,
            number_of_bytes: self.number_of_bytes,
            key_spec: self.key_spec,
            grant_tokens: self.grant_tokens,
        })
    }
}