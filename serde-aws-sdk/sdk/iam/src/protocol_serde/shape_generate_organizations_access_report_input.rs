// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_generate_organizations_access_report_input_input(
    input: &crate::operation::generate_organizations_access_report::GenerateOrganizationsAccessReportInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(
        &mut out,
        "GenerateOrganizationsAccessReport",
        "2010-05-08",
    );
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("EntityPath");
    if let Some(var_2) = &input.entity_path {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("OrganizationsPolicyId");
    if let Some(var_4) = &input.organizations_policy_id {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}