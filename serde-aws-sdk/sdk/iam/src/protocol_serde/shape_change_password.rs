// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_change_password_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::change_password::ChangePasswordOutput,
    crate::operation::change_password::ChangePasswordError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::change_password::ChangePasswordError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::change_password::ChangePasswordError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "EntityTemporarilyUnmodifiable" => crate::operation::change_password::ChangePasswordError::EntityTemporarilyUnmodifiableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::EntityTemporarilyUnmodifiableExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_temporarily_unmodifiable_exception::de_entity_temporarily_unmodifiable_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::change_password::ChangePasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidUserType" => crate::operation::change_password::ChangePasswordError::InvalidUserTypeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidUserTypeExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_user_type_exception::de_invalid_user_type_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::change_password::ChangePasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceeded" => crate::operation::change_password::ChangePasswordError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::change_password::ChangePasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchEntity" => crate::operation::change_password::ChangePasswordError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::change_password::ChangePasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PasswordPolicyViolation" => crate::operation::change_password::ChangePasswordError::PasswordPolicyViolationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::PasswordPolicyViolationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_password_policy_violation_exception::de_password_policy_violation_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::change_password::ChangePasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceFailure" => crate::operation::change_password::ChangePasswordError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::change_password::ChangePasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::change_password::ChangePasswordError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_change_password_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::change_password::ChangePasswordOutput,
    crate::operation::change_password::ChangePasswordError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::change_password::builders::ChangePasswordOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}