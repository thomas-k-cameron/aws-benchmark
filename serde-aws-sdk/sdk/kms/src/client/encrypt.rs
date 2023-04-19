// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`Encrypt`](crate::operation::encrypt::builders::EncryptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<String>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_key_id): <p>Identifies the KMS key to use in the encryption operation. The KMS key must have a <code>KeyUsage</code> of <code>ENCRYPT_DECRYPT</code>. To find the <code>KeyUsage</code> of a KMS key, use the <code>DescribeKey</code> operation.</p>  <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>  <p>For example:</p>  <ul>   <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>   <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li>  </ul>  <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    ///   - [`plaintext(Blob)`](crate::operation::encrypt::builders::EncryptFluentBuilder::plaintext) / [`set_plaintext(Option<Blob>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_plaintext): <p>Data to be encrypted.</p>
    ///   - [`encryption_context(HashMap<String, String>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::encryption_context) / [`set_encryption_context(Option<HashMap<String, String>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_encryption_context): <p>Specifies the encryption context that will be used to encrypt the data. An encryption context is valid only for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#cryptographic-operations">cryptographic operations</a> with a symmetric encryption KMS key. The standard asymmetric encryption algorithms and HMAC algorithms that KMS uses do not support an encryption context. </p>  <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represent additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is supported only on operations with symmetric encryption KMS keys. On operations with symmetric encryption KMS keys, an encryption context is optional, but it is strongly recommended.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption context</a> in the <i>Key Management Service Developer Guide</i>.</p>
    ///   - [`grant_tokens(Vec<String>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec<String>>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_grant_tokens): <p>A list of grant tokens.</p>  <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    ///   - [`encryption_algorithm(EncryptionAlgorithmSpec)`](crate::operation::encrypt::builders::EncryptFluentBuilder::encryption_algorithm) / [`set_encryption_algorithm(Option<EncryptionAlgorithmSpec>)`](crate::operation::encrypt::builders::EncryptFluentBuilder::set_encryption_algorithm): <p>Specifies the encryption algorithm that KMS will use to encrypt the plaintext message. The algorithm must be compatible with the KMS key that you specify.</p>  <p>This parameter is required only for asymmetric KMS keys. The default value, <code>SYMMETRIC_DEFAULT</code>, is the algorithm used for symmetric encryption KMS keys. If you are using an asymmetric KMS key, we recommend RSAES_OAEP_SHA_256.</p>  <p>The SM2PKE algorithm is only available in China Regions.</p>
    /// - On success, responds with [`EncryptOutput`](crate::operation::encrypt::EncryptOutput) with field(s):
    ///   - [`ciphertext_blob(Option<Blob>)`](crate::operation::encrypt::EncryptOutput::ciphertext_blob): <p>The encrypted plaintext. When you use the HTTP API or the Amazon Web Services CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    ///   - [`key_id(Option<String>)`](crate::operation::encrypt::EncryptOutput::key_id): <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the KMS key that was used to encrypt the plaintext.</p>
    ///   - [`encryption_algorithm(Option<EncryptionAlgorithmSpec>)`](crate::operation::encrypt::EncryptOutput::encryption_algorithm): <p>The encryption algorithm that was used to encrypt the plaintext.</p>
    /// - On failure, responds with [`SdkError<EncryptError>`](crate::operation::encrypt::EncryptError)
    pub fn encrypt(&self) -> crate::operation::encrypt::builders::EncryptFluentBuilder {
        crate::operation::encrypt::builders::EncryptFluentBuilder::new(self.handle.clone())
    }
}