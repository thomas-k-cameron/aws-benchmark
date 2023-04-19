// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_task_set_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_task_set::DeleteTaskSetInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cluster {
        object.key("cluster").string(var_1.as_str());
    }
    if let Some(var_2) = &input.service {
        object.key("service").string(var_2.as_str());
    }
    if let Some(var_3) = &input.task_set {
        object.key("taskSet").string(var_3.as_str());
    }
    if let Some(var_4) = &input.force {
        object.key("force").boolean(*var_4);
    }
    Ok(())
}