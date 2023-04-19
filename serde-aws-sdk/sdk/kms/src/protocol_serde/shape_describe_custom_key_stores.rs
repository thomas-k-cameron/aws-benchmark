// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_custom_key_stores_input(
    input: &crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_custom_key_stores_input::ser_describe_custom_key_stores_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_custom_key_stores_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresOutput,
    crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "CustomKeyStoreNotFoundException" => crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError::CustomKeyStoreNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::CustomKeyStoreNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_custom_key_store_not_found_exception::de_custom_key_store_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidMarkerException" => crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError::InvalidMarkerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidMarkerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_marker_exception::de_invalid_marker_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "KMSInternalException" => crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError::KmsInternalException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::KmsInternalExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_internal_exception::de_kms_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_custom_key_stores_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresOutput,
    crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_custom_key_stores::de_describe_custom_key_stores(response.body().as_ref(), output).map_err(crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_describe_custom_key_stores(
    value: &[u8],
    mut builder: crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresOutputBuilder,
) -> Result<
    crate::operation::describe_custom_key_stores::builders::DescribeCustomKeyStoresOutputBuilder,
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
                    "CustomKeyStores" => {
                        builder = builder.set_custom_key_stores(
                            crate::protocol_serde::shape_custom_key_stores_list::de_custom_key_stores_list(tokens)?
                        );
                    }
                    "NextMarker" => {
                        builder = builder.set_next_marker(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "Truncated" => {
                        builder = builder.set_truncated(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(
                                tokens.next(),
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