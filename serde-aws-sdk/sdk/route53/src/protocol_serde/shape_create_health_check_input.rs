// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_health_check_input_input(
    input: &crate::operation::create_health_check::CreateHealthCheckInput,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.caller_reference {
        let mut inner_writer = scope.start_el("CallerReference").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.health_check_config {
        let inner_writer = scope.start_el("HealthCheckConfig");
        crate::protocol_serde::shape_health_check_config::ser_health_check_config(
            var_2,
            inner_writer,
        )?
    }
    scope.finish();
    Ok(())
}