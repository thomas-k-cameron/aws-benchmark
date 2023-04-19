// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_query_logging_config_op_input(
    input: &crate::operation::create_query_logging_config::CreateQueryLoggingConfigInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("CreateQueryLoggingConfigRequest")
            .write_ns("https://route53.amazonaws.com/doc/2013-04-01/", None);
        crate::protocol_serde::shape_create_query_logging_config_input::ser_create_query_logging_config_input_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_query_logging_config_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_query_logging_config::CreateQueryLoggingConfigOutput,
    crate::operation::create_query_logging_config::CreateQueryLoggingConfigError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModification" => crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::ConcurrentModification({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConcurrentModificationBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_concurrent_modification::de_concurrent_modification_xml_err(response.body().as_ref(), output).map_err(crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InsufficientCloudWatchLogsResourcePolicy" => crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::InsufficientCloudWatchLogsResourcePolicy({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InsufficientCloudWatchLogsResourcePolicyBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_insufficient_cloud_watch_logs_resource_policy::de_insufficient_cloud_watch_logs_resource_policy_xml_err(response.body().as_ref(), output).map_err(crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInput" => crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(response.body().as_ref(), output).map_err(crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchCloudWatchLogsLogGroup" => crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::NoSuchCloudWatchLogsLogGroup({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchCloudWatchLogsLogGroupBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_cloud_watch_logs_log_group::de_no_such_cloud_watch_logs_log_group_xml_err(response.body().as_ref(), output).map_err(crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchHostedZone" => crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::NoSuchHostedZone({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchHostedZoneBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_hosted_zone::de_no_such_hosted_zone_xml_err(response.body().as_ref(), output).map_err(crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "QueryLoggingConfigAlreadyExists" => crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::QueryLoggingConfigAlreadyExists({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::QueryLoggingConfigAlreadyExistsBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_query_logging_config_already_exists::de_query_logging_config_already_exists_xml_err(response.body().as_ref(), output).map_err(crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_query_logging_config_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_query_logging_config::CreateQueryLoggingConfigOutput,
    crate::operation::create_query_logging_config::CreateQueryLoggingConfigError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_query_logging_config::builders::CreateQueryLoggingConfigOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_query_logging_config::de_create_query_logging_config(response.body().as_ref(), output).map_err(crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::unhandled)?;
        output = output.set_location(
            crate::protocol_serde::shape_create_query_logging_config_output::de_location_header(response.headers())
                                    .map_err(|_|crate::operation::create_query_logging_config::CreateQueryLoggingConfigError::unhandled("Failed to parse Location from header `Location"))?
        );
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_query_logging_config(
    inp: &[u8],
    mut builder: crate::operation::create_query_logging_config::builders::CreateQueryLoggingConfigOutputBuilder,
) -> Result<
    crate::operation::create_query_logging_config::builders::CreateQueryLoggingConfigOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("CreateQueryLoggingConfigResponse") {
        return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected CreateQueryLoggingConfigResponse but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            );
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("QueryLoggingConfig") /* QueryLoggingConfig com.amazonaws.route53.synthetic#CreateQueryLoggingConfigOutput$QueryLoggingConfig */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_query_logging_config::de_query_logging_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_query_logging_config(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}