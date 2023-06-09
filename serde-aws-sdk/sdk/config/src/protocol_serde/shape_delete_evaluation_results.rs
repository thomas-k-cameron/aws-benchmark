// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_evaluation_results_input(
    input: &crate::operation::delete_evaluation_results::DeleteEvaluationResultsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_evaluation_results_input::ser_delete_evaluation_results_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_evaluation_results_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_evaluation_results::DeleteEvaluationResultsOutput,
    crate::operation::delete_evaluation_results::DeleteEvaluationResultsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::delete_evaluation_results::DeleteEvaluationResultsError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::delete_evaluation_results::DeleteEvaluationResultsError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "NoSuchConfigRuleException" => crate::operation::delete_evaluation_results::DeleteEvaluationResultsError::NoSuchConfigRuleException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchConfigRuleExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_config_rule_exception::de_no_such_config_rule_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_evaluation_results::DeleteEvaluationResultsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceInUseException" => crate::operation::delete_evaluation_results::DeleteEvaluationResultsError::ResourceInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceInUseExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_evaluation_results::DeleteEvaluationResultsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_evaluation_results::DeleteEvaluationResultsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_evaluation_results_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_evaluation_results::DeleteEvaluationResultsOutput,
    crate::operation::delete_evaluation_results::DeleteEvaluationResultsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_evaluation_results::builders::DeleteEvaluationResultsOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
