// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_capacity_provider_input(
    input: &crate::operation::update_capacity_provider::UpdateCapacityProviderInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_capacity_provider_input::ser_update_capacity_provider_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_capacity_provider_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_capacity_provider::UpdateCapacityProviderOutput,
    crate::operation::update_capacity_provider::UpdateCapacityProviderError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::update_capacity_provider::UpdateCapacityProviderError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code =
        match generic.code() {
            Some(code) => code,
            None => return Err(
                crate::operation::update_capacity_provider::UpdateCapacityProviderError::unhandled(
                    generic,
                ),
            ),
        };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ClientException" => crate::operation::update_capacity_provider::UpdateCapacityProviderError::ClientException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ClientExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_client_exception::de_client_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_capacity_provider::UpdateCapacityProviderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::operation::update_capacity_provider::UpdateCapacityProviderError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_capacity_provider::UpdateCapacityProviderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServerException" => crate::operation::update_capacity_provider::UpdateCapacityProviderError::ServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_server_exception::de_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_capacity_provider::UpdateCapacityProviderError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::update_capacity_provider::UpdateCapacityProviderError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_capacity_provider_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_capacity_provider::UpdateCapacityProviderOutput,
    crate::operation::update_capacity_provider::UpdateCapacityProviderError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_capacity_provider::builders::UpdateCapacityProviderOutputBuilder::default();
        let _ = response;
        output =
            crate::protocol_serde::shape_update_capacity_provider::de_update_capacity_provider(
                response.body().as_ref(),
                output,
            )
            .map_err(
                crate::operation::update_capacity_provider::UpdateCapacityProviderError::unhandled,
            )?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_update_capacity_provider(
    value: &[u8],
    mut builder: crate::operation::update_capacity_provider::builders::UpdateCapacityProviderOutputBuilder,
) -> Result<
    crate::operation::update_capacity_provider::builders::UpdateCapacityProviderOutputBuilder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
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
                    "capacityProvider" => {
                        builder = builder.set_capacity_provider(
                            crate::protocol_serde::shape_capacity_provider::de_capacity_provider(
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