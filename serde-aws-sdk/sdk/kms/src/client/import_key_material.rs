// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ImportKeyMaterial`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<String>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_key_id): <p>The identifier of the symmetric encryption KMS key that receives the imported key material. This must be the same KMS key specified in the <code>KeyID</code> parameter of the corresponding <code>GetParametersForImport</code> request. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>. You cannot perform this operation on an asymmetric KMS key, an HMAC KMS key, a KMS key in a custom key store, or on a KMS key in a different Amazon Web Services account</p>  <p>Specify the key ID or key ARN of the KMS key.</p>  <p>For example:</p>  <ul>   <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>  </ul>  <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    ///   - [`import_token(Blob)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::import_token) / [`set_import_token(Option<Blob>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_import_token): <p>The import token that you received in the response to a previous <code>GetParametersForImport</code> request. It must be from the same response that contained the public key that you used to encrypt the key material.</p>
    ///   - [`encrypted_key_material(Blob)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::encrypted_key_material) / [`set_encrypted_key_material(Option<Blob>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_encrypted_key_material): <p>The encrypted key material to import. The key material must be encrypted with the public wrapping key that <code>GetParametersForImport</code> returned, using the wrapping algorithm that you specified in the same <code>GetParametersForImport</code> request.</p>
    ///   - [`valid_to(DateTime)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::valid_to) / [`set_valid_to(Option<DateTime>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_valid_to): <p>The date and time when the imported key material expires. This parameter is required when the value of the <code>ExpirationModel</code> parameter is <code>KEY_MATERIAL_EXPIRES</code>. Otherwise it is not valid.</p>  <p>The value of this parameter must be a future date and time. The maximum value is 365 days from the request date.</p>  <p>When the key material expires, KMS deletes the key material from the KMS key. Without its key material, the KMS key is unusable. To use the KMS key in cryptographic operations, you must reimport the same key material.</p>  <p>You cannot change the <code>ExpirationModel</code> or <code>ValidTo</code> values for the current import after the request completes. To change either value, you must delete (<code>DeleteImportedKeyMaterial</code>) and reimport the key material.</p>
    ///   - [`expiration_model(ExpirationModelType)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::expiration_model) / [`set_expiration_model(Option<ExpirationModelType>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_expiration_model): <p>Specifies whether the key material expires. The default is <code>KEY_MATERIAL_EXPIRES</code>.</p>  <p>When the value of <code>ExpirationModel</code> is <code>KEY_MATERIAL_EXPIRES</code>, you must specify a value for the <code>ValidTo</code> parameter. When value is <code>KEY_MATERIAL_DOES_NOT_EXPIRE</code>, you must omit the <code>ValidTo</code> parameter.</p>  <p>You cannot change the <code>ExpirationModel</code> or <code>ValidTo</code> values for the current import after the request completes. To change either value, you must delete (<code>DeleteImportedKeyMaterial</code>) and reimport the key material.</p>
    /// - On success, responds with [`ImportKeyMaterialOutput`](crate::operation::import_key_material::ImportKeyMaterialOutput)
    /// - On failure, responds with [`SdkError<ImportKeyMaterialError>`](crate::operation::import_key_material::ImportKeyMaterialError)
    pub fn import_key_material(
        &self,
    ) -> crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder {
        crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::new(
            self.handle.clone(),
        )
    }
}