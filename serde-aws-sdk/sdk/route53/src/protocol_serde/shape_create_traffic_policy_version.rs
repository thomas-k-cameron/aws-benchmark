// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_traffic_policy_version_op_input(
    input: &crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("CreateTrafficPolicyVersionRequest")
            .write_ns("https://route53.amazonaws.com/doc/2013-04-01/", None);
        crate::protocol_serde::shape_create_traffic_policy_version_input::ser_create_traffic_policy_version_input_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_traffic_policy_version_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionOutput,
    crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModification" => crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::ConcurrentModification({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConcurrentModificationBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_concurrent_modification::de_concurrent_modification_xml_err(response.body().as_ref(), output).map_err(crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInput" => crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(response.body().as_ref(), output).map_err(crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidTrafficPolicyDocument" => crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::InvalidTrafficPolicyDocument({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidTrafficPolicyDocumentBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_traffic_policy_document::de_invalid_traffic_policy_document_xml_err(response.body().as_ref(), output).map_err(crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchTrafficPolicy" => crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::NoSuchTrafficPolicy({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchTrafficPolicyBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_traffic_policy::de_no_such_traffic_policy_xml_err(response.body().as_ref(), output).map_err(crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyTrafficPolicyVersionsForCurrentPolicy" => crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::TooManyTrafficPolicyVersionsForCurrentPolicy({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyTrafficPolicyVersionsForCurrentPolicyBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_traffic_policy_versions_for_current_policy::de_too_many_traffic_policy_versions_for_current_policy_xml_err(response.body().as_ref(), output).map_err(crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_traffic_policy_version_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionOutput,
    crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_traffic_policy_version::builders::CreateTrafficPolicyVersionOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_traffic_policy_version::de_create_traffic_policy_version(response.body().as_ref(), output).map_err(crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::unhandled)?;
        output = output.set_location(
            crate::protocol_serde::shape_create_traffic_policy_version_output::de_location_header(response.headers())
                                    .map_err(|_|crate::operation::create_traffic_policy_version::CreateTrafficPolicyVersionError::unhandled("Failed to parse Location from header `Location"))?
        );
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_traffic_policy_version(inp: &[u8], mut builder: crate::operation::create_traffic_policy_version::builders::CreateTrafficPolicyVersionOutputBuilder) -> Result<crate::operation::create_traffic_policy_version::builders::CreateTrafficPolicyVersionOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("CreateTrafficPolicyVersionResponse") {
        return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected CreateTrafficPolicyVersionResponse but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            );
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("TrafficPolicy") /* TrafficPolicy com.amazonaws.route53.synthetic#CreateTrafficPolicyVersionOutput$TrafficPolicy */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_traffic_policy::de_traffic_policy(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_traffic_policy(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}