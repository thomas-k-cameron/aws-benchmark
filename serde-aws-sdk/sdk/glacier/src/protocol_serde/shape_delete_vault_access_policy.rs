// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_vault_access_policy_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyOutput,
    crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidParameterValueException" => crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MissingParameterValueException" => crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError::MissingParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::MissingParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_missing_parameter_value_exception::de_missing_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_vault_access_policy_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyOutput,
    crate::operation::delete_vault_access_policy::DeleteVaultAccessPolicyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_vault_access_policy::builders::DeleteVaultAccessPolicyOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}