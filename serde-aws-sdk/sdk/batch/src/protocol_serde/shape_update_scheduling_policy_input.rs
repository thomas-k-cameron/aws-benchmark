// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_scheduling_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_scheduling_policy::UpdateSchedulingPolicyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.arn {
        object.key("arn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.fairshare_policy {
        #[allow(unused_mut)]
        let mut object_3 = object.key("fairsharePolicy").start_object();
        crate::protocol_serde::shape_fairshare_policy::ser_fairshare_policy(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}