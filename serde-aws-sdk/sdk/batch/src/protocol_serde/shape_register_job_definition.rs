// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_register_job_definition_input(
    input: &crate::operation::register_job_definition::RegisterJobDefinitionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_register_job_definition_input::ser_register_job_definition_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_register_job_definition_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::register_job_definition::RegisterJobDefinitionOutput,
    crate::operation::register_job_definition::RegisterJobDefinitionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::register_job_definition::RegisterJobDefinitionError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code =
        match generic.code() {
            Some(code) => code,
            None => return Err(
                crate::operation::register_job_definition::RegisterJobDefinitionError::unhandled(
                    generic,
                ),
            ),
        };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ClientException" => {
            crate::operation::register_job_definition::RegisterJobDefinitionError::ClientException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::types::error::builders::ClientExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_client_exception::de_client_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::register_job_definition::RegisterJobDefinitionError::unhandled)?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        "ServerException" => {
            crate::operation::register_job_definition::RegisterJobDefinitionError::ServerException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::types::error::builders::ServerExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_server_exception::de_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::register_job_definition::RegisterJobDefinitionError::unhandled)?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        _ => {
            crate::operation::register_job_definition::RegisterJobDefinitionError::generic(generic)
        }
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_register_job_definition_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::register_job_definition::RegisterJobDefinitionOutput,
    crate::operation::register_job_definition::RegisterJobDefinitionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::register_job_definition::builders::RegisterJobDefinitionOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_register_job_definition::de_register_job_definition(
            response.body().as_ref(),
            output,
        )
        .map_err(
            crate::operation::register_job_definition::RegisterJobDefinitionError::unhandled,
        )?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_register_job_definition(
    value: &[u8],
    mut builder: crate::operation::register_job_definition::builders::RegisterJobDefinitionOutputBuilder,
) -> Result<
    crate::operation::register_job_definition::builders::RegisterJobDefinitionOutputBuilder,
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
                    "jobDefinitionArn" => {
                        builder = builder.set_job_definition_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "jobDefinitionName" => {
                        builder = builder.set_job_definition_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "revision" => {
                        builder = builder.set_revision(
                            aws_smithy_json::deserialize::token::expect_number_or_null(
                                tokens.next(),
                            )?
                            .map(i32::try_from)
                            .transpose()?,
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