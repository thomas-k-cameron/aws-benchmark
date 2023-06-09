// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_generate_random_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::generate_random::GenerateRandomInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.number_of_bytes {
        object.key("NumberOfBytes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.custom_key_store_id {
        object.key("CustomKeyStoreId").string(var_2.as_str());
    }
    Ok(())
}
