// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_reusable_delegation_set_input_input(
    input: &crate::operation::create_reusable_delegation_set::CreateReusableDelegationSetInput,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.caller_reference {
        let mut inner_writer = scope.start_el("CallerReference").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.hosted_zone_id {
        let mut inner_writer = scope.start_el("HostedZoneId").finish();
        inner_writer.data(var_2.as_str());
    }
    scope.finish();
    Ok(())
}