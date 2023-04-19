// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_assume_role_policy_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyOutput,
    crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code =
        match generic.code() {
            Some(code) => code,
            None => return Err(
                crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError::unhandled(
                    generic,
                ),
            ),
        };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "LimitExceeded" => crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MalformedPolicyDocument" => crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError::MalformedPolicyDocumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::MalformedPolicyDocumentExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_malformed_policy_document_exception::de_malformed_policy_document_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchEntity" => crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceFailure" => crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnmodifiableEntity" => crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError::UnmodifiableEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnmodifiableEntityExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unmodifiable_entity_exception::de_unmodifiable_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_assume_role_policy_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyOutput,
    crate::operation::update_assume_role_policy::UpdateAssumeRolePolicyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_assume_role_policy::builders::UpdateAssumeRolePolicyOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}