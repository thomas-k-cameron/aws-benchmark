// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_task_definitions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_task_definitions::ListTaskDefinitionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.family_prefix {
        object.key("familyPrefix").string(var_1.as_str());
    }
    if let Some(var_2) = &input.status {
        object.key("status").string(var_2.as_str());
    }
    if let Some(var_3) = &input.sort {
        object.key("sort").string(var_3.as_str());
    }
    if let Some(var_4) = &input.next_token {
        object.key("nextToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    Ok(())
}