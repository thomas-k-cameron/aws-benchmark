// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_encrypt_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::encrypt::EncryptInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.key_id {
        object.key("KeyId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.plaintext {
        object
            .key("Plaintext")
            .string_unchecked(&aws_smithy_types::base64::encode(var_2));
    }
    if let Some(var_3) = &input.encryption_context {
        #[allow(unused_mut)]
        let mut object_4 = object.key("EncryptionContext").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    if let Some(var_7) = &input.grant_tokens {
        let mut array_8 = object.key("GrantTokens").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.encryption_algorithm {
        object.key("EncryptionAlgorithm").string(var_10.as_str());
    }
    Ok(())
}