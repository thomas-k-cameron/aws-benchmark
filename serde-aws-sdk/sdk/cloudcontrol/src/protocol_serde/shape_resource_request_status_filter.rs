// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_resource_request_status_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ResourceRequestStatusFilter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.operations {
        let mut array_2 = object.key("Operations").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.operation_statuses {
        let mut array_5 = object.key("OperationStatuses").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    Ok(())
}