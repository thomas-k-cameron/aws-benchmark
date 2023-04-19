// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_change_resource_record_sets_op_input(
    input: &crate::operation::change_resource_record_sets::ChangeResourceRecordSetsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("ChangeResourceRecordSetsRequest")
            .write_ns("https://route53.amazonaws.com/doc/2013-04-01/", None);
        crate::protocol_serde::shape_change_resource_record_sets_input::ser_change_resource_record_sets_input_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_change_resource_record_sets_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::change_resource_record_sets::ChangeResourceRecordSetsOutput,
    crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidChangeBatch" => crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError::InvalidChangeBatch({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidChangeBatchBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_change_batch::de_invalid_change_batch_xml_err(response.body().as_ref(), output).map_err(crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInput" => crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(response.body().as_ref(), output).map_err(crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchHealthCheck" => crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError::NoSuchHealthCheck({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchHealthCheckBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_health_check::de_no_such_health_check_xml_err(response.body().as_ref(), output).map_err(crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchHostedZone" => crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError::NoSuchHostedZone({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchHostedZoneBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_hosted_zone::de_no_such_hosted_zone_xml_err(response.body().as_ref(), output).map_err(crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PriorRequestNotComplete" => crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError::PriorRequestNotComplete({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::PriorRequestNotCompleteBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_prior_request_not_complete::de_prior_request_not_complete_xml_err(response.body().as_ref(), output).map_err(crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_change_resource_record_sets_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::change_resource_record_sets::ChangeResourceRecordSetsOutput,
    crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::change_resource_record_sets::builders::ChangeResourceRecordSetsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_change_resource_record_sets::de_change_resource_record_sets(response.body().as_ref(), output).map_err(crate::operation::change_resource_record_sets::ChangeResourceRecordSetsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_change_resource_record_sets(
    inp: &[u8],
    mut builder: crate::operation::change_resource_record_sets::builders::ChangeResourceRecordSetsOutputBuilder,
) -> Result<
    crate::operation::change_resource_record_sets::builders::ChangeResourceRecordSetsOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("ChangeResourceRecordSetsResponse") {
        return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected ChangeResourceRecordSetsResponse but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            );
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ChangeInfo") /* ChangeInfo com.amazonaws.route53.synthetic#ChangeResourceRecordSetsOutput$ChangeInfo */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_change_info::de_change_info(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_change_info(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}