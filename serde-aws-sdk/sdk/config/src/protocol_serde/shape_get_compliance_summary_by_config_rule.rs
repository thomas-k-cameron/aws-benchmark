// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_compliance_summary_by_config_rule_input(
    _input: &crate::operation::get_compliance_summary_by_config_rule::GetComplianceSummaryByConfigRuleInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_compliance_summary_by_config_rule_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_compliance_summary_by_config_rule::GetComplianceSummaryByConfigRuleOutput,
    crate::operation::get_compliance_summary_by_config_rule::GetComplianceSummaryByConfigRuleError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::get_compliance_summary_by_config_rule::GetComplianceSummaryByConfigRuleError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::get_compliance_summary_by_config_rule::GetComplianceSummaryByConfigRuleError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_compliance_summary_by_config_rule_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_compliance_summary_by_config_rule::GetComplianceSummaryByConfigRuleOutput,
    crate::operation::get_compliance_summary_by_config_rule::GetComplianceSummaryByConfigRuleError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_compliance_summary_by_config_rule::builders::GetComplianceSummaryByConfigRuleOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_compliance_summary_by_config_rule::de_get_compliance_summary_by_config_rule(response.body().as_ref(), output).map_err(crate::operation::get_compliance_summary_by_config_rule::GetComplianceSummaryByConfigRuleError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_get_compliance_summary_by_config_rule(value: &[u8], mut builder: crate::operation::get_compliance_summary_by_config_rule::builders::GetComplianceSummaryByConfigRuleOutputBuilder) -> Result<crate::operation::get_compliance_summary_by_config_rule::builders::GetComplianceSummaryByConfigRuleOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ComplianceSummary" => {
                        builder = builder.set_compliance_summary(
                            crate::protocol_serde::shape_compliance_summary::de_compliance_summary(
                                tokens,
                            )?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(
                    aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )),
                )
            }
        }
    }
    if tokens.next().is_some() {
        return Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "found more JSON tokens after completing parsing",
            ),
        );
    }
    Ok(builder)
}