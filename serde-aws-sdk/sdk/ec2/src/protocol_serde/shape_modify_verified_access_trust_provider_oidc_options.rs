// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_modify_verified_access_trust_provider_oidc_options(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::ModifyVerifiedAccessTrustProviderOidcOptions,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Scope");
    if let Some(var_2) = &input.scope {
        scope_1.string(var_2);
    }
    Ok(())
}
