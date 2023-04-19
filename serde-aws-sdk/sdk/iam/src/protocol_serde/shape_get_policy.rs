// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_policy_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_policy::GetPolicyOutput,
    crate::operation::get_policy::GetPolicyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::get_policy::GetPolicyError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::get_policy::GetPolicyError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::operation::get_policy::GetPolicyError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output =
                    crate::types::error::builders::InvalidInputExceptionBuilder::default();
                let _ = response;
                output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::get_policy::GetPolicyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NoSuchEntity" => crate::operation::get_policy::GetPolicyError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output =
                    crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                let _ = response;
                output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::get_policy::GetPolicyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceFailure" => {
            crate::operation::get_policy::GetPolicyError::ServiceFailureException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::get_policy::GetPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_policy::GetPolicyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_policy_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_policy::GetPolicyOutput,
    crate::operation::get_policy::GetPolicyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_policy::builders::GetPolicyOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_policy::de_get_policy(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::get_policy::GetPolicyError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_policy(
    inp: &[u8],
    mut builder: crate::operation::get_policy::builders::GetPolicyOutputBuilder,
) -> Result<
    crate::operation::get_policy::builders::GetPolicyOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("GetPolicyResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected GetPolicyResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("GetPolicyResult")) {
            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected GetPolicyResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Policy") /* Policy com.amazonaws.iam.synthetic#GetPolicyOutput$Policy */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_policy::de_policy(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_policy(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected GetPolicyResult tag",
        ));
    };
    Ok(builder)
}