// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_eks_properties_override(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::EksPropertiesOverride,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.pod_properties {
        #[allow(unused_mut)]
        let mut object_2 = object.key("podProperties").start_object();
        crate::protocol_serde::shape_eks_pod_properties_override::ser_eks_pod_properties_override(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    Ok(())
}