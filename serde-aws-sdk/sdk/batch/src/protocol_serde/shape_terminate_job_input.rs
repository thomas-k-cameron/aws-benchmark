// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_terminate_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::terminate_job::TerminateJobInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.job_id {
        object.key("jobId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.reason {
        object.key("reason").string(var_2.as_str());
    }
    Ok(())
}