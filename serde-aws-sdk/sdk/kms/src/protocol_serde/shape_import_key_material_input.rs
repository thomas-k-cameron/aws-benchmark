// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_import_key_material_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::import_key_material::ImportKeyMaterialInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.key_id {
        object.key("KeyId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.import_token {
        object
            .key("ImportToken")
            .string_unchecked(&aws_smithy_types::base64::encode(var_2));
    }
    if let Some(var_3) = &input.encrypted_key_material {
        object
            .key("EncryptedKeyMaterial")
            .string_unchecked(&aws_smithy_types::base64::encode(var_3));
    }
    if let Some(var_4) = &input.valid_to {
        object
            .key("ValidTo")
            .date_time(var_4, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_5) = &input.expiration_model {
        object.key("ExpirationModel").string(var_5.as_str());
    }
    Ok(())
}