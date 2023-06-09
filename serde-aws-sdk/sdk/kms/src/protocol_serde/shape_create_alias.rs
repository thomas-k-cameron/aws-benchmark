// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_alias_input(
    input: &crate::operation::create_alias::CreateAliasInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_alias_input::ser_create_alias_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_alias_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_alias::CreateAliasOutput,
    crate::operation::create_alias::CreateAliasError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::create_alias::CreateAliasError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::create_alias::CreateAliasError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AlreadyExistsException" => {
            crate::operation::create_alias::CreateAliasError::AlreadyExistsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::AlreadyExistsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_already_exists_exception::de_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_alias::CreateAliasError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "DependencyTimeoutException" => {
            crate::operation::create_alias::CreateAliasError::DependencyTimeoutException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::DependencyTimeoutExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_alias::CreateAliasError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidAliasNameException" => {
            crate::operation::create_alias::CreateAliasError::InvalidAliasNameException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidAliasNameExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_alias_name_exception::de_invalid_alias_name_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_alias::CreateAliasError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "KMSInternalException" => {
            crate::operation::create_alias::CreateAliasError::KmsInternalException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::KmsInternalExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_internal_exception::de_kms_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_alias::CreateAliasError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "KMSInvalidStateException" => {
            crate::operation::create_alias::CreateAliasError::KmsInvalidStateException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::KmsInvalidStateExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_alias::CreateAliasError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "LimitExceededException" => {
            crate::operation::create_alias::CreateAliasError::LimitExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_alias::CreateAliasError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NotFoundException" => {
            crate::operation::create_alias::CreateAliasError::NotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::NotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_alias::CreateAliasError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::create_alias::CreateAliasError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_alias_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_alias::CreateAliasOutput,
    crate::operation::create_alias::CreateAliasError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::create_alias::builders::CreateAliasOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
